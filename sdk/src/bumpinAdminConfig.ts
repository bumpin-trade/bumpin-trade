import * as anchor from "@coral-xyz/anchor";
import {Wallet} from "@coral-xyz/anchor";
import {KeypairWallet} from "./bumpinClientConfig";

export type BumpinAdminConfig = {
    endpoint: string;
    wallet: Wallet;
}

export class BumpinAdminConfigBuilder {
    private config: BumpinAdminConfig;

    private constructor(endpoint: string) {
        let fakeKeyPair = anchor.web3.Keypair.fromSecretKey(new Uint8Array([159, 222, 136, 96, 224, 28, 2, 126, 96, 31, 178, 12, 1, 194, 40, 140, 68, 226, 121, 253, 223, 156, 185, 179, 63, 203, 243, 26, 171, 54, 23, 240, 118, 96, 247, 225, 72, 140, 201, 40, 48, 149, 165, 42, 37, 180, 230, 21, 181, 53, 23, 130, 102, 124, 222, 172, 189, 57, 125, 39, 250, 96, 144, 120]));
        this.config = {endpoint, wallet: new KeypairWallet(fakeKeyPair)};
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