import {
    DataAndSlot,
    AccountSubscriber,
} from './types';
import {Program} from '@coral-xyz/anchor';
import {PublicKey} from '@solana/web3.js';
import {Pool, UserAccount} from '../types';
import {BulkAccountLoader} from './bulkAccountLoader';
import {BumpinTrade} from "../types/bumpin_trade";

export class PollingPoolAccountSubscriber implements AccountSubscriber<Pool> {
    isSubscribed: boolean;
    program: Program<BumpinTrade>;
    poolPublicKey: PublicKey;

    accountLoader: BulkAccountLoader;
    callbackId?: string;
    errorCallbackId?: string;

    pool?: DataAndSlot<Pool>;

    public constructor(
        program: Program<BumpinTrade>,
        poolPublicKey: PublicKey,
        accountLoader: BulkAccountLoader
    ) {
        this.isSubscribed = false;
        this.program = program;
        this.accountLoader = accountLoader;
        this.poolPublicKey = poolPublicKey;
    }

    async subscribe(userAccount?: Pool): Promise<boolean> {
        if (this.isSubscribed) {
            return true;
        }

        if (userAccount) {
            this.pool = {data: userAccount, slot: undefined};
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
            this.poolPublicKey,
            (buffer, slot: number) => {
                if (!buffer) {
                    return;
                }

                if (this.pool && this.pool.slot > slot) {
                    return;
                }

                const account = this.program.account.pool.coder.accounts.decode(
                    'Pool',
                    buffer
                );
                this.pool = {data: account, slot};
                /*           this.eventEmitter.emit('userAccountUpdate', account);
                           this.eventEmitter.emit('update');*/
            }
        );

        this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {
            /*
                        this.eventEmitter.emit('error', error);
            */
        });
    }

    async fetchIfUnloaded(): Promise<void> {
        if (this.pool === undefined) {
            await this.fetch();
        }
    }

    async fetch(): Promise<void> {
        try {
            const dataAndContext = await this.program.account.pool.fetchAndContext(
                this.poolPublicKey,
                this.accountLoader.commitment
            );
            if (dataAndContext.context.slot > (this.pool?.slot ?? 0)) {
                this.pool = {
                    data: dataAndContext.data as Pool,
                    slot: dataAndContext.context.slot,
                };
            }
        } catch (e) {
            console.log(
                `PollingUserAccountSubscriber.fetch() UserAccount does not exist: ${e.message}`
            );
        }
    }

    doesAccountExist(): boolean {
        return this.pool !== undefined;
    }

    async unsubscribe(): Promise<void> {
        if (!this.isSubscribed) {
            return;
        }

        this.accountLoader.removeAccount(
            this.poolPublicKey,
            this.callbackId
        );
        this.callbackId = undefined;

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

    public getUserAccountAndSlot(): DataAndSlot<Pool> {
        if (!this.doesAccountExist()) {
            throw new Error(
                'You must call `subscribe` or `fetch` before using this function'
            );
        }
        return this.pool;
    }

    public updateData(userAccount: Pool, slot: number): void {
        if (!this.pool || this.pool.slot < slot) {
            this.pool = {data: userAccount, slot};
            /*
            this.eventEmitter.emit('userAccountUpdate', userAccount);
            this.eventEmitter.emit('update');*/
        }
    }
}
