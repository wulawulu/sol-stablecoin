import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stablecoin } from "../target/types/stablecoin";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  AuthorityType,
  TOKEN_2022_PROGRAM_ID,
  createSetAuthorityInstruction,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";

describe("stablecoin", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);

  const program = anchor.workspace.Stablecoin as Program<Stablecoin>;

  const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
  const SOL_PRICE_FEED_ID = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
  const solUsdPriceFeedAccount = pythSolanaReceiver
    .getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID)
    .toBase58();

  console.log(solUsdPriceFeedAccount);

  const priceUpdate = new anchor.web3.PublicKey(solUsdPriceFeedAccount);
  const [config] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId,
  );
  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId,
  );
  const [collateral] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("collateral"), wallet.publicKey.toBuffer()],
    program.programId
  );
  const [solAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("sol"), wallet.publicKey.toBuffer()],
    program.programId,
  );
  const tokenAccount = getAssociatedTokenAddressSync(
    mint,
    wallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  const tokenProgram = TOKEN_2022_PROGRAM_ID;
  const associatedTokenProgram = ASSOCIATED_TOKEN_PROGRAM_ID;
  const systemProgram = anchor.web3.SystemProgram.programId;

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initializeConfig()
      .accounts({
        config,
        mint,
        tokenProgram,
      })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Your transaction signature", tx);

    const setAuthorityIx = createSetAuthorityInstruction(
      mint,
      wallet.publicKey,
      AuthorityType.MintTokens,
      mint,
      [],
      TOKEN_2022_PROGRAM_ID,
    );

    await provider.sendAndConfirm(
      new anchor.web3.Transaction().add(setAuthorityIx),
      [],
      { skipPreflight: true, commitment: "confirmed" },
    );
  });

  it("Deposit Collateral and Mint USDS!", async () => {
    const amountCollateral = 1_000_000_000;
    const amountToMint = 1_000_000_000;
    const tx = await program.methods
      .depositAndMint(
        new anchor.BN(amountCollateral),
        new anchor.BN(amountToMint),
      )
      .accounts({
        config,
        collateral,
        solAccount,
        mint,
        tokenAccount,
        priceUpdate,
        tokenProgram,
        systemProgram,
        associatedTokenProgram,
      })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Your transaction signature", tx);
  });

  it("Redeem Collateral and Burn USDS!", async () => {
    const amountCollateral = 500_000_000;
    const amountToBurn = 500_000_000;
    const tx = await program.methods
      .redeemCollateralAndBurnTokens(
        new anchor.BN(amountCollateral),
        new anchor.BN(amountToBurn),
      )
      .accounts({
        config,
        collateral,
        solAccount,
        mint,
        tokenAccount,
        priceUpdate,
        tokenProgram,
        systemProgram,
      })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Your transaction signature", tx);
  });

  it("Update Config!", async () => {
    const tx = await program.methods
      .updateConfig(new anchor.BN(100))
      .accounts({ config })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Your transaction signature", tx);
  });

  it("Liquidate", async () => {
    const amountToBurn = 500_000_000;
    const tx = await program.methods
      .liquidate(
        new anchor.BN(amountToBurn),
      )
      .accounts({
        config,
        collateral,
        solAccount,
        mint,
        tokenAccount,
        priceUpdate,
        tokenProgram,
        systemProgram,
        associatedTokenProgram,
      })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Your transaction signature", tx);
  });

  it("Update Config!", async () => {
    const tx = await program.methods
      .updateConfig(new anchor.BN(1))
      .accounts({ config })
      .rpc({ skipPreflight: true, commitment: "confirmed" });
    console.log("Your transaction signature", tx);
  });
});
