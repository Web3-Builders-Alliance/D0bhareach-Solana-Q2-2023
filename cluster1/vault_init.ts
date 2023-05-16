import { clusterApiUrl, Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Address } from "@project-serum/anchor"
import { WBA_Vault, IDL } from "./programs/wba_vault";
import wallet from "../wba-wallet.json";
const connection = new Connection(clusterApiUrl("devnet"));
const walletKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const provider = new AnchorProvider(connection, new Wallet(walletKeypair), { commitment: "confirmed"});
const program = new Program<WBA_Vault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);
// programId: D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o


// Accont for what?
const vaultState = Keypair.generate();
// IMPORTANT DON'T FORGET TO SAVE THIS KEY OTHERWISE IT IMPOSSIBLE TO FIND.
console.log(`vaultState PublicKey: ${vaultState.publicKey.toBase58()}`);
// get two PDA  for vaultState Account.
const vault_auth_seed = [Buffer.from('auth'), vaultState.publicKey.toBuffer()] 
const vaultAuth = PublicKey.findProgramAddressSync(vault_auth_seed, program.programId)[0];

const vault_seed = [Buffer.from('vault'), vaultAuth.toBuffer()] 
const vault = PublicKey.findProgramAddressSync(vault_seed, program.programId)[0];

(async () => {
    try {
        const txhash = await program.methods
        .initialize()
        .accounts({
            owner: walletKeypair.publicKey,
            vaultState:  vaultState.publicKey,
            vaultAuth,
            vault,
            systemProgram: SystemProgram.programId
        })
        .signers([
            walletKeypair,
            vaultState
        ]).rpc();
        console.log(`Success! Check out your TX here: 
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();