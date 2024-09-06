import { Wallet } from '@coral-xyz/anchor';
import { NoneWallet } from './bumpinClientConfig';
import { ConnectionConfig } from '@solana/web3.js';

export type BumpinAdminConfig = {
    endpoint: string;
    wallet: Wallet;
    connectionConfig?: ConnectionConfig;
};

export class BumpinAdminConfigBuilder {
    private readonly config: BumpinAdminConfig;

    private constructor(endpoint: string, connectionConfig?: ConnectionConfig) {
        this.config = {
            endpoint,
            wallet: new NoneWallet(),
            connectionConfig: connectionConfig,
        };
    }

    public static mainnet_beta(): BumpinAdminConfigBuilder {
        return new BumpinAdminConfigBuilder('https://mainnet.endpoint.com');
    }

    public static devnet(): BumpinAdminConfigBuilder {
        return new BumpinAdminConfigBuilder('https://api.devnet.solana.com');
    }

    public static localnet(): BumpinAdminConfigBuilder {
        return new BumpinAdminConfigBuilder('http://127.0.0.1:8899');
    }

    public static customNet(
        endpoint: string,
        connectionConfig?: ConnectionConfig,
    ): BumpinAdminConfigBuilder {
        return new BumpinAdminConfigBuilder(endpoint, connectionConfig);
    }

    public wallet(wallet: Wallet): BumpinAdminConfigBuilder {
        this.config.wallet = wallet;
        return this;
    }

    public build(): BumpinAdminConfig {
        return this.config;
    }
}
