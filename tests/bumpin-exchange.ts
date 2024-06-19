import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {BumpinTrade} from "../target/types/bumpin_trade";
import {Pyth} from "../target/types/pyth";
import {Utils} from "./utils/utils";
import {assert} from 'chai';
import {PublicKey} from "@solana/web3.js";
import {ExchangeInitializeParams} from "./exchange/initialize_params";
import {BumpinExchange} from "./exchange/exchange";

describe("bumpin-exchange", () => {

    const program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;
    const programPyth = anchor.workspace.Pyth as Program<Pyth>;

    let admin: anchor.web3.Keypair;
    let Player1: anchor.web3.Keypair;
    let Player2: anchor.web3.Keypair;
    let pool_mint_account: anchor.web3.Keypair;
    let stable_pool_mint_account: anchor.web3.Keypair;
    let trade_token_mint_account: anchor.web3.Keypair;
    let oracle: anchor.web3.Keypair;
    let utils: Utils = new Utils();


    let exchange: BumpinExchange;

    before(async () => {
        let defaultExchangeInitializeParams = ExchangeInitializeParams.defaultParams();
        let ex = new BumpinExchange();
        await ex.initialize(defaultExchangeInitializeParams);
        exchange = ex;
    });

    it("Check State", async () => {
        const state = await program.account.state.fetch(utils.getStatePda(program)[0]);
        assert(state.fundingFeeBaseRate.toString() === "100");
    });

    it("Check User (Player1, Player2)", async () => {
        let pdaForPlayer1 = exchange.getUserPda("Player1");
        const player1 = await program.account.user.fetch(pdaForPlayer1[0]);
        assert(player1.hold.toString() === "0");

        let pdaForPlayer2 = exchange.getUserPda("Player2");
        const player2 = await program.account.user.fetch(pdaForPlayer2[0]);
        assert(player2.hold.toString() === "0");
    });

    it("Check Pool", async () => {
        let pdaForPoolBTC = exchange.getPoolPda("BUMP_P__BTC");
        const poolBTC = await program.account.pool.fetch(pdaForPoolBTC[0]);
        let pdaForPoolUSDC = exchange.getPoolPda("BUMP_P__USDC");
        const poolUSDC = await program.account.pool.fetch(pdaForPoolUSDC[0]);
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
        await exchange.mintTradeTokenToPlayer("Player1", "BTC", 1000, 9);
        // let tradeToken = await utils.createTokenAccount(program.provider, admin, trade_token_mint_account.publicKey, Player1.publicKey);
        // await utils.mintTo(program.provider, admin, trade_token_mint_account.publicKey, tradeToken.address, 1000, 9);
        // await utils.deposit(Player1, tradeToken.address, 0, new BN(100));
    });
});
