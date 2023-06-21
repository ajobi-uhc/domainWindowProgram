import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Fractions } from "../target/types/fractions";

describe("fractions", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.Provider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.Fractions as Program<Fractions>;

  it("Is initialized!", async () => {
    // Add your test here.

    let admin = anchor.web3.Keypair.generate();
    let fromAirdropSignature = await provider.connection.requestAirdrop(
      admin.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );

    await provider.connection.confirmTransaction(fromAirdropSignature);
    console.log("admin pubkey", admin.publicKey.toBase58());

    let userPrizeMintAccount = await Token.createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      10,
      TOKEN_PROGRAM_ID
    );

    let userPrizetokenAccount =
      await userPrizeMintAccount.getOrCreateAssociatedAccountInfo(
        admin.publicKey
      );
    console.log(
      "pool prize token account: owner",
      userPrizetokenAccount.owner.toBase58()
    );

    let amount = new anchor.BN(5);

    await userPrizeMintAccount.mintTo(
      userPrizetokenAccount.address,
      admin,
      [],
      amount.toNumber()
    );

    let phase_number = new anchor.BN(1);
    let [ambience_phase_info, initNonce] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("init")],
        program.programId
      );

    let poolInfo = anchor.web3.Keypair.generate();

    let [pool_signer, component_nonce] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("signer")],
        program.programId
      );

    let componentMint = await Token.createMint(
      provider.connection,
      admin,
      pool_signer,
      null,
      5,
      TOKEN_PROGRAM_ID
    );
    console.log("about to fail");

    let poolComponentHolder = await componentMint.createAccount(pool_signer);

    let poolPrizeHolder = await userPrizeMintAccount.createAccount(pool_signer);

    console.log("pool info pubkey: ", poolInfo.publicKey.toBase58());
    console.log("pool signer pubkey:", pool_signer.toBase58());

    let start_ambience = new anchor.BN(30);
    let pool_cap = new anchor.BN(50);
    let num_total_prize_tokens = new anchor.BN(3);

    const tx = await program.rpc.initialize(
      phase_number,
      initNonce,
      component_nonce,
      num_total_prize_tokens,
      start_ambience,
      pool_cap,
      {
        accounts: {
          poolSigner: pool_signer,
          user: admin.publicKey,
          ambiencePhase: poolInfo.publicKey,
          poolPrize: poolPrizeHolder,
          poolComponentMint: componentMint.publicKey,
          poolComponentAccount: poolComponentHolder,
          userPrizeToTransfer: userPrizetokenAccount.address,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [admin, poolInfo],
      }
    );
    console.log("Your transaction signature", tx);
    let pool = await program.account.ambiencePool.fetch(poolInfo.publicKey);
    console.log(pool);
  });
});
