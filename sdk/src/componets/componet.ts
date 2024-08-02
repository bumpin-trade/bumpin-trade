import { DataAndSlot } from '../account/types';
import { Program, Wallet } from '@coral-xyz/anchor';
import { BumpinTrade } from '../types/bumpin_trade';
import { PollingStateAccountSubscriber } from '../account/pollingStateAccountSubscriber';
import {
    BumpinAccountNotFound,
    BumpinClientInternalError,
    BumpinSubscriptionFailed,
} from '../errors';
import { State } from '../beans/beans';
import {
    AddressLookupTableAccount,
    ConfirmOptions,
    TransactionInstruction,
    TransactionMessage,
    VersionedTransaction,
} from '@solana/web3.js';
import { BumpinClientConfig } from '../bumpinClientConfig';

export abstract class Component {
    readonly config: BumpinClientConfig;
    readonly defaultConfirmOptions: ConfirmOptions;

    // account lookup table
    essentialAccounts: AddressLookupTableAccount[] = [];

    wallet: Wallet | undefined;
    stateSubscriber: PollingStateAccountSubscriber;

    program: Program<BumpinTrade>;

    constructor(
        config: BumpinClientConfig,
        defaultConfirmOptions: ConfirmOptions,
        stateSubscriber: PollingStateAccountSubscriber,
        program: Program<BumpinTrade>,
        wallet?: Wallet,
        essentialAccounts: AddressLookupTableAccount[] = [],
    ) {
        if (!stateSubscriber.isSubscribed) {
            throw new BumpinClientInternalError('State not subscribed');
        }
        this.config = config;
        this.stateSubscriber = stateSubscriber;
        this.defaultConfirmOptions = defaultConfirmOptions;
        this.wallet = wallet;
        this.essentialAccounts = essentialAccounts;
        this.program = program;
    }

    protected getStateWithSlotSync(): DataAndSlot<State> {
        let stateAccount = this.stateSubscriber.state;
        if (!stateAccount) {
            throw new BumpinAccountNotFound('State');
        }
        return stateAccount;
    }

    protected getStateSync(): State {
        let stateAccount = this.stateSubscriber.getAccountAndSlot();
        return stateAccount.data;
    }

    protected async getState(sync: boolean = false): Promise<State> {
        let stateWithSlot = await this.getStateWithSlot(sync);
        return stateWithSlot.data;
    }

    protected async getStateWithSlot(
        sync: boolean = false,
    ): Promise<DataAndSlot<State>> {
        if (!this.stateSubscriber || !this.stateSubscriber.isSubscribed) {
            throw new BumpinSubscriptionFailed('State');
        }
        if (sync) {
            await this.stateSubscriber.fetch();
        }
        let stateAccount = this.stateSubscriber.state;
        if (!stateAccount) {
            throw new BumpinAccountNotFound('State');
        }
        return stateAccount;
    }

    protected async sendAndConfirm(
        ixs: TransactionInstruction[],
        opts?: ConfirmOptions,
    ) {
        let lastBlockHash =
            await this.program.provider.connection.getLatestBlockhash();
        console.log(
            '   ✅ - Fetched latest blockhash. Last valid height:',
            lastBlockHash.lastValidBlockHeight,
        );
        let blockhash = lastBlockHash.blockhash;
        const messageV0 = new TransactionMessage({
            payerKey: this.wallet!.publicKey,
            recentBlockhash: blockhash,
            instructions: ixs,
        }).compileToV0Message(this.essentialAccounts);
        const transactionV0 = new VersionedTransaction(messageV0);
        const options = opts || this.defaultConfirmOptions;
        const txid = await this.program.provider.sendAndConfirm!(
            transactionV0,
            [],
            options,
        );
        console.log(`🎉 Transaction succesfully confirmed! (${txid})`);
    }
}
