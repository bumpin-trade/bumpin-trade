import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {BumpinTrade} from "../target/types/bumpin_trade";
import {Pyth} from "../target/types/pyth";
import {Utils} from "./utils/utils";
import {assert} from 'chai';
import {PublicKey} from "@solana/web3.js";
import BN from "bn.js";
import {ExchangeInitializeParams} from "./exchange/initialize_params";
import {BumpinExchange} from "./exchange/exchange";

describe("bumpin-trade", () => {
    const provider = anchor.AnchorProvider.local();
    const program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    const programPyth = anchor.workspace.Pyth as Program<Pyth>;
    const utils = new Utils();

    let admin: anchor.web3.Keypair;
    let Player1: anchor.web3.Keypair;
    let Player2: anchor.web3.Keypair;
    let pool_mint_account: anchor.web3.Keypair;
    let stable_pool_mint_account: anchor.web3.Keypair;
    let trade_token_mint_account: anchor.web3.Keypair;
    let oracle: anchor.web3.Keypair;

    before(async () => {
        admin = await utils.new_user(provider);
        let oracle_payer = await utils.new_user(programPyth.provider as anchor.AnchorProvider);
        oracle = anchor.web3.Keypair.generate();
        await utils.manual_create_account(programPyth.provider, oracle_payer, oracle, 3312,
            await programPyth.provider.connection.getMinimumBalanceForRentExemption(
                3312
            ), programPyth.programId);

        await utils.initialize_oracle(oracle, 70000);
        pool_mint_account = await utils.create_mint_account(admin, admin);
        stable_pool_mint_account = await utils.create_mint_account(admin, admin);
        trade_token_mint_account = await utils.create_mint_account(admin, admin);
        await utils.initialize_state(admin);
        await utils.initialize_pool(program,pool_mint_account.publicKey, "BUMP_P__BTC", admin);
        await utils.initialize_pool(program,stable_pool_mint_account.publicKey, "BUMP_P__USDC", admin);
        Player1 = await utils.new_user(provider);
        await utils.initialize_user(Player1, admin);
        Player2 = await utils.new_user(provider);
        await utils.initialize_user(Player2, admin);
        await utils.initialize_trade_token(trade_token_mint_account.publicKey, admin, oracle.publicKey, new BN(10), new BN(1));
    });

    it("Check State", async () => {
        const state = await program.account.state.fetch(utils.get_bump_state_pk());
        assert(state.fundingFeeBaseRate.toString() === "100");
    });

    it("Check User (Player1, Player2)", async () => {
        //TODO: Do better
        const [meAddress1, nonce1] = PublicKey.findProgramAddressSync(
            [Buffer.from("user"), Player1.publicKey.toBuffer()],
            program.programId
        );
        const m1 = await program.account.user.fetch(meAddress1);
        assert(m1.hold.toString() === "0");

        const [meAddress2, nonce2] = PublicKey.findProgramAddressSync(
            [Buffer.from("user"), Player2.publicKey.toBuffer()],
            program.programId
        );
        const m2 = await program.account.user.fetch(meAddress2);
        assert(m2.hold.toString() === "0");
    });

    it("Check Pool", async () => {
        const stateNumberOfPoolsBytes = new Uint8Array(new Uint16Array([0]).buffer);
        const seeds = [
            Buffer.from('pool'),
            stateNumberOfPoolsBytes
        ];
        const [meAddress, nonce] = PublicKey.findProgramAddressSync(
            seeds,
            program.programId
        );
        await program.account.pool.fetch(meAddress);
    });


    it("Check TradeToken", async () => {
        const stateNumberOfPoolsBytes = new Uint8Array(new Uint16Array([0]).buffer);
        const seeds = [
            Buffer.from('trade_token'),
            stateNumberOfPoolsBytes
        ];
        const [meAddress, nonce] = PublicKey.findProgramAddressSync(
            seeds,
            program.programId
        );
        await program.account.tradeToken.fetch(meAddress);
    });


    it("Mint & Deposit for Play1", async () => {
        let tradeToken = await utils.createTokenAccount(program.provider, admin, trade_token_mint_account.publicKey, Player1.publicKey);
        await utils.mintTo(program.provider, admin, trade_token_mint_account.publicKey, tradeToken.address, 1000, 9);
        await utils.deposit(Player1, tradeToken.address, 0, new BN(100));
    });
});
