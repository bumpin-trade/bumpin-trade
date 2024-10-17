import { AddressLookupTableAccount, ConfirmOptions, PublicKey } from '@solana/web3.js';
import { BulkAccountLoader } from '../account/bulkAccountLoader';
import { Program, Wallet } from '@coral-xyz/anchor';
import { BumpinTrade } from '../types/bumpin_trade';
import { Component } from './componet';
import { PollingStateAccountSubscriber } from '../account/pollingStateAccountSubscriber';
import { BumpinClientConfig } from '../bumpinClientConfig';
import { StashedPythV2Client } from '../oracles/stashedPythV2Client';
import { PriceUpdateV2 } from '../oracles/pythv2_def';
import { BumpinSubscriptionFailed } from '../errors';

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

    public getPrices(feedId: PublicKey): PriceUpdateV2 {
        let client = this.oracles.get(feedId);
        if (client === undefined) {
            throw new BumpinSubscriptionFailed(
                `TradeToken with the feedId: ${feedId} does not exist`,
            );
        }

        return client.getLastOraclePriceData(1)[0];
    }

    public getPricesWithCount(
        feedId: PublicKey,
        count: number = 1,
    ): PriceUpdateV2[] {
        let client = this.oracles.get(feedId);
        if (client === undefined) {
            throw new BumpinSubscriptionFailed(
                `TradeToken with the feedId: ${feedId} does not exist`,
            );
        }

        return client.getLastOraclePriceData(count);
    }

    // public getTradeTokenPricesByOracleKey(
    //     oracleKey: PublicKey,
    //     count: number,
    // ): PriceData[] {
    //     let stashedPythClient = this.tradeTokenOraclePyths.get(
    //         oracleKey.toString(),
    //     );
    //     if (stashedPythClient === undefined) {
    //         throw new BumpinSubscriptionFailed(
    //             `TradeToken with the oracle key ${oracleKey} does not exist`,
    //         );
    //     }
    //     return stashedPythClient.getLastOraclePriceData(count);
    // }
    //
    // public async getTradeTokenPricesByMintKey(
    //     mintKey: PublicKey,
    //     sync: boolean = false,
    // ): Promise<PriceData> {
    //     let tradeToken = await this.getTradeTokenByMintKey(mintKey, sync);
    //     return this.getTradeTokenPrices(
    //         BumpinUtils.getTradeTokenPda(this.program, tradeToken.index)[0],
    //     );
    // }
    //
    // public async getTradeTokenPricesByMintKeyWithCount(
    //     mintKey: PublicKey,
    //     count: number = 1,
    //     sync: boolean = false,
    // ): Promise<PriceData[]> {
    //     let tradeToken = await this.getTradeTokenByMintKey(mintKey, sync);
    //     return this.getTradeTokenPriceWithCount(
    //         BumpinUtils.getTradeTokenPda(this.program, tradeToken.index)[0],
    //         count,
    //     );
    // }
}
