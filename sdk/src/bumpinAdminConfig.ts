import {Wallet} from "@coral-xyz/anchor";

export type BumpinAdminConfig = {
    endpoint: string;
    wallet: Wallet;
}

export class BumpinAdminConfigBuilder {
    private config: BumpinAdminConfig;

    private constructor(endpoint: string) {
        this.config = {endpoint, wallet: undefined};
    }

    public static mainnet_beta(): BumpinAdminConfigBuilder {
        return new BumpinAdminConfigBuilder('https://mainnet.endpoint.com');
    }

    public static localnet(): BumpinAdminConfigBuilder {
        return new BumpinAdminConfigBuilder('http://127.0.0.1:8899');
    }

    public static customNet(endpoint: string): BumpinAdminConfigBuilder {
        return new BumpinAdminConfigBuilder(endpoint);
    }

    public wallet(wallet: Wallet): BumpinAdminConfigBuilder {
        this.config.wallet = wallet;
        return this;
    }

    public build(): BumpinAdminConfig {
        return this.config;
    }
}