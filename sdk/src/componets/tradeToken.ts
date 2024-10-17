import {
    AddressLookupTableAccount,
    ConfirmOptions,
    PublicKey,
} from '@solana/web3.js';
import { BulkAccountLoader } from '../account/bulkAccountLoader';
import { Program, Wallet } from '@coral-xyz/anchor';
import { BumpinUtils } from '../utils/utils';
import { BumpinTrade } from '../types/bumpin_trade';
import { Component } from './componet';
import { PollingStateAccountSubscriber } from '../account/pollingStateAccountSubscriber';
import { BumpinSubscriptionFailed } from '../errors';
import { DataAndSlot } from '../account/types';
import { PollingTradeTokenAccountSubscriber } from '../account/pollingTradeTokenAccountSubscriber';
import { StashedPythClient } from '../oracles/stashedPythClient';
import { TradeToken } from '../beans/beans';
import { BumpinClientConfig } from '../bumpinClientConfig';

export class TradeTokenComponent extends Component {
    bulkAccountLoader: BulkAccountLoader;
    program: Program<BumpinTrade>;
    tradeTokens: Map<string, PollingTradeTokenAccountSubscriber> = new Map();
    tradeTokenPyths: Map<string, StashedPythClient> = new Map();
    tradeTokenOraclePyths: Map<string, StashedPythClient> = new Map();

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
        let state = super.getStateSync();
        this.bulkAccountLoader = bulkAccountLoader;
        this.program = program;
        for (let i = 0; i < state.tradeTokenSequence; i++) {
            const [pda, _] = BumpinUtils.getTradeTokenPda(this.program, i);
            let tradeTokenAccountSubscriber =
                new PollingTradeTokenAccountSubscriber(
                    program,
                    pda,
                    bulkAccountLoader,
                );
            this.tradeTokens.set(pda.toString(), tradeTokenAccountSubscriber);
        }
    }

    public async subscribe() {
        this.tradeTokens.entries();
        for (let [
            key,
            tradeTokenAccountSubscriber,
        ] of this.tradeTokens.entries()) {
            await tradeTokenAccountSubscriber.subscribe();
            // let tradeToken: TradeToken =
            //     tradeTokenAccountSubscriber.getAccountAndSlot().data;
            // let stashedPythClient = new StashedPythClient(
            //     tradeToken.oracleKey,
            //     2,
            //     this.bulkAccountLoader,
            // );
            // await stashedPythClient.initialize();
            // this.tradeTokenPyths.set(key, stashedPythClient);
            // this.tradeTokenOraclePyths.set(
            //     tradeToken.oracleKey.toString(),
            //     stashedPythClient,
            // );
        }
    }

    public async unsubscribe() {
        for (let tradeTokenAccountSubscriber of this.tradeTokens.values()) {
            await tradeTokenAccountSubscriber.unsubscribe();
        }
    }

    public async getTradeTokens(sync: boolean = false): Promise<TradeToken[]> {
        let tradeTokens = await this.getTradeTokensWithSlot(sync);
        return tradeTokens.map((dataAndSlot) => dataAndSlot.data);
    }

    public getTradeTokensSync(): TradeToken[] {
        let tradeTokens = this.getTradeTokensWithSlotSync();
        return tradeTokens.map((dataAndSlot) => dataAndSlot.data);
    }

    public async getTradeToken(
        tradeTokenKey: PublicKey,
        sync: boolean = false,
    ): Promise<TradeToken> {
        let poolWithSlot = await this.getTradeTokenWithSlot(
            tradeTokenKey,
            sync,
        );
        return poolWithSlot.data;
    }

    public async getTradeTokenByOracleKey(
        oracleKey: PublicKey,
        sync: boolean = false,
    ): Promise<TradeToken> {
        let tradeTokens = await this.getTradeTokens(sync);
        for (let tradeToken of tradeTokens) {
            if (tradeToken.oracleKey.equals(oracleKey)) {
                return tradeToken;
            }
        }
        throw new BumpinSubscriptionFailed(
            `TradeToken with the oracle key ${oracleKey} does not exist`,
        );
    }

    public async getTradeTokenByMintKey(
        mintKey: PublicKey,
        sync: boolean = false,
    ): Promise<TradeToken> {
        let tradeTokens = await this.getTradeTokens(sync);
        for (let tradeToken of tradeTokens) {
            if (tradeToken.mintKey.equals(mintKey)) {
                return tradeToken;
            }
        }
        throw new BumpinSubscriptionFailed(
            `TradeToken with the mint key ${mintKey} does not exist`,
        );
    }

    public async getTradeTokensWithSlot(
        sync: boolean = false,
    ): Promise<DataAndSlot<TradeToken>[]> {
        let tradeTokensWithSlot: DataAndSlot<TradeToken>[] = [];
        for (let tradeTokenAccountSubscriber of this.tradeTokens.values()) {
            if (sync) {
                await tradeTokenAccountSubscriber.fetch();
            }
            tradeTokensWithSlot.push(
                tradeTokenAccountSubscriber.getAccountAndSlot(),
            );
        }
        return tradeTokensWithSlot;
    }

    public getTradeTokensWithSlotSync(): DataAndSlot<TradeToken>[] {
        let tradeTokensWithSlot: DataAndSlot<TradeToken>[] = [];
        for (let tradeTokenAccountSubscriber of this.tradeTokens.values()) {
            tradeTokensWithSlot.push(
                tradeTokenAccountSubscriber.getAccountAndSlot(),
            );
        }
        return tradeTokensWithSlot;
    }

    public async getTradeTokenWithSlot(
        tradeTokenKey: PublicKey,
        sync: boolean = false,
    ): Promise<DataAndSlot<TradeToken>> {
        const tradeTokenAccountSubscriber:
            | PollingTradeTokenAccountSubscriber
            | undefined = this.tradeTokens.get(tradeTokenKey.toString());
        if (tradeTokenAccountSubscriber === undefined) {
            throw new BumpinSubscriptionFailed(
                `TradeToken with the key ${tradeTokenKey} does not exist`,
            );
        }
        if (sync) {
            await tradeTokenAccountSubscriber.fetch();
        }
        return tradeTokenAccountSubscriber.getAccountAndSlot();
    }
}
