import {AccountSubscriber, DataAndSlot,} from './types';
import {Program} from '@coral-xyz/anchor';
import {PublicKey} from '@solana/web3.js';
import {Rewards} from '../types';
import {BulkAccountLoader} from './bulkAccountLoader';
import {BumpinTrade} from "../types/bumpin_trade";

export class PollingRewardsAccountSubscriber implements AccountSubscriber<Rewards> {
    isSubscribed: boolean;
    program: Program<BumpinTrade>;
    rewardsPublicKey: PublicKey;

    accountLoader: BulkAccountLoader;
    callbackId?: string;
    errorCallbackId?: string;

    rewards?: DataAndSlot<Rewards>;

    public constructor(
        program: Program<BumpinTrade>,
        poolPublicKey: PublicKey,
        accountLoader: BulkAccountLoader
    ) {
        this.isSubscribed = false;
        this.program = program;
        this.accountLoader = accountLoader;
        this.rewardsPublicKey = poolPublicKey;
    }

    async subscribe(userAccount?: Rewards): Promise<boolean> {
        if (this.isSubscribed) {
            return true;
        }

        if (userAccount) {
            this.rewards = {data: userAccount, slot: 0};
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
            this.rewardsPublicKey,
            (buffer, slot: number) => {
                if (!buffer) {
                    return;
                }

                if (this.rewards && this.rewards.slot > slot) {
                    return;
                }

                const account = this.program.account.rewards.coder.accounts.decode(
                    'rewards',
                    buffer
                );
                this.rewards = {data: account, slot};

            }
        );

        this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {

        });
    }

    async fetchIfUnloaded(): Promise<void> {
        if (this.rewards === undefined) {
            await this.fetch();
        }
    }

    async fetch(): Promise<void> {
        try {
            const dataAndContext = await this.program.account.pool.fetchAndContext(
                this.rewardsPublicKey,
                this.accountLoader.commitment
            );
            if (dataAndContext.context.slot > (this.rewards?.slot ?? 0)) {
                this.rewards = {
                    data: dataAndContext.data as any as Rewards,
                    slot: dataAndContext.context.slot,
                };
            }
        } catch (e: any) {
            console.log(
                `PollingRewardAccountSubscriber.fetch() RewardsAccount does not exist: ${e.message}`
            );
        }
    }

    doesAccountExist(): boolean {
        return this.rewards !== undefined;
    }

    async unsubscribe(): Promise<void> {
        if (!this.isSubscribed || !this.callbackId) {
            return;
        }

        this.accountLoader.removeAccount(
            this.rewardsPublicKey,
            this.callbackId
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
                'You must call `subscribe` before using this function'
            );
        }
    }

    public getAccountAndSlot(): DataAndSlot<Rewards> {
        if (!this.doesAccountExist() || !this.rewards) {
            throw new Error(
                'You must call `subscribe` or `fetch` before using this function'
            );
        }
        return this.rewards;
    }

    public updateData(userAccount: Rewards, slot: number): void {
        if (!this.rewards || this.rewards.slot < slot) {
            this.rewards = {data: userAccount, slot};
            /*
            this.eventEmitter.emit('userAccountUpdate', userAccount);
            this.eventEmitter.emit('update');*/
        }
    }
}
