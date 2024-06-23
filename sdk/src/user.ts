import {PublicKey} from '@solana/web3.js';
import {OracleClient} from "./oracles/types";
import {AccountSubscriber} from "./account/types";
import {State, UserAccount} from "./types";
import {PollingUserAccountSubscriber} from "./account/pollingUserAccountSubscriber";
import {BumpinClientConfig} from "./bumpinClientConfig";
import {BN} from "@coral-xyz/anchor";

export class User {
    oracleClient: OracleClient
    userAccountPublicKey: PublicKey
    userAccountSubscriber: AccountSubscriber<UserAccount>
    state: State

    constructor(clientConfig: BumpinClientConfig) {
        this.oracleClient = clientConfig.oracleClient;
        this.userAccountPublicKey = clientConfig.userAccountPublicKey;
        this.state = clientConfig.state;
        this.userAccountSubscriber = new PollingUserAccountSubscriber(clientConfig.program,
            clientConfig.userAccountPublicKey, clientConfig.bulkAccountLoader);
    }

    public accountExist(): boolean {
        let userAccountAndSlot = this.userAccountSubscriber.getUserAccountAndSlot();
        return userAccountAndSlot !== undefined;
    }

    public async getUserAvailableValue(): Promise<BN> {
        if (this.accountExist()) {
            let userAccountAndSlot = this.userAccountSubscriber.getUserAccountAndSlot();
            for (const userToken of userAccountAndSlot.data.user_tokens) {

            }
        }
        return new BN(0);
    }
}