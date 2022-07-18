import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OrcaLping } from "../target/types/orca_lping";

describe("orca_lping", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OrcaLping as Program<OrcaLping>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
