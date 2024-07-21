import {PublicKey} from "@solana/web3.js";
import {BN} from "@coral-xyz/anchor";
import {PriceData} from "@pythnetwork/client";
import BigNumber from "bignumber.js";
import {isEqual} from "lodash";
import {StateAccount} from "../typedef";

/**
 * export type StateAccount = {
 *     admin: PublicKey;
 *     bumpSigner: PublicKey;
 *     keeperSigner: PublicKey;
 *     bumpSignerNonce: number;
 *     marketSequence: number;
 *     poolSequence: number;
 *     tradeTokenSequence: number;
 *     minimumOrderMarginUsd: BN;
 *     maximumMaintenanceMarginRate: number;
 *     fundingFeeBaseRate: BN;
 *     maximumFundingBaseRate: BN;
 *     minimumPrecisionMultiple: BN;
 *     poolRewardsIntervalLimit: BN;
 *     initFee: number;
 *     tradingFeeUsdPoolRewardsRatio: number;
 *     poolFeeRewardRatio: number;
 * };
 */

// export class State {
//     public admin: PublicKey;
//     public bumpSigner: PublicKey;
//     public keeperSigner: PublicKey;
//     public bumpSignerNonce: number;
//     public marketSequence: number;
//     public poolSequence: number;
//     public tradeTokenSequence: number;
//     public minimumOrderMarginUsd: BigNumber;
//     public maximumMaintenanceMarginRate: number;
//     public fundingFeeBaseRate: BigNumber;
//     public maximumFundingBaseRate: BigNumber;
//     public minimumPrecisionMultiple: BigNumber;
//     public poolRewardsIntervalLimit: BigNumber;
//     public initFee: number;
//     public tradingFeeUsdPoolRewardsRatio: number;
//     public poolFeeRewardRatio: number;
//
//     constructor(state:StateAccount) {
//         this.admin = state.admin;
//         this.bumpSigner = state.bumpSigner;
//         this.keeperSigner = state.keeperSigner;
//         this.bumpSignerNonce = state.bumpSignerNonce;
//         this.marketSequence = state.marketSequence;
//         this.poolSequence = state.poolSequence;
//         this.tradeTokenSequence = state.tradeTokenSequence;
//         this.minimumOrderMarginUsd = new BigNumber(state.minimumOrderMarginUsd);
//         this.maximumMaintenanceMarginRate = state.maximumMaintenanceMarginRate;
//         this.fundingFeeBaseRate = new BigNumber(state.fundingFeeBaseRate);
//         this.maximumFundingBaseRate = new BigNumber(state.maximumFundingBaseRate);
//         this.minimumPrecisionMultiple = new BigNumber(state.minimumPrecisionMultiple);
//         this.poolRewardsIntervalLimit = new BigNumber(state.poolRewardsIntervalLimit);
//         this.initFee = state.initFee;
//         this.tradingFeeUsdPoolRewardsRatio = state.tradingFeeUsdPoolRewardsRatio;
//         this.poolFeeRewardRatio = state.poolFeeRewardRatio;
//     }
//
//
// }