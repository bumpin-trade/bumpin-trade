import {PublicKey} from "@solana/web3.js";
import {Program} from "@coral-xyz/anchor";
import {Buffer} from "buffer";
import {BumpinTrade} from "../types/bumpin_trade";

export class BumpinUtils {
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
}