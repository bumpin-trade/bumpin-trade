import { AccountSubscriber, DataAndSlot } from './types';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { StateAccount } from '../typedef';
import { BulkAccountLoader } from './bulkAccountLoader';
import { BumpinTrade } from '../types/bumpin_trade';
import { State } from '../beans/beans';

export class PollingStateAccountSubscriber implements AccountSubscriber<State> {
    isSubscribed: boolean;
    program: Program<BumpinTrade>;
    userAccountPublicKey: PublicKey;

    accountLoader: BulkAccountLoader;
    callbackId?: string;
    errorCallbackId?: string;

    state?: DataAndSlot<State>;

    public constructor(
        program: Program<BumpinTrade>,
        publicKey: PublicKey,
        accountLoader: BulkAccountLoader,
    ) {
        this.isSubscribed = false;
        this.program = program;
        this.accountLoader = accountLoader;
        this.userAccountPublicKey = publicKey;
    }

    async subscribe(userAccount?: State): Promise<boolean> {
        if (this.isSubscribed) {
            return true;
        }

        if (userAccount) {
            this.state = { data: userAccount, slot: 0 };
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

                if (this.state && this.state.slot > slot) {
                    return;
                }

                const account =
                    this.program.account.market.coder.accounts.decode(
                        'state',
                        buffer,
                    );
                this.state = { data: this.convert(account), slot };
            },
        );

        this.errorCallbackId = this.accountLoader.addErrorCallbacks(
            (error) => {},
        );
    }

    async fetchIfUnloaded(): Promise<void> {
        if (this.state === undefined) {
            await this.fetch();
        }
    }

    async fetch(): Promise<void> {
        try {
            const dataAndContext =
                await this.program.account.state.fetchAndContext(
                    this.userAccountPublicKey,
                    this.accountLoader.commitment,
                );
            if (dataAndContext.context.slot > (this.state?.slot ?? 0)) {
                this.state = {
                    data: this.convert(
                        dataAndContext.data as any as StateAccount,
                    ),
                    slot: dataAndContext.context.slot,
                };
            }
        } catch (e: any) {
            console.log(
                `PollingStateAccountSubscriber.fetch() UserAccount does not exist: ${e.message}`,
            );
        }
    }

    doesAccountExist(): boolean {
        return this.state !== undefined;
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

    public getAccountAndSlot(): DataAndSlot<State> {
        if (!this.doesAccountExist() || !this.state) {
            throw new Error(
                'You must call `subscribe` or `fetch` before using this function',
            );
        }
        return this.state;
    }

    public updateData(userAccount: State, slot: number): void {
        if (!this.state || this.state.slot < slot) {
            this.state = { data: userAccount, slot };
        }
    }

    private convert(data: StateAccount): State {
        return new State(data);
    }
}
