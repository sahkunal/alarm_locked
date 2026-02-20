import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AlarmLocked } from "../target/types/alarm_locked";
import { assert } from "chai";
import { PublicKey, LAMPORTS_PER_SOL, SystemProgram } from "@solana/web3.js";

describe("alarm_locked", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.alarmLocked as Program<AlarmLocked>;
  const owner = provider.wallet.publicKey;

  const [vaultStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), owner.toBuffer()],
    program.programId
  );

  const [vaultPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultStatePda.toBuffer()],
    program.programId
  );

  async function warpTime(seconds: number) {
    console.log(`Warping time by ${seconds} seconds...`);
    await new Promise(resolve => setTimeout(resolve, 1000));
  }
 
  it("Initializes a vault with future unlock time", async () => {
    const currentTime = Math.floor(Date.now() / 1000);
    const unlockTime = currentTime + 5; 
    
    try {
      const tx = await program.methods
        .initialize(new anchor.BN(unlockTime))
        .accounts({
          owner: owner,
          vaultState: vaultStatePda,
          vault: vaultPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();

      console.log("Initialize transaction signature:", tx);

      const vaultState = await program.account.vaultState.fetch(vaultStatePda);
      
      assert.ok(vaultState.owner.equals(owner));
      assert.equal(vaultState.unlockTime.toNumber(), unlockTime);
      assert.ok(vaultState.initialized);
      
      console.log(`Vault initialized. Unlock time: ${new Date(unlockTime * 1000).toLocaleString()}`);
    } catch (error) {
      console.error("Initialize failed:", error);
      throw error;
    }
  });

  it("Deposits SOL into the vault", async () => {
    const depositAmount = new anchor.BN(0.1 * LAMPORTS_PER_SOL); 
    
    try {
      const initialVaultBalance = await provider.connection.getBalance(vaultPda);
      const initialOwnerBalance = await provider.connection.getBalance(owner);

      const tx = await program.methods
        .deposit(depositAmount)
        .accounts({
          owner: owner,
          vaultState: vaultStatePda,
          vault: vaultPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();

      console.log("Deposit transaction signature:", tx);

      const finalVaultBalance = await provider.connection.getBalance(vaultPda);
      const finalOwnerBalance = await provider.connection.getBalance(owner);

      assert.equal(
        finalVaultBalance - initialVaultBalance,
        depositAmount.toNumber()
      );
      assert.ok(finalOwnerBalance < initialOwnerBalance);
      
      console.log(`Deposited ${depositAmount.toNumber() / LAMPORTS_PER_SOL} SOL to vault`);
    } catch (error) {
      console.error("Deposit failed:", error);
      throw error;
    }
  });

  it("Fails to withdraw before unlock time", async () => {
    try {
      await program.methods
        .withdraw()
        .accounts({
          owner: owner,
          vaultState: vaultStatePda,
          vault: vaultPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();
      
      assert.fail("Should have thrown an error");
    } catch (error) {
      assert.ok(error.toString().includes("VaultStillLocked"));
      console.log("✅ Success: Withdrawal prevented before unlock time");
    }
  });

  it("Withdraws after unlock time", async () => {
    const vaultState = await program.account.vaultState.fetch(vaultStatePda);
    const unlockTime = vaultState.unlockTime.toNumber();
    const currentTime = Math.floor(Date.now() / 1000);
    
    console.log(`Current time: ${currentTime}, Unlock time: ${unlockTime}`);
    
    if (currentTime < unlockTime) {
      const waitTime = (unlockTime - currentTime) * 1000; 
      console.log(`Waiting ${waitTime/1000} seconds for unlock time...`);
      await new Promise(resolve => setTimeout(resolve, waitTime + 1000)); 
    }

    const vaultBalanceBefore = await provider.connection.getBalance(vaultPda);
    const ownerBalanceBefore = await provider.connection.getBalance(owner);
    
    console.log(`Vault balance before: ${vaultBalanceBefore / LAMPORTS_PER_SOL} SOL`);
    console.log(`Owner balance before: ${ownerBalanceBefore / LAMPORTS_PER_SOL} SOL`);

    try {
      const tx = await program.methods
        .withdraw()
        .accounts({
          owner: owner,
          vaultState: vaultStatePda,
          vault: vaultPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();

      console.log("Withdraw transaction signature:", tx);

      const vaultBalanceAfter = await provider.connection.getBalance(vaultPda);
      const ownerBalanceAfter = await provider.connection.getBalance(owner);
      
      console.log(`Vault balance after: ${vaultBalanceAfter / LAMPORTS_PER_SOL} SOL`);
      console.log(`Owner balance after: ${ownerBalanceAfter / LAMPORTS_PER_SOL} SOL`);

      assert.equal(vaultBalanceAfter, 0, "Vault should be empty after withdrawal"); 
      assert.ok(ownerBalanceAfter > ownerBalanceBefore, "Owner balance should increase");
      
    } catch (error) {
      console.error("Withdraw failed:", error);
      throw error;
    }
  });

  it("Closes the vault", async () => {
    const vaultBalance = await provider.connection.getBalance(vaultPda);
    console.log(`Vault balance before closing: ${vaultBalance / LAMPORTS_PER_SOL} SOL`);
    
    if (vaultBalance > 0) {
      console.log("⚠️ Vault still has funds! Withdrawing before close...");
      
      await program.methods
        .withdraw()
        .accounts({
          owner: owner,
          vaultState: vaultStatePda,
          vault: vaultPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();
    }

    try {
      const tx = await program.methods
        .closeVault()
        .accounts({
          owner: owner,
          vaultState: vaultStatePda,
          vault: vaultPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();

      console.log("Close vault transaction signature:", tx);

      try {
        await program.account.vaultState.fetch(vaultStatePda);
        assert.fail("Vault state should be closed");
      } catch (error) {
        assert.ok(error.toString().includes("Account does not exist"));
        console.log("✅ Success: Vault state account closed");
      }
    } catch (error) {
      console.error("Close vault failed:", error);
      throw error;
    }
  });
});