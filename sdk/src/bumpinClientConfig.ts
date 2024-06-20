import {Wallet} from "@coral-xyz/anchor";

export type BumpinClientConfig = {
    endpoint: string;
    wallet: Wallet;
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