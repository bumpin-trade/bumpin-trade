import {Wallet, Program} from "@coral-xyz/anchor";
import {
    PublicKey
} from '@solana/web3.js';
import {OracleClient} from "./oracles/types";
import {BulkAccountLoader} from "./account/bulkAccountLoader";
import {State} from "./types";
import {BumpinTrade} from "./types/bumpin_trade";

export type BumpinClientConfig = {
    endpoint: string;
    program: Program<BumpinTrade>;
    wallet: Wallet;
    userAccountPublicKey: PublicKey;
    state: State;
    oracleClient: OracleClient;
    bulkAccountLoader: BulkAccountLoader;
}

export class BumpinClientConfigBuilder {
    private config: BumpinClientConfig;

    private constructor(endpoint: string) {
        this.config = {endpoint, wallet: undefined};
    }

    public static mainnet_beta(): BumpinClientConfigBuilder {
        return new BumpinClientConfigBuilder('https://mainnet.endpoint.com');
    }

    public static localnet(): BumpinClientConfigBuilder {
        return new BumpinClientConfigBuilder('http://127.0.0.1:8899');
    }

    public wallet(wallet: Wallet): BumpinClientConfigBuilder {
        this.config.wallet = wallet;
        return this;
    }

    public build(): BumpinClientConfig {
        return this.config;
    }
}