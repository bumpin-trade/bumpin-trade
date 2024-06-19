import {PublicKey} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {BumpinTrade} from "./types/bumpin_trade";
import {Pyth} from "./types/pyth";

export class Utils {
    // provider = anchor.AnchorProvider.local();
    program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    programPyth = anchor.workspace.Pyth as Program<Pyth>;


    public getStatePda(): [PublicKey, number] {
        return PublicKey.findProgramAddressSync(
            [Buffer.from("bump_state")],
            this.program.programId
        );

    }


}