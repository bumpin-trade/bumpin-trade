import { AccountSubscriber, DataAndSlot } from "./types";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Market } from "../types";
import { BulkAccountLoader } from "./bulkAccountLoader";
import { BumpinTrade } from "../types/bumpin_trade";

export class PollingMarketAccountSubscriber
  implements AccountSubscriber<Market>
{
  isSubscribed: boolean;
  program: Program<BumpinTrade>;
  userAccountPublicKey: PublicKey;

  accountLoader: BulkAccountLoader;
  callbackId?: string;
  errorCallbackId?: string;

  market?: DataAndSlot<Market>;

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

  async subscribe(userAccount?: Market): Promise<boolean> {
    if (this.isSubscribed) {
      return true;
    }

    if (userAccount) {
      this.market = { data: userAccount, slot: 0 };
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

        if (this.market && this.market.slot > slot) {
          return;
        }

        const account = this.program.account.market.coder.accounts.decode(
          "market",
          buffer
        );
        this.market = { data: account, slot };
        console.log("MarketAccount updated start =====================");
        this.printMarket(this.market);
        console.log("MarketAccount updated end   =====================");

      }
    );

    this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {});
  }

  async printMarket(marketData:DataAndSlot<Market>):Promise<void>{
    let market = marketData.data;
    console.log(`Pool Key: ${market.poolKey.toString()}`);
    console.log(`Pool Mint Key: ${market.poolMintKey.toString()}`);
    console.log(`Index Mint Oracle: ${market.indexMintOracle.toString()}`);
    console.log(`Stable Pool Key: ${market.stablePoolKey.toString()}`);
    console.log(`Stable Pool Mint Key: ${market.stablePoolMintKey.toString()}`);
    console.log(`Index: ${market.index}`);
    console.log(`Symbol: ${market.symbol.join(', ')}`);

    console.log('Long Open Interest:');
    console.log(`  Open Interest: ${market.longOpenInterest.openInterest.toString()}`);
    console.log(`  Entry Price: ${market.longOpenInterest.entryPrice.toString()}`);

    console.log('Short Open Interest:');
    console.log(`  Open Interest: ${market.shortOpenInterest.openInterest.toString()}`);
    console.log(`  Entry Price: ${market.shortOpenInterest.entryPrice.toString()}`);

    console.log('Funding Fee:');
    console.log(`  Long Funding Fee Amount Per Size: ${market.fundingFee.longFundingFeeAmountPerSize.toString()}`);
    console.log(`  Short Funding Fee Amount Per Size: ${market.fundingFee.shortFundingFeeAmountPerSize.toString()}`);
    console.log(`  Total Long Funding Fee: ${market.fundingFee.totalLongFundingFee.toString()}`);
    console.log(`  Total Short Funding Fee: ${market.fundingFee.totalShortFundingFee.toString()}`);
    console.log(`  Long Funding Fee Rate: ${market.fundingFee.longFundingFeeRate.toString()}`);
    console.log(`  Short Funding Fee Rate: ${market.fundingFee.shortFundingFeeRate.toString()}`);
    console.log(`  Updated At: ${market.fundingFee.updatedAt.toString()}`);

    console.log('Config:');
    console.log(`  Tick Size: ${market.config.tickSize.toString()}`);
    console.log(`  Open Fee Rate: ${market.config.openFeeRate.toString()}`);
    console.log(`  Close Fee Rate: ${market.config.closeFeeRate.toString()}`);
    console.log(`  Maximum Long Open Interest Cap: ${market.config.maximumLongOpenInterestCap.toString()}`);
    console.log(`  Maximum Short Open Interest Cap: ${market.config.maximumShortOpenInterestCap.toString()}`);
    console.log(`  Long Short Ratio Limit: ${market.config.longShortRatioLimit.toString()}`);
    console.log(`  Long Short OI Bottom Limit: ${market.config.longShortOiBottomLimit.toString()}`);
    console.log(`  Maximum Leverage: ${market.config.maximumLeverage}`);
    console.log(`  Minimum Leverage: ${market.config.minimumLeverage}`);
  }

  async fetchIfUnloaded(): Promise<void> {
    if (this.market === undefined) {
      await this.fetch();
    }
  }

  async fetch(): Promise<void> {
    try {
      const dataAndContext = await this.program.account.market.fetchAndContext(
        this.userAccountPublicKey,
        this.accountLoader.commitment
      );
      if (dataAndContext.context.slot > (this.market?.slot ?? 0)) {
        this.market = {
          data: dataAndContext.data as any as Market,
          slot: dataAndContext.context.slot,
        };
      }
    } catch (e: any) {
      console.log(
        `PollingMarketAccountSubscriber.fetch() UserAccount does not exist: ${e.message}`
      );
    }
  }

  doesAccountExist(): boolean {
    return this.market !== undefined;
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

  public getAccountAndSlot(): DataAndSlot<Market> {
    if (!this.doesAccountExist() || !this.market) {
      throw new Error(
        "You must call `subscribe` or `fetch` before using this function"
      );
    }
    return this.market;
  }

  public updateData(userAccount: Market, slot: number): void {
    if (!this.market || this.market.slot < slot) {
      this.market = { data: userAccount, slot };
      /*
            this.eventEmitter.emit('userAccountUpdate', userAccount);
            this.eventEmitter.emit('update');*/
    }
  }
}
