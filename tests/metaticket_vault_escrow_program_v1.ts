import * as anchor from "@project-serum/anchor";
import { Program, web3, workspace } from "@project-serum/anchor";
import { MetaticketVaultEscrowProgramV1 } from "../target/types/metaticket_vault_escrow_program_v1";
import { PublicKey, Commitment, Connection,clusterApiUrl, Transaction} from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getMint, getOrCreateAssociatedTokenAccount, mintToChecked, TOKEN_PROGRAM_ID, mintTo, MINT_SIZE, getMinimumBalanceForRentExemptAccount, createInitializeAccount2Instruction, createInitializeMintInstruction, setAuthority, transfer } from "@solana/spl-token";
import { Metaplex, keypairIdentity, bundlrStorage, toMetaplexFile, toTokenAccount } from "@metaplex-foundation/js";




describe("test", () => {
  // Configure the client to use the local cluster.
 const provider = anchor.setProvider(anchor.AnchorProvider.local("http://127.0.0.1:8899"));
  const program = anchor.workspace
    .MetaticketVaultEscrowProgramV1 as Program<MetaticketVaultEscrowProgramV1>;
  const { connection } = program.provider;
  const commitment: Commitment = "confirmed";
  

  // CREATE ROLES
  const metaticket_authority = anchor.web3.Keypair.generate();
  console.log(metaticket_authority.publicKey);

  const metaticket_vault = anchor.web3.Keypair.generate();
  const mint = anchor.web3.Keypair.generate();
  const mint_nft = anchor.web3.Keypair.generate();
  const usdc_authority = anchor.web3.Keypair.generate();




  // CREATE AMOUNTS OF NFTS TO SEND TO VAULT
  // CREATE AMOUNT EXPECTED PER TICKET
  const metaticket_amount_nfts_to_vault = 100;
  const taker_amount_usdc_to_metaticket_per_ticket = 1;

 

  
  before("before call", async () => {
    //Airdrop 5 SOL to metaticket Auth
    const signature = await connection.requestAirdrop(metaticket_authority.publicKey, 100000000000);
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      commitment
    );
  });

  before("before call", async () => {
    //Airdrop 5 SOL to metaticket Auth
    const signature = await connection.requestAirdrop(metaticket_mint_authority, 20000000000);
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      commitment
    );
  });
  

  before("before call", async () => {
  
    const signature = await connection.requestAirdrop(metaticket_vault.publicKey, 200000000000);
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      commitment
    );
  });


  before("before call", async () => {
  
    const signature = await connection.requestAirdrop(vault_authority, 200000000000);
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
      Buffer.from("mint_auth"),
      metaticket_manager.toBuffer(),
      Buffer.from(new anchor.BN(id).toArrayLike(Buffer, "le", 8)),
    ];
  
    const [ metaticket_mint_authority, mint_auth_bump] = PublicKey.findProgramAddressSync(
      mintAuthoritySeeds,
      program.programId
    );
    console.log("This is the mint authority PDA address", metaticket_mint_authority);
  
  // DETERMINE THE ESCROW STATE PDA
  const seeds = [
    Buffer.from("escrow_state"),
    metaticket_manager.toBuffer(),
    Buffer.from(new anchor.BN(id).toArrayLike(Buffer, "le", 8)),
  ];

  const [escrow_state, escrow_bump] = PublicKey.findProgramAddressSync(
    seeds,
    program.programId
  );
  console.log("This is the escrow state PDA address", escrow_state);

  // DETERMINE THE VAULT AUTHORITY STATE PDA
  const vaultAuthoritySeeds = [
    Buffer.from("auth"),
  ];

  const [vault_authority, vault_authority_bump] = PublicKey.findProgramAddressSync(
    vaultAuthoritySeeds,
    program.programId
  );
  console.log("This is the vault authority PDA", vault_authority);



  


    it("Initialized the MetaTicket Manager!", async () => {
    
      try {const tx = await program.methods.
      initializeMetaticketManager()
      .accountsStrict({
        metaticketAuthority: metaticket_authority.publicKey,
        metaticketManager: metaticket_manager,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([metaticket_authority])
      .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("Error while creating a manager account:", error)
    }
    });
  
    it("Initialized the Mint Authority!", async () => {

      let id= 1

     try { const tx = await program.methods.
      mintingAuthAndEscrowState(new anchor.BN(id))
      .accountsStrict({
        metaticketAuthority: metaticket_authority.publicKey,
        metaticketManager: metaticket_manager,
        metaticketMintAuthority: metaticket_mint_authority,
        escrowState: escrow_state,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([metaticket_authority])
      .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("Error while creating a manager account:", error)

    }
    });

  it("Create Mock MetaTicket Collection", async () => {
try{
  let nft_mint = await createMint(
    connection,
    metaticket_authority,
    metaticket_authority.publicKey,
    metaticket_authority.publicKey,
    0
  );
  console.log(nft_mint)


  let nft_ticket_metaticket_ata = await getOrCreateAssociatedTokenAccount(
    connection,
    metaticket_authority,
    nft_mint,
    metaticket_authority.publicKey
  );

  let vault_ata = await getOrCreateAssociatedTokenAccount(
    connection,
    metaticket_authority,
    nft_mint,
    metaticket_vault.publicKey

  );

    let signature = await mintTo(
    connection,
    metaticket_authority,
    nft_mint,
    nft_ticket_metaticket_ata.address,
    metaticket_authority.publicKey, 
    1,
   
  )

  await setAuthority ( 
    connection,
    metaticket_authority,            // Payer of the transaction fees
    nft_mint,                       // Account 
    metaticket_authority.publicKey,  // Current authority 
    0,                              // Authority type: "0" represents Mint Tokens 
    null                            // Setting the new Authority to null
  );

  signature = await transfer(
    connection,
    metaticket_authority,               // Payer of the transaction fees 
    nft_ticket_metaticket_ata.address, // Source account 
    vault_ata.address,   // Destination account 
    metaticket_authority.publicKey,     // Owner of the source account 
    1                         // Number of tokens to transfer 
  );

  console.log("SIGNATURE", signature);

} catch (error) {
  console.log(error)
}
  

  
    });

    
















  //TAKERS AUTHORITY
  const ticket_buyers_account_authority = anchor.web3.Keypair.generate();


  before("before call", async () => {
    //Airdrop 5 SOL to metaticket Auth
    const signature = await connection.requestAirdrop(ticket_buyers_account_authority.publicKey, 5000000000);
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      commitment
    );
  });

  
    xit("Create mock USDC SPL Token for Takers Account", async () => {

      let USDC = await createMint(
        connection,
        ticket_buyers_account_authority,
        usdc_authority.publicKey,
        usdc_authority.publicKey,
        6,
        mint,
        null,
        TOKEN_PROGRAM_ID
      );
  
      let test = await getMint(connection, mint.publicKey, null, TOKEN_PROGRAM_ID);
      console.log(test);
  
      let usdc_ticket_buyers_account = await getOrCreateAssociatedTokenAccount(
        connection, 
        ticket_buyers_account_authority, 
        mint.publicKey, 
        ticket_buyers_account_authority.publicKey, 
        false, 
        undefined, 
        undefined, 
        TOKEN_PROGRAM_ID, 
        ASSOCIATED_TOKEN_PROGRAM_ID)

     
  
      let mint_to_sig = await mintToChecked(
        connection, 
        ticket_buyers_account_authority, 
        mint.publicKey, 
        usdc_ticket_buyers_account.address, 
        usdc_authority, 
        200e6, 
        6, 
        [], 
        undefined, 
        TOKEN_PROGRAM_ID);
  
      console.log(mint_to_sig);
      console.log(USDC);

      mintTo(
        connection,
        ticket_buyers_account_authority,
        USDC,
        ticket_buyers_account_authority.publicKey,
        usdc_authority,
        20

      )
    });

    
    


 







    


// it("Initialized the escrow and vault", async () => {

//   let id = 1

//       // Add your test here.
//      try { const tx = await program.methods.
//       initializeVaultAndEscrow(new anchor.BN(id),new anchor.BN(metaticket_amount_nfts_to_vault), new anchor.BN(taker_amount_usdc_to_metaticket_per_ticket)) 
//       .accountsStrict({
//         metaticketAuthority: metaticket_authority.publicKey,
//         mint: mint.publicKey,
//         metaticketMintAuthority: metaticket_mint_authority,
//         metaticketNftVault: metaticket_vault.publicKey,
//         escrowState: escrow_state,
//         metaticketNftAtasToVault: 
        


//       })
//       .signers([metaticket_authority])
//       .rpc();
//       console.log("Your transaction signature", tx);
//     } catch (error) {
//       console.log("Error while creating a manager account:", error)

//     }
//     });


});



