import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BumpinTrade } from "../target/types/bumpin_trade";
import {Utils} from "./utils/utils";
describe("bumpin-trade", () => {
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.BumpinTrade as Program<BumpinTrade>;

    const utils = new Utils({programId: program.programId});

    it("State initialize", async () => {
        let admin = await utils.new_user();
        await utils.initialize_state(admin);
        const state= await program.account.state.fetch(utils.get_bump_state_pk()) ;
        console.log("State: ", state.fundingFeeBaseRate.toString());

    });
});
