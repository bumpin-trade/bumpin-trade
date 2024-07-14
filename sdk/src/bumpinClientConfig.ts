import * as anchor from "@coral-xyz/anchor";
import {Wallet} from "@coral-xyz/anchor";
import {PublicKey} from "@solana/web3.js";

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
    wallet: Wallet;
    pollingFrequency: number
}

export class BumpinClientConfigBuilder {
    private config: BumpinClientConfig;

    private constructor(netType: NetType, endpoint: string) {
        let fakeKeyPair = anchor.web3.Keypair.fromSecretKey(new Uint8Array([159, 222, 136, 96, 224, 28, 2, 126, 96, 31, 178, 12, 1, 194, 40, 140, 68, 226, 121, 253, 223, 156, 185, 179, 63, 203, 243, 26, 171, 54, 23, 240, 118, 96, 247, 225, 72, 140, 201, 40, 48, 149, 165, 42, 37, 180, 230, 21, 181, 53, 23, 130, 102, 124, 222, 172, 189, 57, 125, 39, 250, 96, 144, 120]));
        this.config = {netType, endpoint, wallet: new KeypairWallet(fakeKeyPair), pollingFrequency: 1000};
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

class BumpinSigner implements anchor.web3.Signer {
    publicKey: PublicKey;
    secretKey: Uint8Array;

    constructor(publicKey: PublicKey, secretKey: Uint8Array) {
        this.publicKey = publicKey;
        this.secretKey = secretKey;
    }
}


export class KeypairWallet implements anchor.Wallet {
    payer: anchor.web3.Keypair;

    constructor(keypair: anchor.web3.Keypair) {
        this.payer = keypair;
    }

    async signTransaction<T extends anchor.web3.Transaction | anchor.web3.VersionedTransaction>(tx: T): Promise<T> {
        if ('version' in tx) {
            tx.sign([new BumpinSigner(this.payer.publicKey, this.payer.secretKey)]);
        } else {
            tx.sign(new BumpinSigner(this.payer.publicKey, this.payer.secretKey));
        }
        return tx;
    }

    async signAllTransactions<T extends anchor.web3.Transaction | anchor.web3.VersionedTransaction>(txs: T[]): Promise<T[]> {
        txs.forEach((tx) => {
            if ('version' in tx) {
                tx.sign([new BumpinSigner(this.payer.publicKey, this.payer.secretKey)]);
            } else {
                tx.sign(new BumpinSigner(this.payer.publicKey, this.payer.secretKey));
            }
        });
        return txs;
    }

    get publicKey(): PublicKey {
        return this.payer.publicKey;
    }
}