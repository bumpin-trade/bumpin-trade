import { PublicKey } from "@solana/web3.js";
import { BulkAccountLoader } from "./bulkAccountLoader";

export class PollingPythAccountSubscriber {
  isSubscribed: boolean;
  poolPublicKey: PublicKey;

  accountLoader: BulkAccountLoader;
  callbackId?: string;
  errorCallbackId?: string;

  public constructor(
    priceDataAccountPublicKey: PublicKey,
    accountLoader: BulkAccountLoader
  ) {
    this.isSubscribed = false;
    this.accountLoader = accountLoader;
    this.poolPublicKey = priceDataAccountPublicKey;
  }

  public async subscribe(
    onData: (data: Buffer, slot: number) => void
  ): Promise<boolean> {
    if (this.isSubscribed) {
      return true;
    }
    this.callbackId = await this.accountLoader.addAccount(
      this.poolPublicKey,
      (buffer, slot: number) => {
        if (!buffer) {
          return;
        }
        onData(buffer, slot);
      }
    );

    this.errorCallbackId = this.accountLoader.addErrorCallbacks((error) => {
      console.error(error);
    });

    this.isSubscribed = true;
    return true;
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
}
