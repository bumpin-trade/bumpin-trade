import {Wallet} from "@coral-xyz/anchor";

export enum NetType {
    MAINNET_BETA = 'mainnet_beta',
    TESTNET = 'testnet',
    DEVNET = 'devnet',
    LOCALNET = 'localnet',
    CUSTOM = 'custom'
}

export type BumpinClientConfig = {
    netType: NetType;
    endpoint: string;
    wallet: Wallet | null;
    pollingFrequency: number
}

export class BumpinClientConfigBuilder {
    private config: BumpinClientConfig;

    private constructor(netType: NetType, endpoint: string) {
        this.config = {netType, endpoint, wallet: undefined, pollingFrequency: 1000};
    }

    public static mainnet_beta(): BumpinClientConfigBuilder {
        return new BumpinClientConfigBuilder(NetType.MAINNET_BETA, 'https://mainnet.endpoint.com');
    }

    public static localnet(): BumpinClientConfigBuilder {
        return new BumpinClientConfigBuilder(NetType.LOCALNET, 'http://127.0.0.1:8899');
    }

    public static customNet(endpoint: string): BumpinClientConfigBuilder {
        return new BumpinClientConfigBuilder(NetType.CUSTOM, endpoint);
    }

    public wallet(wallet: Wallet): BumpinClientConfigBuilder {
        this.config.wallet = wallet;
        return this;
    }

    public pollingFrequency(pollingFrequency: number): BumpinClientConfigBuilder {
        this.config.pollingFrequency = pollingFrequency;
        return this;
    }

    public build(): BumpinClientConfig {
        return this.config;
    }
}