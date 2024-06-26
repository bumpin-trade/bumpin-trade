import {PublicKey} from '@solana/web3.js';
import {Market} from "../types";
import {BulkAccountLoader} from "../account/bulkAccountLoader";
import {Program} from "@coral-xyz/anchor";
import {BumpinUtils} from "../utils/utils";
import {BumpinTrade} from "../types/bumpin_trade";
// import {tokenToUsd} from "./utils/cal_utils";
import {Component} from "./componet";
import {PollingStateAccountSubscriber} from "../account/pollingStateAccountSubscriber";
import {BumpinSubscriptionFailed} from "../errors";
import {DataAndSlot} from "../account/types";
import {PollingMarketAccountSubscriber} from "../account/pollingMarketAccountSubscriber";

export class MarketComponent extends Component {
    program: Program<BumpinTrade>;
    markets: Map<PublicKey, PollingMarketAccountSubscriber> = new Map();

    constructor(bulkAccountLoader: BulkAccountLoader, stateSubscriber: PollingStateAccountSubscriber, program: Program<BumpinTrade>) {
        super(stateSubscriber, program);
        let state = super.getStateSync();
        this.program = program;
        for (let i = 0; i < state.numberOfMarkets; i++) {
            const [pda, _] = BumpinUtils.getMarketPda(this.program, i);
            let marketAccountSubscriber = new PollingMarketAccountSubscriber(program, pda, bulkAccountLoader);
            this.markets.set(pda, marketAccountSubscriber);
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

    public async getMarket(marketPublicKey: PublicKey, sync: boolean = false): Promise<Market> {
        let marketWithSlot = await this.getMarketWithSlot(marketPublicKey, sync);
        return marketWithSlot.data;
    }

    public async getMarketWithSlot(marketPublicKey: PublicKey, sync: boolean = false): Promise<DataAndSlot<Market>> {
        const marketAccountSubscriber: PollingMarketAccountSubscriber | undefined = this.markets.get(marketPublicKey);
        if (marketAccountSubscriber === undefined) {
            throw new BumpinSubscriptionFailed(`Market with the key ${marketPublicKey} does not exist`);
        }
        if (sync) {
            await marketAccountSubscriber.fetch();
        }
        return marketAccountSubscriber.getAccountAndSlot();
    }


}