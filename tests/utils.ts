import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MetaticketVaultEscrowProgramV1 } from "../target/types/metaticket_vault_escrow_program_v1";
import MetaticketVaultEscrowProgramV1IDL from "../target/idl/metaticket_vault_escrow_program_v1.json";
import {
  PublicKey,
  Transaction,
  Keypair,
  Signer,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

export const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);
export const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey(
  "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
);
