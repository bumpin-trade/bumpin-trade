import {
    AccountMeta,
    ConfirmOptions,
    PublicKey,
    TransactionMessage,
    VersionedTransaction,
} from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { BN, Program, Provider, Wallet } from '@coral-xyz/anchor';
import { Buffer } from 'buffer';
import { BumpinTrade } from '../types/bumpin_trade';
import BigNumber from 'bignumber.js';
import { OrderSide } from '../beans/beans';
import { OrderSideAccount } from '../typedef';

export class BumpinUtils {
    public static bigintToUsd(
        amount: bigint,
        tokenPrice: number,
        decimals: number,
    ): BigNumber {
        return BumpinUtils.toUsd(
            new BN(amount.toString()),
            tokenPrice,
            decimals,
        );
    }

    public static toUsd(
        amount: BN,
        tokenPrice: number,
        decimals: number,
    ): BigNumber {
        let size = this.amount2Size(amount, decimals);
        return size.multipliedBy(tokenPrice);
    }

    public static toUsdBN(
        amount: BN,
        tokenPrice: number,
        decimals: number,
    ): BN {
        let size = this.amount2Size(amount, decimals);
        return this.size2Amount(size.multipliedBy(tokenPrice), decimals);
    }

    public static size2Amount(size: BigNumber, decimals: number): BN {
        const bigNum = size.multipliedBy(
            new BigNumber(10).pow(Math.abs(decimals)),
        );
        return new BN(bigNum.toString());
    }

    public static amount2Size(amount: BN, decimals: number): BigNumber {
        return new BigNumber(amount.toString()).dividedBy(
            new BigNumber(10).pow(Math.abs(decimals)),
        );
    }

