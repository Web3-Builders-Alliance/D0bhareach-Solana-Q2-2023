import { clusterApiUrl, Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "../wba-wallet.json"

// Import our keypair from the wallet file
// Create a new token mint with Decimals set to 6

// Make your WBA devnet wallet the mint authority
// Create a associated token account for your wallet
// Mint tokens to your self

// create keypair from my wallet
const walletKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// new accont for mint 
const payer = Keypair.generate();

// Mint Authority is the public key of the keypair that is allowed to mint tokens.
// If anyone else tries to mint tokens, it will fail.

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection(clusterApiUrl('devnet'), commitment);

// Decimals — dictates the smallest denomination of the token.
// For NFTs, it should be zero. Nine is the default.
(async () => {
    try {
        const mint = await createMint(
            connection,
            walletKeypair,
            walletKeypair.publicKey,
            null, // now going to freeze this mint. What ever this means.
            6 // We are using 6 decimals
        );
        console.log(`Mint's programId: ${mint.toBase58()}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
// Mint's programId: 6DojqQKrXhyuTzAKNgkVELjfYfkEThyP7p9RoM2itdaA