import { AccountSubscriber, DataAndSlot } from "./types";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { UserAccount } from "../types";
import { BulkAccountLoader } from "./bulkAccountLoader";
import { BumpinTrade } from "../types/bumpin_trade";

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
        console.log("UserAccount updated", this.user);
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
