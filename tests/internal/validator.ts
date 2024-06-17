import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {BumpinTrade} from "../../target/types/bumpin_trade";
import {AccountMeta, LAMPORTS_PER_SOL, PublicKey} from "@solana/web3.js";
import {ACCOUNT_SIZE, createMint, TOKEN_PROGRAM_ID} from "@solana/spl-token";

describe("bumpin-trade", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;

    it("Is initialized!", async () => {
        console.log("Program ID: ", program.programId.toBase58());
        const kvUser = anchor.web3.Keypair.generate();
        const kvUser2 = anchor.web3.Keypair.generate();
        const feePayer = anchor.web3.Keypair.generate();
        const signer = anchor.web3.Keypair.generate();
        const wallet = anchor.Wallet.local();
        const [tokenAccount1Pda, bump] = await PublicKey.findProgramAddress(
            [Buffer.from("trade_token_vault")],
            program.programId
        );

        console.log("Token Account 1 PDA: ", tokenAccount1Pda.toBase58());

        //[124, 59, 210, 236, 9, 238, 55, 186]

        const lamports = await provider.connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE);
        const airdropSignature = await provider.connection.requestAirdrop(feePayer.publicKey, lamports * 100); // Request 2x lamports for safety
        await provider.connection.confirmTransaction(airdropSignature);

        const airdropSignature1 = await provider.connection.requestAirdrop(wallet.publicKey, LAMPORTS_PER_SOL);
        await provider.connection.confirmTransaction(airdropSignature1);

        // Create the mint
        const token = await createMint(
            provider.connection,
            feePayer,
            feePayer.publicKey,
            null,
            9, // Decimals
        );
        console.log("Token minted successfully: ", token.toBase58());


        await program.methods.initialize()
            .accounts(
                {
                    // tradeTokenVault: tokenAccount1Pda,
                    bumpSigner: signer.publicKey,
                    tradeTokenMint: token,
                    admin: feePayer.publicKey,
                    // systemProgram: anchor.web3.SystemProgram.programId,
                    // rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                    // tokenProgram: TOKEN_PROGRAM_ID,
                }
            )
            .signers([feePayer])
            .rpc();

        const remainingAccounts = new Array<AccountMeta>();
        remainingAccounts.push({
            pubkey: tokenAccount1Pda,
            isWritable: true,
            isSigner: false,
        });

        await program.rpc.initialize1({
            accounts: {
                keyValue: kvUser.publicKey,
                user: feePayer.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [kvUser, feePayer],
            remainingAccounts,
        });


        const remainingAccounts2 = new Array<AccountMeta>();
        remainingAccounts2.push({
            pubkey: tokenAccount1Pda,
            isWritable: true,
            isSigner: false,
        });

        remainingAccounts2.push({
            pubkey: kvUser.publicKey,
            isWritable: true,
            isSigner: false,
        });
        await program.rpc.initialize1({
            accounts: {
                keyValue: kvUser2.publicKey,
                user: feePayer.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [kvUser2, feePayer],
            remainingAccounts: remainingAccounts2,
        });

        console.log("Token accounts initialized successfully");

    });
});
