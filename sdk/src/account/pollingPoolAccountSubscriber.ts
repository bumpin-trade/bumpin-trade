import { AccountSubscriber, DataAndSlot } from "./types";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Pool } from "../typedef";
import { BulkAccountLoader } from "./bulkAccountLoader";
import { BumpinTrade } from "../types/bumpin_trade";

export class PollingPoolAccountSubscriber implements AccountSubscriber<Pool> {
  isSubscribed: boolean;
  program: Program<BumpinTrade>;
  poolPublicKey: PublicKey;

  accountLoader: BulkAccountLoader;
  callbackId?: string;
  errorCallbackId?: string;

  pool?: DataAndSlot<Pool>;

  public constructor(
    program: Program<BumpinTrade>,
    poolPublicKey: PublicKey,
    accountLoader: BulkAccountLoader
  ) {
    this.isSubscribed = false;
    this.program = program;
    this.accountLoader = accountLoader;
    this.poolPublicKey = poolPublicKey;
  }

  async subscribe(userAccount?: Pool): Promise<boolean> {
    if (this.isSubscribed) {
      return true;
    }

    if (userAccount) {
      this.pool = { data: userAccount, slot: 0 };
    }

    await this.addToAccountLoader();

    await this.fetchIfUnloaded();
    if (this.doesAccountExist()) {
    }

    this.isSubscribed = true;
    return true;
  }

