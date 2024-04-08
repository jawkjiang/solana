import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { Instructions } from "../target/types/instructions";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Instructions as anchor.Program<Instructions>;
  
   // Metaplex Constants
    const METADATA_SEED = "metadata";
    const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    // Constants from our program
    const MINT_SEEDs = ["credit","00001","00002","00003","00004","00005","00006","00007","00008","00009"];

    const metadatas = [
        {
          name: "Credit",
          symbol: "CRD",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/credit.json",
          decimals: 0,
          id: "credit",
          price:new BN(0.0002 * web3.LAMPORTS_PER_SOL),
        },
        {
          name: "Twilight Whisper",
          symbol: "TWS",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/TWS.json",
          decimals: 0,
          id: "00001",
          price:new BN(0.01 * web3.LAMPORTS_PER_SOL),
        },
        {
          name: "Azure Seascape",
          symbol: "ASC",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/ASC.json",
          decimals: 0,
          id: "00002",
          price:new BN(0.02 * web3.LAMPORTS_PER_SOL),
        },
        {
          name: "Blockchain Elixir",
          symbol: "BCE",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/BCE.json",
          decimals: 0,
          id: "00003",
          price:new BN(0.234 * web3.LAMPORTS_PER_SOL),
        },
        {
          name: "Satoshi Spritz",
          symbol: "SPZ",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/SPZ.json",
          decimals: 0,
          id: "00004",
          price:new BN(0.198 * web3.LAMPORTS_PER_SOL),
        },
        {
          name: "Ether Fizz",
          symbol: "ETF",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/ETF.json",
          decimals: 0,
          id: "00005",
          price:new BN(0.177 * web3.LAMPORTS_PER_SOL),
        },
        {
          name: "Crypto Colada",
          symbol: "CCL",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/CCL.json",
          decimals: 0,
          id: "00006",
          price:new BN(0.209 * web3.LAMPORTS_PER_SOL),
        },
        {
          name: "Forest Murmur",
          symbol: "FMU",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/FMU.json",
          decimals: 0,
          id: "00007",
          price:new BN(0.155 * web3.LAMPORTS_PER_SOL),
        },
        {
          name: "Mythical Brew",
          symbol: "MYB",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/MYB.json",
          decimals: 0,
          id: "00008",
          price:new BN(0.199 * web3.LAMPORTS_PER_SOL),
        },
        {
          name: "Sunset Serenade",
          symbol: "SSD",
          uri: "https://raw.githubusercontent.com/jawkjiang/solanaHackathon/main/SSD.json",
          decimals: 0,
          id: "00009",
          price:new BN(0.177 * web3.LAMPORTS_PER_SOL)
        }
    ]
  
    // Data for our tests
    const payer = program.provider.publicKey;

    let mintPDAs = [];
    let pricePDAs = [];
    let metadataAddresses = [];

    for (const SEED of MINT_SEEDs) {
        let [mint] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEED)],
             program.programId
        );
        mintPDAs.push(mint);
        let [pricePDA] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from(SEED + "-price")],
            program.programId
        );
        pricePDAs.push(pricePDA);
        let [metadataAddress] = web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from(METADATA_SEED),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
          TOKEN_METADATA_PROGRAM_ID
        );
        metadataAddresses.push(metadataAddress);

    }
  
    // Test init token
    it("init 9 fts ", async () => {

      const priceAccount = await program.account.price.fetch(pricePDAs[1])
      console.log(`FT id: ${priceAccount.id}`)
      console.log(`FT price(unit lamport 1 sol = 1*10 18 lamport ): ${priceAccount.price}`)
      

    // mintPDAs.slice(0,3).forEach(async function (v,i){
    //   const context = {
    //     ftPrice:pricePDAs[i],
    //     metadata: metadataAddresses[i],
    //     mint: v,
    //     payer: payer,
    //     rent: web3.SYSVAR_RENT_PUBKEY,
    //     systemProgram: web3.SystemProgram.programId,
    //     tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    //     tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    //   };
    //   const txHash = await program.methods
    //     .initFt(metadatas[i])
    //     .accounts(context)
    //     .rpc();
    //   const priceAccount = await program.account.price.fetch(pricePDAs[i])
    //   console.log(`FT id: ${priceAccount.id}`)
    //   console.log(`FT price(unit lamport 1 sol = 1*10 18 lamport ): ${priceAccount.price}`)
    // });
    
    });


  
    //Test mint tokens
    it("mint tokens", async () => {

    const recipient = new web3.PublicKey("66T11pRjNzMnvgAUYTo6zjRGqEJK5Gi8WkEaDVmwEYoa");

    const destination1 = await anchor.utils.token.associatedAddress({
      mint: mintPDAs[1],
      owner: payer,
    });

    const destination2 = await anchor.utils.token.associatedAddress({
      mint: mintPDAs[2],
      owner: payer,
    });

    const destinationCredit = await anchor.utils.token.associatedAddress({
      mint: mintPDAs[0],
      owner: payer,
    });

    const credit_recipient = await anchor.utils.token.associatedAddress({
      mint: mintPDAs[0],
      owner: recipient,
    });



    
    const context = {

      mintCredit: mintPDAs[0],
      destinationCredit: destinationCredit,

      mint1: mintPDAs[1],
      destination1: destination1,
      ftPrice1: pricePDAs[1],

      mint2: mintPDAs[2],
      destination2: destination2,
      ftPrice2: pricePDAs[2],

      payer:payer,

      solRecipient: recipient,
      creditRecipient: credit_recipient,

      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    };

    const txHash = await program.methods
      .buyFt([new BN(1),new BN(0)], true,new BN(30) )
      .accounts(context)
      .rpc(
        {
          skipPreflight:true
        }
      );
    await program.provider.connection.confirmTransaction(txHash);
    console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);

    const postBalance = (
      await program.provider.connection.getTokenAccountBalance(destination1)
    ).value.uiAmount;
    console.log("ft1: ", postBalance);

    const postBalance1 = (
      await program.provider.connection.getTokenAccountBalance(destination2)
    ).value.uiAmount;
    console.log("ft1: ", postBalance1);

    const creditBalance1 = (
      await program.provider.connection.getTokenAccountBalance(destinationCredit)
    ).value.uiAmount;
    console.log("repcipient credit: ", creditBalance1);



  });

});
