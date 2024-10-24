import {
    AddressLookupTableAccount,
    ConfirmOptions,
    PublicKey,
} from '@solana/web3.js';
import { PollingPoolAccountSubscriber } from '../account/pollingPoolAccountSubscriber';
import { BulkAccountLoader } from '../account/bulkAccountLoader';
import { Program, Wallet } from '@coral-xyz/anchor';
import { BumpinUtils } from '../utils/utils';
import { BumpinTrade } from '../types/bumpin_trade';
// import {tokenToUsd} from "./utils/cal_utils";
import { Component } from './componet';
import { PollingStateAccountSubscriber } from '../account/pollingStateAccountSubscriber';
import { BumpinSubscriptionFailed } from '../errors';
import { DataAndSlot } from '../account/types';
import { TradeTokenComponent } from './tradeToken';
import { Pool } from '../beans/beans';
import { BumpinClientConfig } from '../bumpinClientConfig';

export class PoolComponent extends Component {
    program: Program<BumpinTrade>;
    pools: Map<string, PollingPoolAccountSubscriber> = new Map();

    tradeTokenComponent: TradeTokenComponent;

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
        this.tradeTokenComponent = tradeTokenComponent;
        this.program = program;
        for (let i = 0; i < state.poolSequence; i++) {
            const [pda, _] = BumpinUtils.getPoolPda(this.program, i);
            let poolAccountSubscriber = new PollingPoolAccountSubscriber(
                program,
                pda,
                bulkAccountLoader,
                tradeTokenComponent,
            );
            this.pools.set(pda.toString(), poolAccountSubscriber);
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

    public async getPools(sync: boolean = false): Promise<Pool[]> {
        let pools = await this.getPoolsWithSlot(sync);
        return pools.map((dataAndSlot) => dataAndSlot.data);
    }

    public getPoolsSync(): Pool[] {
        let pools = this.getPoolsWithSlotSync();
        return pools.map((dataAndSlot) => dataAndSlot.data);
    }

    public async getPool(
        poolKey: PublicKey,
        sync: boolean = false,
    ): Promise<Pool> {
        let poolWithSlot = await this.getPoolWithSlot(poolKey, sync);
        return poolWithSlot.data;
    }

    public async getPoolsWithSlot(
        sync: boolean = false,
    ): Promise<DataAndSlot<Pool>[]> {
        let poolsWithSlot: DataAndSlot<Pool>[] = [];
        for (let poolAccountSubscriber of this.pools.values()) {
            if (sync) {
                await poolAccountSubscriber.fetch();
            }
            poolsWithSlot.push(poolAccountSubscriber.getAccountAndSlot());
        }
        return poolsWithSlot;
    }

    public getPoolsWithSlotSync(): DataAndSlot<Pool>[] {
        let poolsWithSlot: DataAndSlot<Pool>[] = [];
        for (let poolAccountSubscriber of this.pools.values()) {
            poolsWithSlot.push(poolAccountSubscriber.getAccountAndSlot());
        }
        return poolsWithSlot;
    }

    public async getPoolWithSlot(
        poolKey: PublicKey,
        sync: boolean = false,
    ): Promise<DataAndSlot<Pool>> {
        const poolAccountSubscriber: PollingPoolAccountSubscriber | undefined =
            this.pools.get(poolKey.toString());
        if (poolAccountSubscriber === undefined) {
            throw new BumpinSubscriptionFailed(
                `Pool with the key ${poolKey} does not exist`,
            );
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
