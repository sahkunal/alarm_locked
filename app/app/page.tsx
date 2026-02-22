"use client";

import { useEffect, useState } from "react";
import { motion } from "framer-motion";
import toast, { Toaster } from "react-hot-toast";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { getProgram } from "../lib/anchor";

export default function Home() {
  const wallet = useWallet();
  const [mounted, setMounted] = useState(false);

useEffect(() => {
  setMounted(true);
}, []);
const owner = wallet.publicKey;

  const [balance, setBalance] = useState<number>(0);
  const [unlockTime, setUnlockTime] = useState<number | null>(null);
  const [countdown, setCountdown] = useState<string>("");

  const getPdas = () => {
    if (!owner) return null;

    const program = getProgram(wallet);

    const [vaultState] = PublicKey.findProgramAddressSync(
      [Buffer.from("state"), owner.toBuffer()],
      program.programId
    );

    const [vault] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultState.toBuffer()],
      program.programId
    );

    return { program, vaultState, vault };
  };

  const refreshData = async () => {
    try {
      const data = getPdas();
      if (!data) return;

      const { program, vaultState, vault } = data;

      const vaultBal = await program.provider.connection.getBalance(vault);
      setBalance(vaultBal / LAMPORTS_PER_SOL);

      try {
       const state = await (program.account as any).vaultState.fetch(vaultState);    
        setUnlockTime(state.unlockTime.toNumber());
      } catch {}
    } catch {}
  };

  useEffect(() => {
    refreshData();
  }, [wallet.connected]);

  useEffect(() => {
    if (!unlockTime) return;

    const interval = setInterval(() => {
      const now = Math.floor(Date.now() / 1000);
      const diff = unlockTime - now;

      if (diff <= 0) {
        setCountdown("Unlocked ✅");
      } else {
        setCountdown(`${diff}s remaining`);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, [unlockTime]);

  const runTx = async (fn: () => Promise<any>, label: string) => {
    const id = toast.loading(`${label}...`);
    try {
      await fn();
      toast.success(`${label} success`, { id });
      refreshData();
    } catch (err: any) {
      toast.error(err.message ?? "Transaction failed", { id });
    }
  };

  const initialize = async () => {
    const data = getPdas();
    if (!data) return;
    const { program, vaultState, vault } = data;

    const unlock = new anchor.BN(Math.floor(Date.now() / 1000) + 60);

    await runTx(
      () =>
        program.methods.initialize(unlock).accounts({
          owner,
          vaultState,
          vault,
          systemProgram: SystemProgram.programId,
        }as any).rpc(),
      "Initialize Vault"
    );
  };

  const deposit = async () => {
    const data = getPdas();
    if (!data) return;
    const { program, vaultState, vault } = data;

    await runTx(
      () =>
        program.methods
          .deposit(new anchor.BN(0.1 * LAMPORTS_PER_SOL))
          .accounts({
            owner,
            vaultState,
            vault,
            systemProgram: SystemProgram.programId,
          }as any)
          .rpc(),
      "Deposit"
    );
  };

  const withdraw = async () => {
    const data = getPdas();
    if (!data) return;
    const { program, vaultState, vault } = data;

    await runTx(
      () =>
        program.methods.withdraw().accounts({
          owner,
          vaultState,
          vault,
          systemProgram: SystemProgram.programId,
        }as any).rpc(),
      "Withdraw"
    );
  };

  const closeVault = async () => {
    const data = getPdas();
    if (!data) return;
    const { program, vaultState, vault } = data;

    await runTx(
      () =>
        program.methods.closeVault().accounts({
          owner,
          vaultState,
          vault,
        }as any).rpc(),
      "Close Vault"
    );
  };

  return (
    <main className="min-h-screen bg-gradient-to-br from-black via-neutral-950 to-neutral-900 flex items-center justify-center p-10">
      <Toaster position="top-right" />

      <motion.div
        initial={{ opacity: 0, y: 40 }}
        animate={{ opacity: 1, y: 0 }}
        className="w-full max-w-xl backdrop-blur-xl bg-white/5 border border-white/10 rounded-3xl p-10 shadow-2xl"
      >
        <h1 className="text-4xl font-bold text-center mb-6">
          ⏰ Alarm Locked Vault
        </h1>

        <div className="flex justify-center mb-6">
          {mounted && <WalletMultiButton />}
        </div>

        {wallet.connected && (
          <>
            <div className="grid grid-cols-2 gap-4 mb-6 text-center">
              <div className="bg-white/5 p-4 rounded-xl">
                <p className="text-sm opacity-70">Vault Balance</p>
                <p className="text-xl font-semibold">{balance} SOL</p>
              </div>

              <div className="bg-white/5 p-4 rounded-xl">
                <p className="text-sm opacity-70">Unlock Status</p>
                <p className="text-xl font-semibold">{countdown}</p>
              </div>
            </div>

            <div className="grid gap-4">
              <button
                onClick={initialize}
                className="bg-purple-600 hover:bg-purple-700 p-3 rounded-xl"
              >
                Initialize Vault
              </button>

              <button
                onClick={deposit}
                className="bg-blue-600 hover:bg-blue-700 p-3 rounded-xl"
              >
                Deposit 0.1 SOL
              </button>

              <button
                onClick={withdraw}
                className="bg-green-600 hover:bg-green-700 p-3 rounded-xl"
              >
                Withdraw
              </button>

              <button
                onClick={closeVault}
                className="bg-red-600 hover:bg-red-700 p-3 rounded-xl"
              >
                Close Vault
              </button>
            </div>
          </>
        )}
      </motion.div>
    </main>
  );
}