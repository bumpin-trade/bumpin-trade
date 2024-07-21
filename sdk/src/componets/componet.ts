import { DataAndSlot } from "../account/types";
import { StateAccount } from "../typedef";
import { Program } from "@coral-xyz/anchor";
import { BumpinTrade } from "../types/bumpin_trade";
import { PollingStateAccountSubscriber } from "../account/pollingStateAccountSubscriber";
import {
  BumpinAccountNotFound,
  BumpinClientInternalError,
  BumpinSubscriptionFailed,
} from "../errors";

export abstract class Component {
  stateSubscriber: PollingStateAccountSubscriber;
  program: Program<BumpinTrade>;

  constructor(
    stateSubscriber: PollingStateAccountSubscriber,
    program: Program<BumpinTrade>
  ) {
    if (!stateSubscriber.isSubscribed) {
      throw new BumpinClientInternalError("State not subscribed");
    }
    this.stateSubscriber = stateSubscriber;
    this.program = program;
  }

  protected getStateWithSlotSync(): DataAndSlot<StateAccount> {
    let stateAccount = this.stateSubscriber.state;
    if (!stateAccount) {
      throw new BumpinAccountNotFound("State");
    }
    return stateAccount;
  }

  protected getStateSync(): StateAccount {
    let stateAccount = this.stateSubscriber.getAccountAndSlot();
    return stateAccount.data;
  }

  protected async getState(sync: boolean = false): Promise<StateAccount> {
    let stateWithSlot = await this.getStateWithSlot(sync);
    return stateWithSlot.data;
  }

  protected async getStateWithSlot(
    sync: boolean = false
  ): Promise<DataAndSlot<StateAccount>> {
    if (!this.stateSubscriber || !this.stateSubscriber.isSubscribed) {
      throw new BumpinSubscriptionFailed("State");
    }
    if (sync) {
      await this.stateSubscriber.fetch();
    }
    let stateAccount = this.stateSubscriber.state;
    if (!stateAccount) {
      throw new BumpinAccountNotFound("State");
    }
    return stateAccount;
  }
}
