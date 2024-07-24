import * as anchor from '@coral-xyz/anchor';
import { Wallet } from '@coral-xyz/anchor';
import { ConnectionConfig, PublicKey } from '@solana/web3.js';

export enum NetType {
    MAINNET_BETA = 'mainnet_beta',
    TESTNET = 'testnet',
    DEVNET = 'devnet',
    LOCALNET = 'localnet',
    CUSTOM = 'custom',
}

export type BumpinClientConfig = {
    netType: NetType;
    verbose: boolean;
    endpoint: string;
    wallet: Wallet;
    connectionConfig?: ConnectionConfig;
    pollingFrequency: number;
};

export class BumpinClientConfigBuilder {
    private readonly config: BumpinClientConfig;

    private constructor(
        netType: NetType,
        endpoint: string,
        connectionConfig?: ConnectionConfig,
    ) {
        this.config = {
            netType,
            verbose: false,
            endpoint,
            wallet: new NoneWallet(),
            pollingFrequency: 1000,
            connectionConfig,
        };
    }

    public static mainnet_beta(): BumpinClientConfigBuilder {
        return new BumpinClientConfigBuilder(
            NetType.MAINNET_BETA,
            'https://mainnet.endpoint.com',
        );
    }

    public static localnet(): BumpinClientConfigBuilder {
        return new BumpinClientConfigBuilder(
            NetType.LOCALNET,
            'http://127.0.0.1:8899',
        );
    }

    public static customNet(
        endpoint: string,
        connectionConfig?: ConnectionConfig,
    ): BumpinClientConfigBuilder {
        return new BumpinClientConfigBuilder(
            NetType.CUSTOM,
            endpoint,
            connectionConfig,
        );
    }

    public verbose(verbose: boolean): BumpinClientConfigBuilder {
        this.config.verbose = verbose;
        return this;
    }

    public wallet(wallet: Wallet): BumpinClientConfigBuilder {
        this.config.wallet = wallet;
        return this;
    }

    public pollingFrequency(
        pollingFrequency: number,
    ): BumpinClientConfigBuilder {
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

export class NoneWallet implements anchor.Wallet {
    payer: anchor.web3.Keypair = anchor.web3.Keypair.generate();

    constructor() {}

    async signTransaction<
        T extends anchor.web3.Transaction | anchor.web3.VersionedTransaction,
    >(tx: T): Promise<T> {
        throw new Error('NoneWallet does not support signing transactions');
    }

    async signAllTransactions<
        T extends anchor.web3.Transaction | anchor.web3.VersionedTransaction,
    >(txs: T[]): Promise<T[]> {
        throw new Error('NoneWallet does not support signing transactions');
    }

    get publicKey(): PublicKey {
        throw new Error('NoneWallet does not have a public key');
    }
}

export class KeypairWallet implements anchor.Wallet {
    payer: anchor.web3.Keypair;

    constructor(keypair: anchor.web3.Keypair) {
        this.payer = keypair;
    }

    async signTransaction<
        T extends anchor.web3.Transaction | anchor.web3.VersionedTransaction,
    >(tx: T): Promise<T> {
        if ('version' in tx) {
            tx.sign([
                new BumpinSigner(this.payer.publicKey, this.payer.secretKey),
            ]);
        } else {
            tx.sign(
                new BumpinSigner(this.payer.publicKey, this.payer.secretKey),
            );
        }
        return tx;
    }

    async signAllTransactions<
        T extends anchor.web3.Transaction | anchor.web3.VersionedTransaction,
    >(txs: T[]): Promise<T[]> {
        txs.forEach((tx) => {
            if ('version' in tx) {
                tx.sign([
                    new BumpinSigner(
                        this.payer.publicKey,
                        this.payer.secretKey,
                    ),
                ]);
            } else {
                tx.sign(
                    new BumpinSigner(
                        this.payer.publicKey,
                        this.payer.secretKey,
                    ),
                );
            }
        });
        return txs;
    }

    get publicKey(): PublicKey {
        return this.payer.publicKey;
    }
}
