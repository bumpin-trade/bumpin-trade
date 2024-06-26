import {OracleClient} from "../oracles/types";
import {PublicKey} from '@solana/web3.js';
import {TradeToken} from "../types";
import {BulkAccountLoader} from "../account/bulkAccountLoader";
import {Program} from "@coral-xyz/anchor";
import {BumpinUtils} from "../utils/utils";
import {BumpinTrade} from "../types/bumpin_trade";
// import {tokenToUsd} from "./utils/cal_utils";
import {Component} from "./componet";
import {PollingStateAccountSubscriber} from "../account/pollingStateAccountSubscriber";
import {BumpinSubscriptionFailed} from "../errors";
import {DataAndSlot} from "../account/types";
import {PollingTradeTokenAccountSubscriber} from "../account/pollingTradeTokenAccountSubscriber";

export class TradeTokenComponent extends Component {
    program: Program<BumpinTrade>;
    tradeTokens: Map<PublicKey, PollingTradeTokenAccountSubscriber>;

    constructor(bulkAccountLoader: BulkAccountLoader, stateSubscriber: PollingStateAccountSubscriber, program: Program<BumpinTrade>) {
        super(stateSubscriber, program);
        let state = super.getStateSync();
        this.program = program;
        for (let i = 0; i < state.numberOfTradeTokens; i++) {
            const [pda, _] = BumpinUtils.getTradeTokenPda(this.program, i);
            let tradeTokenAccountSubscriber = new PollingTradeTokenAccountSubscriber(program, pda, bulkAccountLoader);
            this.tradeTokens.set(pda, tradeTokenAccountSubscriber);
        }
    }

    public async subscribe() {
        for (let tradeTokenAccountSubscriber of this.tradeTokens.values()) {
            await tradeTokenAccountSubscriber.subscribe();
        }
    }

    public async unsubscribe() {
        for (let tradeTokenAccountSubscriber of this.tradeTokens.values()) {
            await tradeTokenAccountSubscriber.unsubscribe();
        }
    }

    public async getTradeToken(tradeTokenKey: PublicKey, sync: boolean = false): Promise<TradeToken> {
        let poolWithSlot = await this.getTradeTokenWithSlot(tradeTokenKey, sync);
        return poolWithSlot.data;
    }

    public async getTradeTokenWithSlot(tradeTokenKey: PublicKey, sync: boolean = false): Promise<DataAndSlot<TradeToken>> {
        const tradeTokenAccountSubscriber: PollingTradeTokenAccountSubscriber | undefined = this.tradeTokens.get(tradeTokenKey);
        if (tradeTokenAccountSubscriber === undefined) {
            throw new BumpinSubscriptionFailed(`TradeToken with the key ${tradeTokenKey} does not exist`);
        }
        if (sync) {
            await tradeTokenAccountSubscriber.fetch();
        }
        return tradeTokenAccountSubscriber.getAccountAndSlot();
    }

}