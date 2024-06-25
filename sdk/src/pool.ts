import {OracleClient} from "./oracles/types";
import {PublicKey} from '@solana/web3.js';
import {AccountSubscriber, DataAndSlot} from "./account/types";
import {Pool, State, TradeToken, UserAccount} from "./types";
import {BumpinClientConfig} from "./bumpinClientConfig";
import {PollingPoolAccountSubscriber} from "./account/pollingPoolAccountSubscriber";
import {BulkAccountLoader} from "./account/bulkAccountLoader";
import {AnchorProvider, BN, Program, Wallet} from "@coral-xyz/anchor";
import {BumpinUtils} from "./utils/utils";
import {BumpinTrade} from "./types/bumpin_trade";
import {tokenToUsd} from "./utils/cal_utils";

export class PoolComponent {
    oracleClient: OracleClient
    state: State
    program: Program<BumpinTrade>;
    pools: Map<PublicKey, PollingPoolAccountSubscriber>;

    constructor(oracleClient: OracleClient, bulkAccountLoader: BulkAccountLoader, program: Program<BumpinTrade>) {
        this.oracleClient = oracleClient;
        this.program = program;
        for (let i = 0; i < this.state!.numberOfPools; i++) {
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

    public getPool(poolKey: PublicKey): Pool {
        const poolAccountSubscriber: PollingPoolAccountSubscriber | undefined = this.pools.get(poolKey);
        if (poolAccountSubscriber === undefined) {
            throw new Error(`Pool with the key ${poolKey} does not exist`);
        }
        return poolAccountSubscriber.getUserAccountAndSlot().data;
    }


    public getPoolAvailableLiquidity(poolKey: PublicKey): BN {
        let pool = this.getPool(poolKey);
    }

    public async getPoolUsd(poolKey: PublicKey, tradeTokenMap: Map<PublicKey, TradeToken>): Promise<BN> {
        let pool = this.getPool(poolKey);
        let tradeToken = tradeTokenMap.get(pool.poolMint);
        let oraclePriceData = await this.oracleClient.getOraclePriceData(pool.poolMint);
        let poolUsd = tokenToUsd(pool.poolBalance.amount.add(pool.poolBalance.unSettleAmount),
            tradeToken.decimals, oraclePriceData.price);
    }
}