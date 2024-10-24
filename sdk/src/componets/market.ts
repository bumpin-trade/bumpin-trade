import {
    AddressLookupTableAccount,
    ConfirmOptions,
    PublicKey,
} from '@solana/web3.js';
import { BulkAccountLoader } from '../account/bulkAccountLoader';
import { Program, Wallet } from '@coral-xyz/anchor';
import { BumpinUtils } from '../utils/utils';
import { BumpinTrade } from '../types/bumpin_trade';
// import {tokenToUsd} from "./utils/cal_utils";
import { Component } from './componet';
import { PollingStateAccountSubscriber } from '../account/pollingStateAccountSubscriber';
import { BumpinSubscriptionFailed } from '../errors';
import { DataAndSlot } from '../account/types';
import { PollingMarketAccountSubscriber } from '../account/pollingMarketAccountSubscriber';
import { TradeTokenComponent } from './tradeToken';
import { Market } from '../beans/beans';
import { BumpinClientConfig } from '../bumpinClientConfig';

export class MarketComponent extends Component {
    program: Program<BumpinTrade>;
    markets: Map<string, PollingMarketAccountSubscriber> = new Map();

    constructor(
        config: BumpinClientConfig,
        defaultConfirmOptions: ConfirmOptions,
        bulkAccountLoader: BulkAccountLoader,
        stateSubscriber: PollingStateAccountSubscriber,
        tradeTokenComponent: TradeTokenComponent,
        program: Program<BumpinTrade>,
        wallet?: Wallet,
        essentialAccounts: AddressLookupTableAccount[] = [],
    ) {
        super(
            config,
            defaultConfirmOptions,
            stateSubscriber,
            program,
            wallet,
            essentialAccounts,
        );
        let state = super.getStateSync();
        this.program = program;
        for (let i = 0; i < state.marketSequence; i++) {
            const [pda, _] = BumpinUtils.getMarketPda(this.program, i);
            let marketAccountSubscriber = new PollingMarketAccountSubscriber(
                program,
                pda,
                bulkAccountLoader,
                tradeTokenComponent,
            );
            this.markets.set(pda.toString(), marketAccountSubscriber);
        }
    }

    public async subscribe() {
        for (let marketAccountSubscriber of this.markets.values()) {
            await marketAccountSubscriber.subscribe();
        }
    }

    public async unsubscribe() {
        for (let marketAccountSubscriber of this.markets.values()) {
            await marketAccountSubscriber.unsubscribe();
        }
    }

    public async getMarkets(sync: boolean = false): Promise<Market[]> {
        let markets = await this.getMarketsWithSlot(sync);
        return markets.map((dataAndSlot) => dataAndSlot.data);
    }

    public async getMarketsWithSlot(
        sync: boolean = false,
    ): Promise<DataAndSlot<Market>[]> {
        let marketsWithSlot: DataAndSlot<Market>[] = [];
        for (let marketAccountSubscriber of this.markets.values()) {
            if (sync) {
                await marketAccountSubscriber.fetch();
            }
            marketsWithSlot.push(marketAccountSubscriber.getAccountAndSlot());
        }
        return marketsWithSlot;
    }

    public async getMarket(
        marketPublicKey: PublicKey,
        sync: boolean = false,
    ): Promise<Market> {
        let marketWithSlot = await this.getMarketWithSlot(
            marketPublicKey,
            sync,
        );
        return marketWithSlot.data;
    }

    public async getMarketWithSlot(
        marketPublicKey: PublicKey,
        sync: boolean = false,
    ): Promise<DataAndSlot<Market>> {
        const marketAccountSubscriber:
            | PollingMarketAccountSubscriber
            | undefined = this.markets.get(marketPublicKey.toString());
        if (marketAccountSubscriber === undefined) {
            throw new BumpinSubscriptionFailed(
                `Market with the key ${marketPublicKey} does not exist`,
            );
        }
        if (sync) {
            await marketAccountSubscriber.fetch();
        }
        return marketAccountSubscriber.getAccountAndSlot();
    }
}
