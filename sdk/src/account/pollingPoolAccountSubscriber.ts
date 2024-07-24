import { AccountSubscriber, DataAndSlot } from './types';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { PoolAccount } from '../typedef';
import { BulkAccountLoader } from './bulkAccountLoader';
import { BumpinTrade } from '../types/bumpin_trade';
import { Pool } from '../beans/beans';
import { TradeTokenComponent } from '../componets/tradeToken';
import { BumpinTokenUtils } from '../utils/token';

export class PollingPoolAccountSubscriber implements AccountSubscriber<Pool> {
    isSubscribed: boolean;
    program: Program<BumpinTrade>;
    poolPublicKey: PublicKey;

    accountLoader: BulkAccountLoader;
    callbackId?: string;
    errorCallbackId?: string;

    pool?: DataAndSlot<Pool>;

    tradeTokenComponent: TradeTokenComponent;

    public constructor(
        program: Program<BumpinTrade>,
        poolPublicKey: PublicKey,
        accountLoader: BulkAccountLoader,
        tradeTokenComponent: TradeTokenComponent,
    ) {
        this.isSubscribed = false;
        this.program = program;
        this.accountLoader = accountLoader;
        this.poolPublicKey = poolPublicKey;
        this.tradeTokenComponent = tradeTokenComponent;
    }

    async subscribe(userAccount?: Pool): Promise<boolean> {
        if (this.isSubscribed) {
            return true;
        }

        if (userAccount) {
            this.pool = { data: userAccount, slot: 0 };
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
            this.poolPublicKey,
            (buffer, slot: number) => {
                if (!buffer) {
                    return;
                }

                if (this.pool && this.pool.slot > slot) {
                    return;
                }

                const account = this.program.account.pool.coder.accounts.decode(
                    'pool',
                    buffer,
                ) as PoolAccount;
                this.pool = { data: this.convert(account), slot };
            },
        );

        this.errorCallbackId = this.accountLoader.addErrorCallbacks(
            (error) => {},
        );
    }

    async fetchIfUnloaded(): Promise<void> {
        if (this.pool === undefined) {
            await this.fetch();
        }
    }

    async fetch(): Promise<void> {
        try {
            const dataAndContext =
                await this.program.account.pool.fetchAndContext(
                    this.poolPublicKey,
                    this.accountLoader.commitment,
                );
            if (dataAndContext.context.slot > (this.pool?.slot ?? 0)) {
                this.pool = {
                    data: this.convert(
                        dataAndContext.data as any as PoolAccount,
                    ),
                    slot: dataAndContext.context.slot,
                };
            }
        } catch (e: any) {
            console.log(
                `PollingPoolAccountSubscriber.fetch() PoolAccount does not exist: ${e.message}`,
            );
        }
    }

    doesAccountExist(): boolean {
        return this.pool !== undefined;
    }

    async unsubscribe(): Promise<void> {
        if (!this.isSubscribed || !this.callbackId) {
            return;
        }

        this.accountLoader.removeAccount(this.poolPublicKey, this.callbackId);
        this.callbackId = undefined;

        if (this.errorCallbackId)
            this.accountLoader.removeErrorCallbacks(this.errorCallbackId);
        this.errorCallbackId = undefined;

        this.isSubscribed = false;
    }

    assertIsSubscribed(): void {
        if (!this.isSubscribed) {
            throw new Error(
                'You must call `subscribe` before using this function',
            );
        }
    }

    public getAccountAndSlot(): DataAndSlot<Pool> {
        if (!this.doesAccountExist() || !this.pool) {
            throw new Error(
                'You must call `subscribe` or `fetch` before using this function',
            );
        }
        return this.pool;
    }

    public updateData(userAccount: Pool, slot: number): void {
        if (!this.pool || this.pool.slot < slot) {
            this.pool = { data: userAccount, slot };
        }
    }

    private convert(data: PoolAccount): Pool {
        const tradeTokens = this.tradeTokenComponent.getTradeTokensSync();
        const tradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            data.mintKey,
            tradeTokens,
        );
        const stableToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
            data.stableMintKey,
            tradeTokens,
        );
        return new Pool(data, tradeToken.decimals, stableToken.decimals);
    }
}
