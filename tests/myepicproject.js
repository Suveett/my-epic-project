const anchor = require('@project-serum/anchor');
//Need to specify Solana runtime as the SytemProgram. 
const {SystemProgram} = anchor.web3;


const main = async() => {
  console.log("🚀 Starting test...");

// Specify provider environment. 
const provider = anchor.Provider.env();
//Set provider.
anchor.setProvider(provider);

//Specify the workspace 
const program = anchor.workspace.Myepicproject;

//Generate keyPair for baseAccount
const baseAccount = anchor.web3.Keypair.generate();



//Lets invocate/ initialise the startStuffOff context. 
let tx = await program.rpc.startStuffOff({
  accounts : {
    baseAccount : baseAccount.publicKey, 
    user : provider.wallet.publicKey,
    systemProgram : SystemProgram.programId,
  },
  signers : [baseAccount],
});

//Console.log the Transaction signature of the Initialization procedure. 
console.log("You transaction signature", tx);

let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
//Lets now get the exact Gif Count until this stage. 
console.log("👀 Total Gif Count : ", account.totalGifs.toString());


//Now lets add Gifs that the User/ Player provides to the struct 'baseAccount' in our 'Myepicproject program'. 
await program.rpc.addGifs("Provide a Fancy Gif Link here", {
  accounts : {
    baseAccount : baseAccount.publicKey,
    user : provider.wallet.publicKey, 
  }
});

//Now lets again check the GIF Count. It must have changed.
account = await program.account.baseAccount.fetch(baseAccount.publicKey);
//Now lets console.log the 'Total Gifs'.
console.log("👀 Total Gif Count : ", account.totalGifs.toString());
//Now lets console log the Gif List of the User.
console.log("👀 Gif List : ", account.gifList);



// Now lets try to send some SOL from one account to another :

/*await program.rpc.sendSol(1, {
  accounts : {
    
    user: provider.wallet.publicKey,
    baseAccount : baseAccount.publicKey, 
      
  }
});*/

}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
}


runMain();




