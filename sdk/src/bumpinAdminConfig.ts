import { Wallet } from '@coral-xyz/anchor';
import { NoneWallet } from './bumpinClientConfig';

export type BumpinAdminConfig = {
    endpoint: string;
    wallet: Wallet;
};

export class BumpinAdminConfigBuilder {
    private readonly config: BumpinAdminConfig;

    private constructor(endpoint: string) {
        this.config = { endpoint, wallet: new NoneWallet() };
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
