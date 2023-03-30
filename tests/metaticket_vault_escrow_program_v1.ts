import * as anchor from "@project-serum/anchor";
import { Program, web3, workspace } from "@project-serum/anchor";
import { MetaticketVaultEscrowProgramV1 } from "../target/types/metaticket_vault_escrow_program_v1";
import { PublicKey, Commitment, Transaction} from "@solana/web3.js";
describe("test", () => {
  // Configure the client to use the local cluster.
 anchor.setProvider(anchor.AnchorProvider.local("http://127.0.0.1:8899"));
  const program = anchor.workspace
    .MetaticketVaultEscrowProgramV1 as Program<MetaticketVaultEscrowProgramV1>;
    
  const { connection } = program.provider;
  const commitment: Commitment = "confirmed";

  // CREATE ROLES

  const metaticket_authority = anchor.web3.Keypair.generate();
  const mint = anchor.web3.Keypair.generate();


  before("before call", async () => {
    //Airdrop 5 SOL to metaticket Auth
    const signature = await connection.requestAirdrop(metaticket_authority.publicKey, 5000000000);
    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      commitment
    );
  });


  //SET ID to 1, which will be the value of the first vault created by MetaTicket
  let id = 0;
  console.log(id);

  // DETERMINE THE METATICKET MANAGER WITH THE PDA

  const managerSeeds = [
    Buffer.from("manager"),
    metaticket_authority.publicKey.toBuffer()
  ];

  const [metaticket_manager, _manager_bump] = PublicKey.findProgramAddressSync(
    managerSeeds,
    program.programId
  );
  console.log("This is the Metaticket Manager PDA", metaticket_manager);


   // DETERMINE THE MINT AUTHORITY  PDA
    // THIS WILL BE THE AUTHORITY WE USE TO MINT TICKETS FOR EACH COLLECTION

    const mintAuthoritySeeds = [
      Buffer.from("mint_auth"),
      metaticket_manager.toBuffer(),
      Buffer.from(id.toString()),
    ];
  
    const [ metaticket_mint_authority, _mint_auth_bump] = PublicKey.findProgramAddressSync(
      mintAuthoritySeeds,
      program.programId
    );
    console.log("This is the mint authority PDA address", metaticket_mint_authority);
  

  // DETERMINE THE METATICKET NFT VAULT WITH THE PDA

  const vaultSeeds = [
    Buffer.from("vault"),
    metaticket_authority.publicKey.toBuffer(),
    Buffer.from(id.toString()),
  ];

  const [metaticket_nft_vault, _vault_bump] = PublicKey.findProgramAddressSync(
    vaultSeeds,
    program.programId
  );
  console.log("This is the nft vault PDA address", metaticket_nft_vault);










  // DETERMINE THE ESCROW STATE PDA

  const seeds = [
    Buffer.from("escrow_state"),
    metaticket_manager.toBuffer(),
    Buffer.from(id.toString()),
  ];

  const [escrow_state, escrow_bump] = PublicKey.findProgramAddressSync(
    seeds,
    program.programId
  );
  console.log("This is the escrow state PDA address", escrow_state);

  // AFTER PDAS ARE GENERATED WE CALL AN INCREMENT FUNCTION FOR THE NEXT ROUND OF PDAS
  // THIS WONT BE IMPLIMENTED BECAUSE TEST RUNS FRESH EVERYTIME
  //SOLUTION?

  function incrementId() {
    id += 1;
  }
  incrementId();
  console.log(id);








  // FUND METATICKET AUTHORITYnAS PAYER






    it("Initialized the MetaTicket Manager!", async () => {

      let id= 0

      // Add your test here.
      const tx = await program.methods.
      initializeMetaticketManager(new anchor.BN(id))
      .accounts({
        metaticketAuthority: metaticket_authority.publicKey,
        metaticketManager: metaticket_manager,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([metaticket_authority])
      .rpc();
      console.log("Your transaction signature", tx);
    });

    it("Initialized the Mint Authority!", async () => {

      let id= 1

      // Add your test here.
      const tx = await program.methods.
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
    });





});

