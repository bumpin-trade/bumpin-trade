import { AccountSubscriber, DataAndSlot } from "./types";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import {
  OrderStatus,
  PositionStatus,
  UserAccount,
  UserStakeStatus,
  UserTokenStatus,
} from "../typedef";
import { BulkAccountLoader } from "./bulkAccountLoader";
import { BumpinTrade } from "../types/bumpin_trade";
// @ts-ignore
import { isEqual } from "lodash";

export class PollingUserAccountSubscriber
  implements AccountSubscriber<UserAccount>
{
  isSubscribed: boolean;
  program: Program<BumpinTrade>;
  userAccountPublicKey: PublicKey;

  accountLoader: BulkAccountLoader;
  callbackId?: string;
  errorCallbackId?: string;

  user?: DataAndSlot<UserAccount>;

  public constructor(
    program: Program<BumpinTrade>,
    userAccountPublicKey: PublicKey,
    accountLoader: BulkAccountLoader
  ) {
    this.isSubscribed = false;
    this.program = program;
    this.accountLoader = accountLoader;
    this.userAccountPublicKey = userAccountPublicKey;
  }

  async subscribe(userAccount?: UserAccount): Promise<boolean> {
    if (this.isSubscribed) {
      return true;
    }

    if (userAccount) {
      this.user = { data: userAccount, slot: 0 };
    }

    await this.addToAccountLoader();

    await this.fetchIfUnloaded();
    if (this.doesAccountExist()) {
      //  this.eventEmitter.emit('update');
    }

    this.isSubscribed = true;
    return true;
  }

  async addToAccountLoader(): Promise<void> {
    if (this.callbackId) {
      return;
    }

    this.callbackId = await this.accountLoader.addAccount(
      this.userAccountPublicKey,
      (buffer, slot: number) => {
        if (!buffer) {
          return;
        }

        if (this.user && this.user.slot > slot) {
          return;
        }

        const account = this.program.account.user.coder.accounts.decode(
          "user",
          buffer
        );
        this.user = { data: account, slot };
        console.log("UserAccount updated start =====================");
        this.printUser(this.user);
        console.log("UserAccount updated end   =====================");
      }
    );

    this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {});
  }

  async printUser(userData: DataAndSlot<UserAccount>): Promise<void> {
    let user = userData.data;
    console.log("user data", user);
    console.log(`Next Order ID: ${user.nextOrderId.toString()}`);
    console.log(`Next Liquidation ID: ${user.nextLiquidationId.toString()}`);
    console.log(`Hold: ${user.hold.toString()}`);
    console.log(`Key: ${user.key.toBase58()}`);
    console.log(`Authority: ${user.authority.toBase58()}`);

    console.log("Tokens:");
    user.tokens.forEach((token, index) => {
      if (isEqual(token.userTokenStatus, UserTokenStatus.INIT)) {
        return;
      }
      console.log(`  Token ${index + 1}:`);
      console.log(`    Amount: ${token.amount.toString()}`);
      console.log(`    Used Amount: ${token.usedAmount.toString()}`);
      console.log(`    Liability Amount: ${token.liabilityAmount.toString()}`);
      console.log(`    Token Mint Key: ${token.tokenMintKey.toString()}`);
      console.log(
        `    User Token Account Key: ${token.userTokenAccountKey.toString()}`
      );
    });

    console.log("Stakes:");
    user.stakes.forEach((stake, index) => {
      if (isEqual(stake.userStakeStatus, UserStakeStatus.INIT)) {
        return;
      }
      console.log(`  Stake ${index + 1}:`);
      console.log(`    Staked Share: ${stake.stakedShare.toString()}`);
      console.log(`    User Rewards:`);
      console.log(
        `      Reward Amount: ${stake.userRewards.realisedRewardsTokenAmount.toString()}`
      );
      console.log(
        `      Reward Mint Key: ${stake.userRewards.openRewardsPerStakeToken.toString()}`
      );
      console.log(`    Pool Key: ${stake.poolKey.toString()}`);
    });

    console.log("Positions:");
    user.positions.forEach((position, index) => {
      if (isEqual(position.status, PositionStatus.INIT)) {
        return;
      }
      console.log(`  Position ${index + 1}:`);
      console.log(`    Position Size: ${position.positionSize.toString()}`);
      console.log(`    Entry Price: ${position.entryPrice.toString()}`);
      console.log(`    Initial Margin: ${position.initialMargin.toString()}`);
      console.log(
        `    Initial Margin USD: ${position.initialMarginUsd.toString()}`
      );
      console.log(
        `    Initial Margin USD From Portfolio: ${position.initialMarginUsdFromPortfolio.toString()}`
      );
      console.log(`    MM USD: ${position.mmUsd.toString()}`);
      console.log(
        `    Hold Pool Amount: ${position.holdPoolAmount.toString()}`
      );
      console.log(`    Open Fee: ${position.openFee.toString()}`);
      console.log(`    Open Fee In USD: ${position.openFeeInUsd.toString()}`);
      console.log(
        `    Realized Borrowing Fee: ${position.realizedBorrowingFee.toString()}`
      );
      console.log(
        `    Realized Borrowing Fee In USD: ${position.realizedBorrowingFeeInUsd.toString()}`
      );
      console.log(
        `    Open Borrowing Fee Per Token: ${position.openBorrowingFeePerToken.toString()}`
      );
      console.log(
        `    Realized Funding Fee: ${position.realizedFundingFee.toString()}`
      );
      console.log(
        `    Realized Funding Fee In USD: ${position.realizedFundingFeeInUsd.toString()}`
      );
      console.log(
        `    Open Funding Fee Amount Per Size: ${position.openFundingFeeAmountPerSize.toString()}`
      );
      console.log(`    Close Fee In USD: ${position.closeFeeInUsd.toString()}`);
      console.log(`    Realized PNL: ${position.realizedPnl.toString()}`);
      console.log(`    User Key: ${position.userKey.toString()}`);
      console.log(`    Symbol: ${position.symbol.join(",")}`);
      console.log(`    Updated At: ${position.updatedAt.toString()}`);
      console.log(`    Leverage: ${position.leverage}`);
      console.log(`    Is Long: ${position.isLong}`);
      console.log(`    Is Portfolio Margin: ${position.isPortfolioMargin}`);
      console.log(`    Status: ${position.status}`);
    });

    console.log("Orders:");
    user.orders.forEach((order, index) => {
      if (isEqual(order.status, OrderStatus.INIT)) {
        return;
      }
      console.log(`  Order ${index + 1}:`);
      console.log(`    Order Margin: ${order.orderMargin.toString()}`);
      console.log(`    Order Size: ${order.orderSize.toString()}`);
      console.log(`    Trigger Price: ${order.triggerPrice.toString()}`);
      console.log(`    Acceptable Price: ${order.acceptablePrice.toString()}`);
      console.log(`    Created At: ${order.createdAt.toString()}`);
      console.log(`    Order ID: ${order.orderId.toString()}`);
      console.log(`    Margin Mint Key: ${order.marginMintKey.toString()}`);
      console.log(`    Authority: ${order.authority.toString()}`);
      console.log(`    Symbol: ${order.symbol.join(",")}`);
      console.log(`    Leverage: ${order.leverage}`);
      console.log(`    Order Side: ${order.orderSide}`);
      console.log(`    Position Side: ${order.positionSide}`);
      console.log(`    Order Type: ${order.orderType}`);
      console.log(`    Stop Type: ${order.stopType}`);
      console.log(`    Status: ${order.status}`);
      console.log(`    Is Portfolio Margin: ${order.isPortfolioMargin}`);
    });
  }

  async fetchIfUnloaded(): Promise<void> {
    if (this.user === undefined) {
      await this.fetch();
    }
  }

  async fetch(): Promise<void> {
    try {
      const dataAndContext = await this.program.account.user.fetchAndContext(
        this.userAccountPublicKey,
        this.accountLoader.commitment
      );
      if (dataAndContext.context.slot > (this.user?.slot ?? 0)) {
        this.user = {
          data: dataAndContext.data as any as UserAccount,
          slot: dataAndContext.context.slot,
        };
      }
    } catch (e: any) {
      console.log(
        `PollingUserAccountSubscriber.fetch() UserAccount does not exist: ${e.message}`
      );
    }
  }

  doesAccountExist(): boolean {
    return this.user !== undefined;
  }

  async unsubscribe(): Promise<void> {
    if (!this.isSubscribed || !this.callbackId) {
      return;
    }

    this.accountLoader.removeAccount(
      this.userAccountPublicKey,
      this.callbackId
    );
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

  public getAccountAndSlot(): DataAndSlot<UserAccount> {
    if (!this.doesAccountExist() || !this.user) {
      throw new Error(
        "You must call `subscribe` or `fetch` before using this function"
      );
    }
    return this.user;
  }

  public updateData(userAccount: UserAccount, slot: number): void {
    if (!this.user || this.user.slot < slot) {
      this.user = { data: userAccount, slot };
      /*
            this.eventEmitter.emit('userAccountUpdate', userAccount);
            this.eventEmitter.emit('update');*/
    }
  }
}
