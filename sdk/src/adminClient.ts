import {Connection, PublicKey} from '@solana/web3.js';
import * as anchor from "@coral-xyz/anchor";
import {Idl, Wallet} from "@coral-xyz/anchor";
import idlBumpinTrade from "./idl/bumpin_trade.json"

export class Utils {


    public static async initState(pubKey: PublicKey, wallet: Wallet) {
        console.log("Init state");
        const connection = new Connection('http://127.0.0.1:8899');
        const provider = new anchor.AnchorProvider(connection, wallet, anchor.AnchorProvider.defaultOptions());
        const program = new anchor.Program(idlBumpinTrade as Idl, provider);
        console.log("finished program init");

        const param = {
            minOrderMarginUsd: new anchor.BN(100),
            maxMaintenanceMarginRate: new anchor.BN(0.1),
            fundingFeeBaseRate: new anchor.BN(0.0001),
            maxFundingBaseRate: new anchor.BN(0.0005),
            tradingFeeStakingRewardsRatio: new anchor.BN(0.5),
            tradingFeePoolRewardsRatio: new anchor.BN(0.5),
            tradingFeeUsdPoolRewardsRatio: new anchor.BN(0.5),
            borrowingFeeStakingRewardsRatio: new anchor.BN(0.5),
            borrowingFeePoolRewardsRatio: new anchor.BN(0.5),
            minPrecisionMultiple: new anchor.BN(100),
            mintFeeStakingRewardsRatio: new anchor.BN(0.5),
            mintFeePoolRewardsRatio: new anchor.BN(0.5),
            redeemFeeStakingRewardsRatio: new anchor.BN(0.5),
            redeemFeePoolRewardsRatio: new anchor.BN(0.5),
            poolRewardsIntervalLimit: new anchor.BN(100),
            initFee: new anchor.BN(0.0001),
            stakingFeeRewardRatio: new anchor.BN(0.5),
            poolFeeRewardRatio: new anchor.BN(0.5)
        };


        console.log("param init done: ", param);

        await program.methods.initializeState(
            param
        ).accounts({
            admin: pubKey,
        }).signers([]).rpc();
    }


}