import {PublicKey} from '@solana/web3.js';
import {Rewards} from "../types";
import {BulkAccountLoader} from "../account/bulkAccountLoader";
import {Program} from "@coral-xyz/anchor";
import {BumpinUtils} from "../utils/utils";
import {BumpinTrade} from "../types/bumpin_trade";
// import {tokenToUsd} from "./utils/cal_utils";
import {Component} from "./componet";
import {PollingStateAccountSubscriber} from "../account/pollingStateAccountSubscriber";
import {BumpinSubscriptionFailed} from "../errors";
import {DataAndSlot} from "../account/types";
import {PollingRewardsAccountSubscriber} from "../account/pollingRewardsAccountSubscriber";

export class RewardsComponent extends Component {
    program: Program<BumpinTrade>;
    rewards: Map<string, PollingRewardsAccountSubscriber> = new Map();

    constructor(bulkAccountLoader: BulkAccountLoader, stateSubscriber: PollingStateAccountSubscriber, program: Program<BumpinTrade>) {
        super(stateSubscriber, program);
        let state = super.getStateSync();
        this.program = program;
        for (let i = 0; i < state.poolSequence; i++) {
            const [pda, _] = BumpinUtils.getRewardsPda(this.program, i);
            let rewardsAccountSubscriber = new PollingRewardsAccountSubscriber(program, pda, bulkAccountLoader);
            this.rewards.set(pda.toString(), rewardsAccountSubscriber);
        }
    }

    public async subscribe() {
        for (let rewardsAccountSubscriber of this.rewards.values()) {
            await rewardsAccountSubscriber.subscribe();
        }
    }

    public async unsubscribe() {
        for (let rewardsAccountSubscriber of this.rewards.values()) {
            await rewardsAccountSubscriber.unsubscribe();
        }
    }

    public async getRewards(sync: boolean = false): Promise<Rewards[]> {
        let rewards = await this.getRewardsWithSlot(sync);
        return rewards.map((dataAndSlot) => dataAndSlot.data);
    }

    public async getReward(rewardsKey: PublicKey, sync: boolean = false): Promise<Rewards> {
        let rewardsWithSlot = await this.getRewardWithSlot(rewardsKey, sync);
        return rewardsWithSlot.data;
    }

    public async getRewardsWithSlot(sync: boolean = false): Promise<DataAndSlot<Rewards>[]> {
        let rewardsWithSlot: DataAndSlot<Rewards>[] = [];
        for (let rewardsAccountSubscriber of this.rewards.values()) {
            if (sync) {
                await rewardsAccountSubscriber.fetch();
            }
            rewardsWithSlot.push(rewardsAccountSubscriber.getAccountAndSlot());
        }
        return rewardsWithSlot;
    }

    public async getRewardWithSlot(rewardKey: PublicKey, sync: boolean = false): Promise<DataAndSlot<Rewards>> {
        const rewardsAccountSubscriber: PollingRewardsAccountSubscriber | undefined = this.rewards.get(rewardKey.toString());
        if (rewardsAccountSubscriber === undefined) {
            throw new BumpinSubscriptionFailed(`Reward with the key ${rewardKey} does not exist`);
        }
        if (sync) {
            await rewardsAccountSubscriber.fetch();
        }
        return rewardsAccountSubscriber.getAccountAndSlot();
    }
}