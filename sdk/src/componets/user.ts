import {PublicKey} from '@solana/web3.js';
import {UserAccount} from "../types";
import {BulkAccountLoader} from "../account/bulkAccountLoader";
import {Program} from "@coral-xyz/anchor";
import {BumpinUtils} from "../utils/utils";
import {BumpinTrade} from "../types/bumpin_trade";
// import {tokenToUsd} from "./utils/cal_utils";
import {Component} from "./componet";
import {PollingStateAccountSubscriber} from "../account/pollingStateAccountSubscriber";
import {PollingUserAccountSubscriber} from "../account/pollingUserAccountSubscriber";
import {BumpinAccountNotFound, BumpinSubscriptionFailed} from "../errors";
import {DataAndSlot} from "../account/types";

export class UserComponent extends Component {
    publicKey: PublicKey;
    program: Program<BumpinTrade>;
    userAccountSubscriber: PollingUserAccountSubscriber;

    constructor(publicKey: PublicKey, bulkAccountLoader: BulkAccountLoader, stateSubscriber: PollingStateAccountSubscriber, program: Program<BumpinTrade>) {
        super(stateSubscriber, program);
        this.publicKey = publicKey;
        this.program = program;
        const [pda, _] = BumpinUtils.getPdaSync(this.program, [Buffer.from("user"), this.publicKey.toBuffer()]);
        this.userAccountSubscriber = new PollingUserAccountSubscriber(this.program, pda, bulkAccountLoader);
    }

    public async subscribe() {
        await this.userAccountSubscriber.subscribe();
    }

    public async unsubscribe() {
        await this.userAccountSubscriber.unsubscribe();
    }
    public async getUser(sync: boolean = false): Promise<UserAccount> {
        let userWithSlot = await this.getUserWithSlot(sync);
        return userWithSlot.data;
    }


    public async getUserWithSlot(sync: boolean = false): Promise<DataAndSlot<UserAccount>> {
        if (!this.userAccountSubscriber || !this.userAccountSubscriber.isSubscribed) {
            throw new BumpinSubscriptionFailed("User")
        }
        if (sync) {
            await this.userAccountSubscriber.fetch();
        }
        let userAccount = this.userAccountSubscriber.getAccountAndSlot();
        if (!userAccount) {
            throw new BumpinAccountNotFound("User")
        }
        return userAccount;
    }
}