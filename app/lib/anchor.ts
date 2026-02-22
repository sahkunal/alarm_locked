"use client";

import * as anchor from "@coral-xyz/anchor";
import idl from "../idl/alarm_locked.json";
import { Connection, PublicKey } from "@solana/web3.js";

const programId = new PublicKey(
  "8SKpWVeyrbDTJpGztuEVK399jHSx5n2HuAGSAjgHKGQo"
);

export function getProgram(wallet: any) {
  const connection = new Connection("https://api.devnet.solana.com");

  const provider = new anchor.AnchorProvider(
    connection,
    wallet,
    anchor.AnchorProvider.defaultOptions()
  );

  return new anchor.Program(idl as any, provider);
}