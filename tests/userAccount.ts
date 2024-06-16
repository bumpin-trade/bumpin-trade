import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BumpinTrade } from "../target/types/bumpin_trade";

describe("bumpin-trade", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;

    it("User Account initialize", async () => {
        // const tokenAccount1 = anchor.web3.Keypair.generate();
        // const tokenAccount2 = anchor.web3.Keypair.generate();
        //
        // // Get the user's public key
        // const user = provider.wallet.publicKey;
        //
        // // Add your test here.
        // const tx = await program.methods.initialize({
        //     accounts: {
        //         tokenAccount1: tokenAccount1.publicKey,
        //         tokenAccount2: tokenAccount2.publicKey,
        //         user,
        //         systemProgram: SystemProgram.programId,
        //         rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        //     },
        //     signers: [tokenAccount1, tokenAccount2],
        //     instructions: [
        //         await program.account.tokenAccount.createInstruction(tokenAccount1),
        //         await program.account.tokenAccount.createInstruction(tokenAccount2),
        //     ],
        // }).rpc();
        // console.log("Your transaction signature", tx);
    });
});