  async addToAccountLoader(): Promise<void> {
    if (this.callbackId) {
      return;
    }

    this.callbackId = await this.accountLoader.addAccount(
      this.poolPublicKey,
      (buffer, slot: number) => {
        if (!buffer) {
          return;
        }

        if (this.pool && this.pool.slot > slot) {
          return;
        }

        const account = this.program.account.pool.coder.accounts.decode(
          "pool",
          buffer
        );
        this.pool = { data: account, slot };

        console.log("PoolAccount updated start =====================");
        this.printPool(this.pool);
        console.log("PoolAccount updated end   =====================");
      }
    );

    this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {});
  }

  async fetchIfUnloaded(): Promise<void> {
    if (this.pool === undefined) {
      await this.fetch();
    }
  }

  async fetch(): Promise<void> {
    try {
      const dataAndContext = await this.program.account.pool.fetchAndContext(
        this.poolPublicKey,
        this.accountLoader.commitment
      );
      if (dataAndContext.context.slot > (this.pool?.slot ?? 0)) {
        this.pool = {
          data: dataAndContext.data as any as Pool,
          slot: dataAndContext.context.slot,
        };
      }
    } catch (e: any) {
      console.log(
        `PollingPoolAccountSubscriber.fetch() PoolAccount does not exist: ${e.message}`
      );
    }
  }

  doesAccountExist(): boolean {
    return this.pool !== undefined;
  }

  async unsubscribe(): Promise<void> {
    if (!this.isSubscribed || !this.callbackId) {
      return;
    }

    this.accountLoader.removeAccount(this.poolPublicKey, this.callbackId);
    this.callbackId = undefined;

    if (this.errorCallbackId)
      this.accountLoader.removeErrorCallbacks(this.errorCallbackId);
    this.errorCallbackId = undefined;

    this.isSubscribed = false;
  }

  assertIsSubscribed(): void {
    if (!this.isSubscribed) {
      throw new Error("You must call `subscribe` before using this function");
    }
  }

  public getAccountAndSlot(): DataAndSlot<Pool> {
    if (!this.doesAccountExist() || !this.pool) {
      throw new Error(
        "You must call `subscribe` or `fetch` before using this function"
      );
    }
    return this.pool;
  }

  public updateData(userAccount: Pool, slot: number): void {
    if (!this.pool || this.pool.slot < slot) {
      this.pool = { data: userAccount, slot };
      /*
            this.eventEmitter.emit('userAccountUpdate', userAccount);
            this.eventEmitter.emit('update');*/
    }
  }

  private printPool(pool_data: DataAndSlot<Pool>) {
    console.log(`Pool Info: ${pool_data.data}`);
    let pool = pool_data.data;
    console.log(`Name: ${pool.name.join(", ")}`);
    console.log(`PnL: ${pool.pnl.toString()}`);
    console.log(`APR: ${pool.apr.toString()}`);
    console.log(
      `Insurance Fund Amount: ${pool.insuranceFundAmount.toString()}`
    );
    console.log(`Total Supply: ${pool.totalSupply.toString()}`);
    console.log(`Index: ${pool.index}`);
    console.log(`Status: ${pool.status}`);
    console.log(`Stable: ${pool.stable}`);

    console.log("Pool Vault Key:");
    console.log(`  ${pool.poolVaultKey.toString()}`);
    console.log("Key:");
    console.log(`  ${pool.key.toString()}`);
    console.log("Stable Mint Key:");
    console.log(`  ${pool.stableMintKey.toString()}`);
    console.log("Mint Key:");
    console.log(`  ${pool.mintKey.toString()}`);

    console.log("Balance:");
    console.log(
      `  Settle Funding Fee: ${pool.balance.settleFundingFee.toString()}`
    );
    console.log(`  Amount: ${pool.balance.amount.toString()}`);
    console.log(`  Hold Amount: ${pool.balance.holdAmount.toString()}`);
    console.log(`  Unsettle Amount: ${pool.balance.unSettleAmount.toString()}`);
    console.log(
      `  Settle Funding Fee Amount: ${pool.balance.settleFundingFeeAmount.toString()}`
    );
    console.log(`  Loss Amount: ${pool.balance.lossAmount.toString()}`);

    console.log("Stable Balance:");
    console.log(
      `  Settle Funding Fee: ${pool.stableBalance.settleFundingFee.toString()}`
    );
    console.log(`  Amount: ${pool.stableBalance.amount.toString()}`);
    console.log(`  Hold Amount: ${pool.stableBalance.holdAmount.toString()}`);
    console.log(
      `  Unsettle Amount: ${pool.stableBalance.unSettleAmount.toString()}`
    );
    console.log(
      `  Settle Funding Fee Amount: ${pool.stableBalance.settleFundingFeeAmount.toString()}`
    );
    console.log(`  Loss Amount: ${pool.stableBalance.lossAmount.toString()}`);

    console.log("Borrowing Fee:");
    console.log(
      `  Total Borrowing Fee: ${pool.borrowingFee.totalBorrowingFee.toString()}`
    );
    console.log(
      `  Total Realized Borrowing Fee: ${pool.borrowingFee.totalRealizedBorrowingFee.toString()}`
    );
    console.log(
      `  Cumulative Borrowing Fee Per Token: ${pool.borrowingFee.cumulativeBorrowingFeePerToken.toString()}`
    );
    console.log(`  Updated At: ${pool.borrowingFee.updatedAt.toString()}`);

    console.log("Fee Reward:");
    console.log(`  Fee Amount: ${pool.feeReward.feeAmount.toString()}`);
    console.log(
      `  Unsettle Fee Amount: ${pool.feeReward.unSettleFeeAmount.toString()}`
    );
    console.log(
      `  Cumulative Rewards Per Stake Token: ${pool.feeReward.cumulativeRewardsPerStakeToken.toString()}`
    );
    console.log(
      `  Last Rewards Per Stake Token Deltas: ${pool.feeReward.lastRewardsPerStakeTokenDeltas
        .map((delta) => delta.toString())
        .join(", ")}`
    );

    console.log("Stable Fee Reward:");
    console.log(`  Fee Amount: ${pool.stableFeeReward.feeAmount.toString()}`);
    console.log(
      `  Unsettle Fee Amount: ${pool.stableFeeReward.unSettleFeeAmount.toString()}`
    );
    console.log(
      `  Cumulative Rewards Per Stake Token: ${pool.stableFeeReward.cumulativeRewardsPerStakeToken.toString()}`
    );
    console.log(
      `  Last Rewards Per Stake Token Deltas: ${pool.stableFeeReward.lastRewardsPerStakeTokenDeltas
        .map((delta) => delta.toString())
        .join(", ")}`
    );

    console.log("Config:");
    console.log(
      `  Minimum Stake Amount: ${pool.config.minimumStakeAmount.toString()}`
    );
    console.log(
      `  Minimum UnStake Amount: ${pool.config.minimumUnStakeAmount.toString()}`
    );
    console.log(
      `  Pool Liquidity Limit: ${pool.config.poolLiquidityLimit.toString()}`
    );
    console.log(
      `  Borrowing Interest Rate: ${pool.config.borrowingInterestRate.toString()}`
    );
    console.log(`  Stake Fee Rate: ${pool.config.stakeFeeRate}`);
    console.log(`  UnStake Fee Rate: ${pool.config.unStakeFeeRate}`);
    console.log(
      `  UnSettle Mint Ratio Limit: ${pool.config.unSettleMintRatioLimit}`
    );
    console.log(`  Padding: ${pool.config.padding.join(", ")}`);
  }
}
