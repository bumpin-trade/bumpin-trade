import { AccountSubscriber, DataAndSlot } from "./types";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import {
  OrderStatusAccount,
  PositionStatusAccount,
  UserAccount,
  UserStakeStatusAccount,
  UserTokenStatusAccount,
} from "../typedef";
import { BulkAccountLoader } from "./bulkAccountLoader";
import { BumpinTrade } from "../types/bumpin_trade";
// @ts-ignore
import { isEqual } from "lodash";
import { TradeTokenComponent } from "../componets/tradeToken";
import { PoolComponent } from "../componets/pool";
import { User } from "../beans/beans";

export class PollingUserAccountSubscriber implements AccountSubscriber<User> {
  isSubscribed: boolean;
  program: Program<BumpinTrade>;
  userAccountPublicKey: PublicKey;

  accountLoader: BulkAccountLoader;
  callbackId?: string;
  errorCallbackId?: string;

  user?: DataAndSlot<User>;
  tradeTokenComponent: TradeTokenComponent;
  poolComponent: PoolComponent;

  public constructor(
    program: Program<BumpinTrade>,
    userAccountPublicKey: PublicKey,
    accountLoader: BulkAccountLoader,
    tradeTokenComponent: TradeTokenComponent,
    poolComponent: PoolComponent
  ) {
    this.isSubscribed = false;
    this.program = program;
    this.accountLoader = accountLoader;
    this.userAccountPublicKey = userAccountPublicKey;
    this.tradeTokenComponent = tradeTokenComponent;
    this.poolComponent = poolComponent;
  }

  async subscribe(userAccount?: User): Promise<boolean> {
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
        this.user = { data: this.convert(account), slot };
      }
    );

    this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {});
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
          data: this.convert(dataAndContext.data as any as UserAccount),
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

  public getAccountAndSlot(): DataAndSlot<User> {
    if (!this.doesAccountExist() || !this.user) {
      throw new Error(
        "You must call `subscribe` or `fetch` before using this function"
      );
    }
    return this.user;
  }

  public updateData(userAccount: User, slot: number): void {
    if (!this.user || this.user.slot < slot) {
      this.user = { data: userAccount, slot };
    }
  }

  private convert(user: UserAccount): User {
    const tradeTokens = this.tradeTokenComponent.getTradeTokensSync();
    const pools = this.poolComponent.getPoolsSync();
    return new User(user, pools, tradeTokens);
  }
}
