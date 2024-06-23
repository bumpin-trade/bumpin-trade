import {PublicKey} from '@solana/web3.js';
import {OracleClient} from "./oracles/types";
import {UserAccountSubscriber} from "./account/types";
import {UserAccount} from "./types";
import {PollingUserAccountSubscriber} from "./account/pollingUserAccountSubscriber";

export class User {
    oracleClient: OracleClient
    userAccountPublicKey: PublicKey
    userAccountSubscriber:UserAccountSubscriber<UserAccount>
    constructor(oracleClient: OracleClient, userAccountPublicKey: PublicKey) {
        this.oracleClient = oracleClient;
        this.userAccountPublicKey = userAccountPublicKey;
        this.userAccountSubscriber=new PollingUserAccountSubscriber();
    }
}