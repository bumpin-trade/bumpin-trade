import {PublicKey, TransactionMessage, VersionedTransaction} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {Program, Provider} from "@coral-xyz/anchor";
import {Buffer} from "buffer";
import {BumpinTrade} from "../types/bumpin_trade";

export class BumpinUtils {
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
            [Buffer.from("trade_token"), Buffer.from(index.toString())],
            program.programId
        );
    }

    public static getPoolPda(program: Program<BumpinTrade>, index: number): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from("pool"), Buffer.from(index.toString())],
            program.programId
        );
    }

    public static getMarketPda(program: Program<BumpinTrade>, index: number): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from("market"), Buffer.from(index.toString())],
            program.programId
        );
    }

    public static async manualCreateAccount(provider: Provider, fromPk: PublicKey, newAccountPk: anchor.web3.Keypair, space: number, lamports: number, programId: PublicKey) {
        let i = anchor.web3.SystemProgram.createAccount({
            fromPubkey: fromPk,
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
            payerKey: fromPk,
            recentBlockhash: blockhash,
        }).compileToV0Message();
        const transaction = new VersionedTransaction(messageV0);
        transaction.sign([newAccountPk]);
        const signature = await provider.connection.sendTransaction(transaction);
        await provider.connection.confirmTransaction({
            blockhash,
            lastValidBlockHeight,
            signature
        });

    }
}