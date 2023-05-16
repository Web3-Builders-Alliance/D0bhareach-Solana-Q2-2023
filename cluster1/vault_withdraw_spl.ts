import { clusterApiUrl, Connection, Keypair, SystemProgram, PublicKey, LAMPORTS_PER_SOL, Commitment } from "@solana/web3.js"
import { BN, Program, Wallet, AnchorProvider, Address } from "@project-serum/anchor"
import { WBA_Vault, IDL } from "./programs/wba_vault";
import wallet from "../wba-wallet.json";
import { TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

const commitment: Commitment = 'finalized';
const connection = new Connection(clusterApiUrl("devnet"), commitment);

const walletKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const provider = new AnchorProvider(connection, new Wallet(walletKeypair), { commitment: "confirmed"});
const program = new Program<WBA_Vault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

const vaultState = new PublicKey("BspA59rfKVSUw1kQMnnPWyLKTQ9nYig2zerLCHYSTkk7");
// get two PDA  for vaultState Account.
const vault_auth_seed = [Buffer.from('auth'), vaultState.toBuffer()] 
const vaultAuth = PublicKey.findProgramAddressSync(vault_auth_seed, program.programId)[0];
const mint = new PublicKey("6DojqQKrXhyuTzAKNgkVELjfYfkEThyP7p9RoM2itdaA");


async function DepositSpl(){}
(async () => {
    try {
        const ownerAta = await getOrCreateAssociatedTokenAccount(
            connection,
            walletKeypair,
            mint,
            walletKeypair.publicKey
        );
        const vaultAta = await getOrCreateAssociatedTokenAccount(
            connection,
            walletKeypair,
            mint,
            vaultAuth,
            true
        );

        const txhash = await program.methods
        .withdrawSpl(new BN(0.1* LAMPORTS_PER_SOL),)
        .accounts({
            owner: walletKeypair.publicKey,
            vaultState,
            vaultAuth,
            systemProgram: SystemProgram.programId,
            ownerAta: ownerAta.address,
            vaultAta: vaultAta.address,
            tokenMint: mint,
            tokenProgram: TOKEN_PROGRAM_ID
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