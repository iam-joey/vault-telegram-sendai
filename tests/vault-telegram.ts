import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VaultTelegram } from "../target/types/vault_telegram";
import { randomBytes } from "crypto";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("vault-telegram", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const program = anchor.workspace.VaultTelegram as Program<VaultTelegram>;
  let [creator, user1, user2, user3] = [
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
  ];
  const seed = new anchor.BN(randomBytes(8));

  const [betStatePda, betStatePdaBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("bet_state"),
        creator.publicKey.toBuffer(),
        seed.toBuffer().reverse(),
      ],
      program.programId
    );
  const [vault, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), betStatePda.toBuffer()],
    program.programId
  );
  it("airdrop sol", async () => {
    const tx1 = await connection.requestAirdrop(
      creator.publicKey,
      LAMPORTS_PER_SOL * 5
    );
    await connection.confirmTransaction(tx1);
    const tx2 = await connection.requestAirdrop(
      user1.publicKey,
      LAMPORTS_PER_SOL * 5
    );
    await connection.confirmTransaction(tx2);
    const tx3 = await connection.requestAirdrop(
      user2.publicKey,
      LAMPORTS_PER_SOL * 5
    );
    await connection.confirmTransaction(tx3);
    const tx4 = await connection.requestAirdrop(
      user3.publicKey,
      LAMPORTS_PER_SOL * 5
    );
    await connection.confirmTransaction(tx4);
    console.log("Airdrop done");
    // const balance = await connection.getBalance(user3.publicKey);
    // console.log("Balance of user3", balance);
  });
  it("Create a bet!", async () => {
    const tx = await program.methods
      .create(
        seed,
        new anchor.BN(4),
        new anchor.BN(1000000000),
        new anchor.BN(1000000000)
      )
      .accountsPartial({
        creator: creator.publicKey,
        betState: betStatePda,
        vaultPool: vault,
      })
      .signers([creator])
      .rpc();

    console.log("Your transaction signature", tx);
    console.log("Bet created");
  });
  it("Join a bet! creator joing", async () => {
    const tx = await program.methods
      .join(seed)
      .accountsPartial({
        maker: creator.publicKey,
        user: creator.publicKey,
        betState: betStatePda,
        vaultPool: vault,
      })
      .signers([creator])
      .rpc();

    console.log("creator join the bet", tx);
  });
  it("Join a bet! user 1", async () => {
    const tx = await program.methods
      .join(seed)
      .accountsPartial({
        maker: creator.publicKey,
        user: user1.publicKey,
        betState: betStatePda,
        vaultPool: vault,
      })
      .signers([user1])
      .rpc();
    console.log("user 1 join the bet", tx);
  });
  it("Join a bet! user 2", async () => {
    const tx = await program.methods
      .join(seed)
      .accountsPartial({
        maker: creator.publicKey,
        user: user2.publicKey,
        betState: betStatePda,
        vaultPool: vault,
      })
      .signers([user2])
      .rpc();

    console.log("user 2 join the bet", tx);
  });
  it("Join a bet! user 3", async () => {
    const tx = await program.methods
      .join(seed)
      .accountsPartial({
        maker: creator.publicKey,
        user: user3.publicKey,
        betState: betStatePda,
        vaultPool: vault,
      })
      .signers([user3])
      .rpc();
    // console.log("user 3 join the bet", tx);
    // const balanceuser3 = await connection.getBalance(user3.publicKey);
    // console.log("Balance of user3", balanceuser3);
    // const bet = await program.account.betState.fetch(vaultPda);
    // console.log("Bet state", bet);
    const balance = await connection.getBalance(vault);
    console.log("Vault balance", balance);
  });
  it("Resolve bet", async () => {
    const tx = await program.methods
      .resolve(seed)
      .accountsPartial({
        betState: betStatePda,
        vaultPool: vault,
        creator: creator.publicKey,
        winner: user3.publicKey,
      })
      .signers([creator])
      .rpc();
    console.log("Bet resolved", tx);
  });
  it("Check winner balance", async () => {
    const balance = await connection.getBalance(user3.publicKey);
    console.log("Winner balance", balance);
  });
});
