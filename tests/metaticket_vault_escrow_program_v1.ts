import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MetaticketVaultEscrowProgramV1 } from "../target/types/metaticket_vault_escrow_program_v1";
import { PublicKey, Commitment } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  getMint,
  getOrCreateAssociatedTokenAccount,
  mintToChecked,
  TOKEN_PROGRAM_ID,
  mintTo,
} from "@solana/spl-token";
import {Connection} from "@solana/web3.js";

import { 
  PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID
} from '@metaplex-foundation/mpl-token-metadata';



describe("Metaticket Vault Program V1", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  

  const program = anchor.workspace
    .MetaticketVaultEscrowProgramV1 as Program<MetaticketVaultEscrowProgramV1>;

  const commitment: Commitment = "confirmed";
  const connection = new Connection('https://api.devnet.solana.com', commitment)




  // CREATE METATICKET ROLES
  const metaticket_authority = anchor.web3.Keypair.generate();
  const metaticket_vault = anchor.web3.Keypair.generate();

  // CREATE USDC ROLES
  const mintUSDC = anchor.web3.Keypair.generate();
  const usdc_authority = anchor.web3.Keypair.generate();

  // CREATE AMOUNTS OF NFTS TO SEND TO VAULT
  // CREATE AMOUNT EXPECTED PER TICKET
  const send_amount = 2;

  // CREATE AMOUNT USDC TO SEND TO VAULT FOR NFTS
  const taker_amount_usdc_to_metaticket_per_ticket = 2;

  //TAKERS AUTHORITY
  const ticket_buyers_account_authority = anchor.web3.Keypair.generate();

  // SET UP NFT METADATA CONSTANTS
  const nftTitle = "MetaTicket Genesis Event"
  const nftSymbol = "METAGEN"
  const nftUri = "https://raw.githubusercontent.com/LoneStarDev/metaticket-vault-escrow-program-v1/main/uri.json"

  // SET UP MINT KEYPAIR
  const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();




  //FUNDING METATICKET PAYER ACCOUNT
  before("before call", async () => {
    //Airdrop 5 SOL to metaticket Auth
    const signature = await connection.requestAirdrop(
      metaticket_authority.publicKey,
      2000000000
    );
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      commitment
    );
  });


  // DETERMINE THE METATICKET MANAGER WITH THE PDA
  const managerSeeds = [
    Buffer.from("manager"),
    metaticket_authority.publicKey.toBuffer(),
  ];

  const [metaticket_manager, manager_bump] = PublicKey.findProgramAddressSync(
    managerSeeds,
    program.programId
  );
  console.log("This is the Metaticket Manager PDA", metaticket_manager);

  //SET ID to 1, which will be the value of the first vault created by MetaTicket
  let id = 1;
  console.log(id);






  // DETERMINE THE MINT AUTHORITY  PDA
  // THIS WILL BE THE AUTHORITY WE USE TO MINT TICKETS FOR EACH COLLECTION

  const mintAuthoritySeeds = [
    Buffer.from("mint_authority"),
  ];

  const [metaticket_mint_authority, mint_auth_bump] =
    PublicKey.findProgramAddressSync(mintAuthoritySeeds, program.programId);
  console.log(
    "This is the mint authority PDA address",
    metaticket_mint_authority
  );


  // DETERMINE THE VAULT KEY PDA
  const vaultSeeds = [
    Buffer.from("vault"),
    Buffer.from(new anchor.BN(id).toArrayLike(Buffer, "le", 8)),
  ];

  const [vault_key, vault_bump] = PublicKey.findProgramAddressSync(
    vaultSeeds,
    program.programId
  );
  console.log("This is the vault key PDA", vault_key);

  // DETERMINE THE VAULT AUTHORITY STATE PDA
  const vaultAuthoritySeeds = [Buffer.from("auth")];

  const [vault_authority, vault_authority_bump] =
    PublicKey.findProgramAddressSync(vaultAuthoritySeeds, program.programId);
  console.log("This is the vault authority PDA", vault_authority);






  it("Initialized the MetaTicket Manager!", async () => {
    try {
      const tx = await program.methods
        .initializeMetaticketManager()
        .accountsStrict({
          metaticketAuthority: metaticket_authority.publicKey,
          metaticketManager: metaticket_manager,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([metaticket_authority])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("Error while creating a Manager Account:", error);
    }
  });

  it("Initialized the Mint Authority!", async () => {
    let id = 1;

    try {
      const tx = await program.methods
        .initializeMintingAuthority(new anchor.BN(id))
        .accounts({
          metaticketAuthority: metaticket_authority.publicKey,
          metaticketManager: metaticket_manager,
          metaticketMintAuthority: metaticket_mint_authority,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([metaticket_authority])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("Error while creating a Mint Authority Account:", error);
    }
  });


  before("before call", async () => {
    //Airdrop 5 SOL to metaticket Auth
    const signature = await connection.requestAirdrop(
      ticket_buyers_account_authority.publicKey,
      5000000000
    );
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      commitment
    );
  });


  it("Create mock USDC SPL Token for Takers Account", async () => {
    let USDC = await createMint(
      connection,
      ticket_buyers_account_authority,
      usdc_authority.publicKey,
      usdc_authority.publicKey,
      6,
      mintUSDC,
      null,
      TOKEN_PROGRAM_ID
    );

    let test = await getMint(
      connection,
      mintUSDC.publicKey,
      null,
      TOKEN_PROGRAM_ID
    );
    console.log(test);

    let usdc_ticket_buyers_account = await getOrCreateAssociatedTokenAccount(
      connection,
      ticket_buyers_account_authority,
      mintUSDC.publicKey,
      ticket_buyers_account_authority.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    let mint_to_sig = await mintToChecked(
      connection,
      ticket_buyers_account_authority,
      mintUSDC.publicKey,
      usdc_ticket_buyers_account.address,
      usdc_authority,
      200e6,
      6,
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );

    console.log(mint_to_sig);
    console.log(USDC);

    mintTo(
      connection,
      ticket_buyers_account_authority,
      USDC,
      ticket_buyers_account_authority.publicKey,
      usdc_authority,
      20
    );
  });


  




  it("Create an NFT!", async () => {


    try{
      const metadataAddress = (anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mintKeypair.publicKey.toBuffer(),
          ],
          TOKEN_METADATA_PROGRAM_ID
      ))[0];
  
      const sx = await program.methods.createToken(
        nftTitle, nftSymbol, nftUri
      )
        .accounts({
          metaticketAuthority: metaticket_authority.publicKey,
          metadataAccount: metadataAddress,
          mintAccount: mintKeypair.publicKey,
          metaticketMintAuthority: metaticket_mint_authority,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        })
        .signers([mintKeypair, metaticket_authority])
        .rpc();
  
      console.log("Success!");
          console.log(`   Mint Address: ${mintKeypair.publicKey}`);
          console.log(`   Tx Signature: ${sx}`);

    } catch (err){
      console.log(err)
    }
  });

});
