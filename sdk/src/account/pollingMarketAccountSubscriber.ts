import {
    DataAndSlot,
    AccountSubscriber,
} from './types';
import {Program} from '@coral-xyz/anchor';
import {PublicKey} from '@solana/web3.js';
import {Market, } from '../types';
import {BulkAccountLoader} from './bulkAccountLoader';
import {BumpinTrade} from "../types/bumpin_trade";
export class PollingMarketAccountSubscriber implements AccountSubscriber<Market> {
    isSubscribed: boolean;
    program: Program<BumpinTrade>;
    userAccountPublicKey: PublicKey;

    accountLoader: BulkAccountLoader;
    callbackId?: string;
    errorCallbackId?: string;

    market?: DataAndSlot<Market>;

    public constructor(
        program: Program<BumpinTrade>,
        userAccountPublicKey: PublicKey,
        accountLoader: BulkAccountLoader
    ) {
        this.isSubscribed = false;
        this.program = program;
        this.accountLoader = accountLoader;
        this.userAccountPublicKey = userAccountPublicKey;
    }

    async subscribe(userAccount?: Market): Promise<boolean> {
        if (this.isSubscribed) {
            return true;
        }

        if (userAccount) {
            this.market = {data: userAccount, slot: undefined};
        }

        await this.addToAccountLoader();

        await this.fetchIfUnloaded();
        if (this.doesAccountExist()) {
            //  this.eventEmitter.emit('update');
        }

        this.isSubscribed = true;
        return true;
    }

    async addToAccountLoader(): Promise<void> {
        if (this.callbackId) {
            return;
        }

        this.callbackId = await this.accountLoader.addAccount(
            this.userAccountPublicKey,
            (buffer, slot: number) => {
                if (!buffer) {
                    return;
                }

                if (this.market && this.market.slot > slot) {
                    return;
                }

                const account = this.program.account.market.coder.accounts.decode(
                    'market',
                    buffer
                );
                this.market = {data: account, slot};
                /*           this.eventEmitter.emit('userAccountUpdate', account);
                           this.eventEmitter.emit('update');*/
            }
        );

        this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {
            /*
                        this.eventEmitter.emit('error', error);
            */
        });
    }

    async fetchIfUnloaded(): Promise<void> {
        if (this.market === undefined) {
            await this.fetch();
        }
    }

    async fetch(): Promise<void> {
        try {
            const dataAndContext = await this.program.account.market.fetchAndContext(
                this.userAccountPublicKey,
                this.accountLoader.commitment
            );
            if (dataAndContext.context.slot > (this.market?.slot ?? 0)) {
                this.market = {
                    data: dataAndContext.data as any as Market,
                    slot: dataAndContext.context.slot,
                };
            }
        } catch (e) {
            console.log(
                `PollingUserAccountSubscriber.fetch() UserAccount does not exist: ${e.message}`
            );
        }
    }

    doesAccountExist(): boolean {
        return this.market !== undefined;
    }

    async unsubscribe(): Promise<void> {
        if (!this.isSubscribed) {
            return;
        }

        this.accountLoader.removeAccount(
            this.userAccountPublicKey,
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

    public getUserAccountAndSlot(): DataAndSlot<Market> {
        if (!this.doesAccountExist()) {
            throw new Error(
                'You must call `subscribe` or `fetch` before using this function'
            );
        }
        return this.market;
    }

    public updateData(userAccount: Market, slot: number): void {
        if (!this.market || this.market.slot < slot) {
            this.market = {data: userAccount, slot};
            /*
            this.eventEmitter.emit('userAccountUpdate', userAccount);
            this.eventEmitter.emit('update');*/
        }
    }
}
