import { AddressLookupTableAccount, ConfirmOptions, PublicKey } from '@solana/web3.js';
import { BulkAccountLoader } from '../account/bulkAccountLoader';
import { Program, Wallet } from '@coral-xyz/anchor';
import { BumpinTrade } from '../types/bumpin_trade';
import { Component } from './componet';
import { PollingStateAccountSubscriber } from '../account/pollingStateAccountSubscriber';
import { BumpinClientConfig } from '../bumpinClientConfig';
import { StashedPythV2Client } from '../oracles/stashedPythV2Client';
import { PriceUpdateV2 } from '../oracles/pythv2_def';

export class OracleComponent extends Component {
    bulkAccountLoader: BulkAccountLoader;
    program: Program<BumpinTrade>;
    essentialAccounts: AddressLookupTableAccount[] = [];

    oracles: Map<PublicKey, StashedPythV2Client> = new Map();

    constructor(
        config: BumpinClientConfig,
        defaultConfirmOptions: ConfirmOptions,
        bulkAccountLoader: BulkAccountLoader,
        stateSubscriber: PollingStateAccountSubscriber,
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
        this.bulkAccountLoader = bulkAccountLoader;
        this.program = program;
        this.essentialAccounts = essentialAccounts;
    }

    public async subscribe() {
        const subscriptionPromises = this.essentialAccounts.flatMap((account) => {
            console.log('Subscribing to account, len: ', account.state.addresses.length);
            return account.state.addresses.map(async (key) => {
                return await this.subscribe0(key);
            });
        });

        let promises: Awaited<[StashedPythV2Client, PublicKey] | undefined>[] = await Promise.all(subscriptionPromises);
        for (const p of promises) {
            if (p) {
                if (p) {
                    this.oracles.set(p[1], p[0]);
                    console.log('Subscribed to oracle: ', p[1].toString());
                }
            }
        }

        console.log('All accounts subscribed.');
    }

    private async subscribe0(account: PublicKey): Promise<[StashedPythV2Client, PublicKey] | undefined> {
        const client = new StashedPythV2Client(account, 2, this.program.provider.connection);
        try {
            const priceUpdateV2: PriceUpdateV2 = await client.getPriceData();
            if (priceUpdateV2) {
                return [client, new PublicKey(priceUpdateV2.priceMessage.feedId)];
            }
        } catch (e) {
            return;
        }
    }

    public async unsubscribe() {

    }
}
