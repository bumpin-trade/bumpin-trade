import { AccountSubscriber, DataAndSlot } from './types';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { MarketAccount } from '../typedef';
import { BulkAccountLoader } from './bulkAccountLoader';
import { BumpinTrade } from '../types/bumpin_trade';
import { Market } from '../beans/beans';
import { TradeTokenComponent } from '../componets/tradeToken';
import { BumpinTokenUtils } from '../utils/token';

export class PollingMarketAccountSubscriber
    implements AccountSubscriber<Market>
{
    isSubscribed: boolean;
    program: Program<BumpinTrade>;
    userAccountPublicKey: PublicKey;

    accountLoader: BulkAccountLoader;
    callbackId?: string;
    errorCallbackId?: string;

    market?: DataAndSlot<Market>;
    tradeTokenComponent: TradeTokenComponent;

    public constructor(
        program: Program<BumpinTrade>,
        userAccountPublicKey: PublicKey,
        accountLoader: BulkAccountLoader,
        tradeTokenComponent: TradeTokenComponent,
    ) {
        this.isSubscribed = false;
        this.program = program;
        this.accountLoader = accountLoader;
        this.userAccountPublicKey = userAccountPublicKey;
        this.tradeTokenComponent = tradeTokenComponent;
    }

    async subscribe(userAccount?: Market): Promise<boolean> {
        if (this.isSubscribed) {
            return true;
        }

        if (userAccount) {
            this.market = { data: userAccount, slot: 0 };
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

                const account =
                    this.program.account.market.coder.accounts.decode(
                        'market',
                        buffer,
                    );
                this.market = { data: this.convert(account), slot };
            },
        );

        this.errorCallbackId = this.accountLoader.addErrorCallbacks(
            (error) => {},
        );
    }

    async fetchIfUnloaded(): Promise<void> {
        if (this.market === undefined) {
            await this.fetch();
        }
    }

    async fetch(): Promise<void> {
        try {
            const dataAndContext =
                await this.program.account.market.fetchAndContext(
                    this.userAccountPublicKey,
                    this.accountLoader.commitment,
                );
            if (dataAndContext.context.slot > (this.market?.slot ?? 0)) {
                this.market = {
                    data: this.convert(
                        dataAndContext.data as any as MarketAccount,
                    ),
                    slot: dataAndContext.context.slot,
                };
            }
        } catch (e: any) {
            console.log(
                `PollingMarketAccountSubscriber.fetch() UserAccount does not exist: ${e.message}`,
            );
        }
    }

    doesAccountExist(): boolean {
        return this.market !== undefined;
    }

    async unsubscribe(): Promise<void> {
        if (!this.isSubscribed || !this.callbackId) {
            return;
        }

        this.accountLoader.removeAccount(
            this.userAccountPublicKey,
            this.callbackId,
        );
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

    public getAccountAndSlot(): DataAndSlot<Market> {
        if (!this.doesAccountExist() || !this.market) {
            throw new Error(
                'You must call `subscribe` or `fetch` before using this function',
            );
        }
        return this.market;
    }

    public updateData(userAccount: Market, slot: number): void {
        if (!this.market || this.market.slot < slot) {
            this.market = { data: userAccount, slot };
        }
    }

    private convert(market: MarketAccount): Market {
        const tradeTokens = this.tradeTokenComponent.getTradeTokensSync();
        return new Market(
            market,
            BumpinTokenUtils.getTradeTokenByMintPublicKey(
                market.poolMintKey,
                tradeTokens,
            ).decimals,
            BumpinTokenUtils.getTradeTokenByMintPublicKey(
                market.stablePoolMintKey,
                tradeTokens,
            ).decimals,
        );
    }
}
