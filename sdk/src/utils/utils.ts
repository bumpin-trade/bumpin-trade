import {PublicKey, TransactionMessage, VersionedTransaction} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {BN, Program, Provider, Wallet} from "@coral-xyz/anchor";
import {Buffer} from "buffer";
import {BumpinTrade} from "../types/bumpin_trade";
import BigNumber from 'bignumber.js';

export class BumpinUtils {
    public static size2Amount(size: number, decimals: number): BN {
        const bigNum = new BigNumber(size).multipliedBy(new BigNumber(10).pow(Math.abs(decimals)));
        return new BN(bigNum.toFixed(0));
    }

    public static amount2Size(amount: BN, decimals: number): number {
        const bigNum = new BigNumber(amount.toString()).dividedBy(new BigNumber(10).pow(Math.abs(decimals)));
        return bigNum.toNumber();
    }

    public static decodeString(bytes: number[]): string {
        const buffer = Buffer.from(bytes);
        return buffer.toString('utf8').trim();
    }

    public static capitalize(value: string): string {
        return value[0].toUpperCase() + value.slice(1);
    }

    public static numberToLEBytes(number: number): Buffer {
        const buffer = Buffer.alloc(8);
        buffer.writeUInt32LE(number, 0);
        return buffer;
    }

    public static string2Padded32Bytes(str: string): number[] {
        const buffer = Buffer.from(str, 'utf-8');
        const paddedBuffer = Buffer.concat([buffer, Buffer.alloc(32 - buffer.length)]);
        return Array.from(paddedBuffer);
    }

    public static getPdaSync(program: Program<BumpinTrade>, seeds: Array<Buffer | Uint8Array>): [PublicKey, number] {
        const [address, nonce] = PublicKey.findProgramAddressSync(
            seeds,
            program.programId
        );
        return [address, nonce];
    }

    public static getBumpinStatePda(program: Program<BumpinTrade>): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from("bump_state")],
            program.programId
        );
    }

    public static getTradeTokenPda(program: Program<BumpinTrade>, index: number): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from("trade_token"), new anchor.BN(index).toArrayLike(Buffer, 'le', 2)],
            program.programId
        );
    }

    public static getTradeTokenVaultPda(program: Program<BumpinTrade>, index: number): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from("trade_token_vault"), new anchor.BN(index).toArrayLike(Buffer, 'le', 2)],
            program.programId
        );
    }

    public static getPoolPda(program: Program<BumpinTrade>, index: number): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from("pool"), new anchor.BN(index).toArrayLike(Buffer, 'le', 2)],
            program.programId
        );
    }

    public static getMarketPda(program: Program<BumpinTrade>, index: number): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from("market"), new anchor.BN(index).toArrayLike(Buffer, 'le', 2)],
            program.programId
        );
    }


    public static async manualCreateAccount(provider: Provider, wallet: Wallet, newAccountPk: anchor.web3.Keypair, space: number, lamports: number, programId: PublicKey) {
        let i = anchor.web3.SystemProgram.createAccount({
            fromPubkey: wallet.publicKey,
            newAccountPubkey: newAccountPk.publicKey,
            space: space,
            lamports: lamports,
            programId: programId,
        });
        let lastBlockHash = await provider.connection
            .getLatestBlockhash();
        let blockhash = lastBlockHash.blockhash;
        let lastValidBlockHeight = lastBlockHash.lastValidBlockHeight;


        const messageV0 = new TransactionMessage({
            instructions: [i],
            payerKey: wallet.publicKey,
            recentBlockhash: blockhash,
        }).compileToV0Message();
        const transaction = new VersionedTransaction(messageV0);
        let signedTransaction = await wallet.signTransaction(transaction);
        signedTransaction.sign([newAccountPk]);
        const signature = await provider.connection.sendTransaction(signedTransaction);
        await provider.connection.confirmTransaction({
            blockhash,
            lastValidBlockHeight,
            signature
        });

    }
}