import {AccountSubscriber, DataAndSlot,} from './types';
import {Program} from '@coral-xyz/anchor';
import {PublicKey} from '@solana/web3.js';
import {TradeToken} from '../types';
import {BulkAccountLoader} from './bulkAccountLoader';
import {BumpinTrade} from "../types/bumpin_trade";

export class PollingTradeTokenAccountSubscriber implements AccountSubscriber<TradeToken> {
    isSubscribed: boolean;
    program: Program<BumpinTrade>;
    tradeTokenPublicKey: PublicKey;

    accountLoader: BulkAccountLoader;
    callbackId?: string;
    errorCallbackId?: string;

    tradeToken?: DataAndSlot<TradeToken>;

    public constructor(
        program: Program<BumpinTrade>,
        tradeTokenPublicKey: PublicKey,
        accountLoader: BulkAccountLoader
    ) {
        this.isSubscribed = false;
        this.program = program;
        this.accountLoader = accountLoader;
        this.tradeTokenPublicKey = tradeTokenPublicKey;
    }

    async subscribe(userAccount?: TradeToken): Promise<boolean> {
        if (this.isSubscribed) {
            return true;
        }

        if (userAccount) {
            this.tradeToken = {data: userAccount, slot: undefined};
        }

        await this.addToAccountLoader();

        await this.fetchIfUnloaded();
        if (this.doesAccountExist()) {
        }

        this.isSubscribed = true;
        return true;
    }

    async addToAccountLoader(): Promise<void> {
        if (this.callbackId) {
            return;
        }

        this.callbackId = await this.accountLoader.addAccount(
            this.tradeTokenPublicKey,
            (buffer, slot: number) => {
                if (!buffer) {
                    return;
                }

                if (this.tradeToken && this.tradeToken.slot > slot) {
                    return;
                }

                const account = this.program.account.pool.coder.accounts.decode(
                    'tradeToken',
                    buffer
                );
                this.tradeToken = {data: account, slot};

            }
        );

        this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {
        });
    }

    async fetchIfUnloaded(): Promise<void> {
        if (this.tradeToken === undefined) {
            await this.fetch();
        }
    }

    async fetch(): Promise<void> {
        try {
            const dataAndContext = await this.program.account.tradeToken.fetchAndContext(
                this.tradeTokenPublicKey,
                this.accountLoader.commitment
            );
            if (dataAndContext.context.slot > (this.tradeToken?.slot ?? 0)) {
                this.tradeToken = {
                    data: dataAndContext.data as any as TradeToken,
                    slot: dataAndContext.context.slot,
                };
            }
        } catch (e) {
            console.log(
                `PollingUserAccountSubscriber.fetch() TradeTokenAccount does not exist: ${e.message}`
            );
        }
    }

    doesAccountExist(): boolean {
        return this.tradeToken !== undefined;
    }

    async unsubscribe(): Promise<void> {
        if (!this.isSubscribed) {
            return;
        }

        this.accountLoader.removeAccount(
            this.tradeTokenPublicKey,
            this.callbackId
        );
        this.callbackId = undefined;

        this.accountLoader.removeErrorCallbacks(this.errorCallbackId);
        this.errorCallbackId = undefined;

        this.isSubscribed = false;
    }

    assertIsSubscribed(): void {
        if (!this.isSubscribed) {
            throw new Error(
                'You must call `subscribe` before using this function'
            );
        }
    }

    public getAccountAndSlot(): DataAndSlot<TradeToken> {
        if (!this.doesAccountExist()) {
            throw new Error(
                'You must call `subscribe` or `fetch` before using this function'
            );
        }
        return this.tradeToken;
    }

    public updateData(userAccount: TradeToken, slot: number): void {
        if (!this.tradeToken || this.tradeToken.slot < slot) {
            this.tradeToken = {data: userAccount, slot};

        }
    }
}