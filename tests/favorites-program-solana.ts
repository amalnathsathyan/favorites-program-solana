import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FavoritesProgramSolana } from "../target/types/favorites_program_solana";

describe("favorites-program-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FavoritesProgramSolana as Program<FavoritesProgramSolana>;
  const user = anchor.web3.Keypair.generate();

  it("It set's the favorites!", async () => {
    // Add your test here.
    const number = new anchor.BN('25');
    const color = "orange"
    const hobbies = ["reading","skipping"]
    const tx = await program.methods.setFavorites(
      number,
      color,
      hobbies
    ).rpc();
    console.log("Your transaction signature", tx);
  });
});
