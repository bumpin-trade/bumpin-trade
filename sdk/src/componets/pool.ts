import {OracleClient} from "../oracles/types";
import {PublicKey} from '@solana/web3.js';
import {Pool} from "../types";
import {PollingPoolAccountSubscriber} from "../account/pollingPoolAccountSubscriber";
import {BulkAccountLoader} from "../account/bulkAccountLoader";
import {Program} from "@coral-xyz/anchor";
import {BumpinUtils} from "../utils/utils";
import {BumpinTrade} from "../types/bumpin_trade";
// import {tokenToUsd} from "./utils/cal_utils";
import {Component} from "./componet";
import {PollingStateAccountSubscriber} from "../account/pollingStateAccountSubscriber";
import {BumpinSubscriptionFailed} from "../errors";
import {DataAndSlot} from "../account/types";

export class PoolComponent extends Component {
    oracleClient: OracleClient
    program: Program<BumpinTrade>;
    pools: Map<PublicKey, PollingPoolAccountSubscriber> = new Map();

    constructor(oracleClient: OracleClient, bulkAccountLoader: BulkAccountLoader, stateSubscriber: PollingStateAccountSubscriber, program: Program<BumpinTrade>) {
        super(stateSubscriber, program);
        let state = super.getStateSync();
        this.oracleClient = oracleClient;
        this.program = program;
        for (let i = 0; i < state.numberOfPools; i++) {
            const [pda, _] = BumpinUtils.getPoolPda(this.program, i);
            let poolAccountSubscriber = new PollingPoolAccountSubscriber(program, pda, bulkAccountLoader);
            this.pools.set(pda, poolAccountSubscriber);
        }
    }

    public async subscribe() {
        for (let poolAccountSubscriber of this.pools.values()) {
            await poolAccountSubscriber.subscribe();
        }
    }

    public async unsubscribe() {
        for (let poolAccountSubscriber of this.pools.values()) {
            await poolAccountSubscriber.unsubscribe();
        }
    }

    public async getPool(poolKey: PublicKey, sync: boolean = false): Promise<Pool> {
        let poolWithSlot = await this.getPoolWithSlot(poolKey, sync);
        return poolWithSlot.data;
    }

    public async getPoolWithSlot(poolKey: PublicKey, sync: boolean = false): Promise<DataAndSlot<Pool>> {
        const poolAccountSubscriber: PollingPoolAccountSubscriber | undefined = this.pools.get(poolKey);
        if (poolAccountSubscriber === undefined) {
            throw new BumpinSubscriptionFailed(`Pool with the key ${poolKey} does not exist`);
        }
        if (sync) {
            await poolAccountSubscriber.fetch();
        }
        return poolAccountSubscriber.getAccountAndSlot();
    }


    // public getPoolAvailableLiquidity(poolKey: PublicKey): BN {
    //     let pool = this.getPool(poolKey);
    // }

    // public async getPoolUsd(poolKey: PublicKey, tradeTokenMap: Map<PublicKey, TradeToken>): Promise<BN> {
    //     let pool = this.getPool(poolKey);
    //     let tradeToken = tradeTokenMap.get(pool.poolMint);
    //     let oraclePriceData = await this.oracleClient.getOraclePriceData(pool.poolMint);
    //     let poolUsd = tokenToUsd(pool.poolBalance.amount.add(pool.poolBalance.unSettleAmount),
    //         tradeToken.decimals, oraclePriceData.price);
    // }
}