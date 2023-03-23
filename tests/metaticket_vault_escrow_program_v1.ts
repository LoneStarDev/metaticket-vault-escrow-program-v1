import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {
  createInitializeMintInstruction,
  MintLayout,
  TOKEN_PROGRAM_ID,
  transfer,
} from "@solana/spl-token";
import { MetaticketVaultEscrowProgramV1 } from "../target/types/metaticket_vault_escrow_program_v1";
import { PublicKey, Transaction, Signer, Commitment } from "@solana/web3.js";
import MetaticketVaultEscrowProgramV1IDL from "../target/idl/metaticket_vault_escrow_program_v1.json";

const { SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } = anchor.web3;

describe("nft-vault", () => {
  // Configure the client to use the local cluster.23
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  const program = anchor.workspace
    .MetaticketVaultEscrowProgramV1 as Program<MetaticketVaultEscrowProgramV1>;

  const payer = new PublicKey("sRGTjNgc4qLFKj2CAAHezaNjgLnrTrAdGrdurpinjEN");
  const metaticket_authority = Keypair.generate();

  before("funding setup", async () => {
    const signature = await provider.connection.requestAirdrop(
      metaticket_authority.publicKey,
      6000000000
    );
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...latestBlockhash,
    });

    // 2. Fund main roles: initializer and taker
    const fundingTx = new Transaction();
    fundingTx.add(
      SystemProgram.transfer({
        fromPubkey: payer,
        toPubkey: metaticket_authority.publicKey,
        lamports: 500000000,
      })
    );
  });

  it("Initialized the Vault", async () => {
    const getNftVaultPda = async (
      programId: PublicKey = new PublicKey(
        MetaticketVaultEscrowProgramV1IDL.metadata.address
      )
    ) => {
      return anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vault")],
        programId
      );
    };

    const [metaticketNFTVault, _] = await getNftVaultPda();
    console.log(metaticketNFTVault);

    let { lastValidBlockHeight, blockhash } =
      await provider.connection.getLatestBlockhash("finalized");

    try {
      const instruction = await program.methods
        .initialize()
        .accountsStrict({
          metaticketAuthority: metaticket_authority.publicKey,
          metaticketNftVault: metaticketNFTVault,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([metaticket_authority])
        .rpc();

      await provider.connection.confirmTransaction({
        signature: instruction,
        blockhash: blockhash,
        lastValidBlockHeight: lastValidBlockHeight,
      });
    } catch (error) {
      console.error("Error while creating the nft vault account:", error);
    } finally {
      let nftVault;
      try {
        nftVault = await program.account.vault.fetch(metaticketNFTVault);
      } catch (error) {
        console.error("Error while fetching the vault account:", error);
      }
    }
  });
});