    public static number2Precision(amount: number, decimals: number): BN {
        return new BN(
            BigNumber(amount)
                .multipliedBy(new BigNumber(10).pow(Math.abs(decimals)))
                .integerValue()
                .toString(),
        );
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

    public static encodeString(str: string): number[] {
        const buffer = Buffer.from(str, 'utf-8');
        const paddedBuffer = Buffer.concat([
            buffer,
            Buffer.alloc(32 - buffer.length),
        ]);
        return Array.from(paddedBuffer);
    }

    public static getPdaSync(
        program: Program<BumpinTrade>,
        seeds: Array<Buffer | Uint8Array>,
    ): [PublicKey, number] {
        const [address, nonce] = PublicKey.findProgramAddressSync(
            seeds,
            program.programId,
        );
        return [address, nonce];
    }

    public static getBumpinStatePda(
        program: Program<BumpinTrade>,
    ): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from('bump_state')],
            program.programId,
        );
    }

    public static getTradeTokenPda(
        program: Program<BumpinTrade>,
        index: number,
    ): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from('trade_token'),
                new anchor.BN(index).toArrayLike(Buffer, 'le', 2),
            ],
            program.programId,
        );
    }

    public static getTradeTokenVaultPda(
        program: Program<BumpinTrade>,
        index: number,
    ): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from('trade_token_vault'),
                new anchor.BN(index).toArrayLike(Buffer, 'le', 2),
            ],
            program.programId,
        );
    }

    public static getPoolPda(
        program: Program<BumpinTrade>,
        index: number,
    ): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from('pool'),
                new anchor.BN(index).toArrayLike(Buffer, 'le', 2),
            ],
            program.programId,
        );
    }

    public static getRewardsPda(
        program: Program<BumpinTrade>,
        index: number,
    ): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from('rewards'),
                new anchor.BN(index).toArrayLike(Buffer, 'le', 2),
            ],
            program.programId,
        );
    }

    public static getPoolVaultPda(
        program: Program<BumpinTrade>,
        pool_index: number,
    ): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from('pool_vault'),
                new anchor.BN(pool_index).toArrayLike(Buffer, 'le', 2),
            ],
            program.programId,
        );
    }

    public static getPoolRewardsVaultPda(
        program: Program<BumpinTrade>,
        pool_index: number,
    ): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from('pool_rewards_vault'),
                new anchor.BN(pool_index).toArrayLike(Buffer, 'le', 2),
            ],
            program.programId,
        );
    }

    public static getMarketPda(
        program: Program<BumpinTrade>,
        index: number,
    ): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [
                Buffer.from('market'),
                new anchor.BN(index).toArrayLike(Buffer, 'le', 2),
            ],
            program.programId,
        );
    }

    public static prettyUsd(amount: BigNumber, max: number = 8): number {
        let amountStr = amount.toFixed(max);
        if (amountStr.length <= max + 1) {
            return parseFloat(amountStr);
        }
        const [integerPart, decimalPart] = amountStr.split('.');
        if (integerPart.length >= max + 1) {
            return parseFloat(integerPart.slice(0, max + 1));
        }
        const maxDecimalLength = max - integerPart.length;
        const truncatedDecimalPart = decimalPart.slice(0, maxDecimalLength);
        const formattedAmountStr = `${integerPart}.${truncatedDecimalPart}`;
        return parseFloat(formattedAmountStr);
    }

    public static getDefaultConfirmOptions() {
        let opt: ConfirmOptions = {
            skipPreflight: false,
            commitment: 'confirmed', //default commitment: confirmed
            preflightCommitment: 'confirmed',
            maxRetries: 3,
            minContextSlot: undefined,
        };
        return opt;
    }

    public static getRootConfirmOptions() {
        let opt: ConfirmOptions = {
            skipPreflight: false,
            commitment: 'root',
            preflightCommitment: 'root',
            maxRetries: 0,
            minContextSlot: undefined,
        };
        return opt;
    }

    public static removeDuplicateAccounts(
        accounts: Array<AccountMeta>,
    ): AccountMeta[] {
        let accountMap = new Map<string, AccountMeta>();
        for (let account of accounts) {
            accountMap.set(account.pubkey.toString(), account);
        }
        return Array.from(accountMap.values());
    }

    public static async manualCreateAccount(
        provider: Provider,
        wallet: Wallet,
        newAccountPk: anchor.web3.Keypair,
        space: number,
        lamports: number,
        programId: PublicKey,
    ) {
        let i = anchor.web3.SystemProgram.createAccount({
            fromPubkey: wallet.publicKey,
            newAccountPubkey: newAccountPk.publicKey,
            space: space,
            lamports: lamports,
            programId: programId,
        });
        let lastBlockHash = await provider.connection.getLatestBlockhash();
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
        const signature = await provider.connection.sendTransaction(
            signedTransaction,
        );
        await provider.connection.confirmTransaction({
            blockhash,
            lastValidBlockHeight,
            signature,
        });
    }

    public static prettyPrintParam(
        param: any,
        headMessage?: string,
        indent: string = '',
    ): void {
        if (headMessage) {
            console.log(
                '==============={ ' + headMessage + ' }===============',
            );
        }
        if (param === null || param === undefined) {
            console.log(indent + 'Param is null or undefined');
            return;
        }

        if (typeof param !== 'object') {
            console.log(indent + `[Value]: ${param.toString()}`);
            return;
        }

        for (const [key, value] of Object.entries(param)) {
            if (
                Array.isArray(value) &&
                value.every((item) => typeof item === 'number')
            ) {
                console.log(
                    indent + `[${key}] : ${BumpinUtils.decodeString(value)}`,
                );
            } else if (
                value instanceof BN ||
                value instanceof BigNumber ||
                value instanceof PublicKey
            ) {
                console.log(indent + `[${key}] : ${value.toString()}`);
            } else if (typeof value === 'object' && value !== null) {
                console.log(indent + `[${key}] : {`);
                BumpinUtils.prettyPrintParam(value, undefined, indent + '  ');
                console.log(indent + `}`);
            } else if (value === null) {
                console.log(indent + `[${key}] : null | undefined`);
            } else {
                console.log(indent + `[${key}] : ${value!.toString()}`);
            }
        }
    }
}
