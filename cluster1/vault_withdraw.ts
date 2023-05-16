import { clusterApiUrl, Connection, Keypair, SystemProgram, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js"
import { BN, Program, Wallet, AnchorProvider, Address } from "@project-serum/anchor"
import { WBA_Vault, IDL } from "./programs/wba_vault";
import wallet from "../wba-wallet.json";
const connection = new Connection(clusterApiUrl("devnet"));
const walletKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const provider = new AnchorProvider(connection, new Wallet(walletKeypair), { commitment: "confirmed"});
const program = new Program<WBA_Vault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

const vaultState = new PublicKey("BspA59rfKVSUw1kQMnnPWyLKTQ9nYig2zerLCHYSTkk7");
// get two PDA  for vaultState Account.
const vault_auth_seed = [Buffer.from('auth'), vaultState.toBuffer()] 
const vaultAuth = PublicKey.findProgramAddressSync(vault_auth_seed, program.programId)[0];

const vault_seed = [Buffer.from('vault'), vaultAuth.toBuffer()] 
const vault = PublicKey.findProgramAddressSync(vault_seed, program.programId)[0];


(async () => {
    try {
        const txhash = await program.methods
        .withdraw(new BN(0.1* LAMPORTS_PER_SOL),)
        .accounts({
            owner: walletKeypair.publicKey,
            vaultState:  vaultState,
            vaultAuth,
            vault,
            systemProgram: SystemProgram.programId
        })
        .signers([
            walletKeypair,
        ]).rpc();
        console.log(`Success! Check out your TX here: 
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();