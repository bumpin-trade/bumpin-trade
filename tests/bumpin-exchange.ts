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


    it("Mint for Player1 & Player2", async () => {
        let player1 = exchange.getPlayer("Player1");
        let tradeTokenBtc = exchange.getTradeToken("BTC");
        await player1.mintTradeToken("BTC", tradeTokenBtc.mint.publicKey, 1000, 9);

        let player2 = exchange.getPlayer("Player2");
        let tradeTokenUSDC = exchange.getTradeToken("USDC");
        await player2.mintTradeToken("USDC", tradeTokenUSDC.mint.publicKey, 1000, 9);
    });

    it("Deposit for Player1 & Player2", async () => {
        await exchange.playerDeposit("Player1", "BTC", 500);
        await exchange.playerDeposit("Player2", "USDC", 500);

        // let pdaForPlayer1 = exchange.getUserPda("Player1");
        // const player1 = await program.account.user.fetch(pdaForPlayer1[0]);
    });
});
