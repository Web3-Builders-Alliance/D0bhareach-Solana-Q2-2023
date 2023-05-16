import { clusterApiUrl, Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../wba-wallet.json"

/**
 * Create Associated Token Account
 * https://spl.solana.com/associated-token-account
 * for my wallet. Then send some tokens to my wallet with mintTo method.
 */

// Import our keypair from the wallet file
const walletKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection(clusterApiUrl("devnet"), commitment);

// denominator. Still not sure what it is for.
const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("6DojqQKrXhyuTzAKNgkVELjfYfkEThyP7p9RoM2itdaA");

(async () => {
    try {
        // Get Associated Token Accont
        const ata = await getOrCreateAssociatedTokenAccount(
            connection,
            walletKeypair,
            mint,
            walletKeypair.publicKey,
        );
        console.log(`Associated Token Account: ${ata.address.toBase58()}`);
        // LbmroXgZCpBb3gbMMCQiCdNNBFL6PxDut6MvGgForTj

        const res  = await mintTo(
            connection,
            walletKeypair,
            mint,
            ata.address,
            walletKeypair.publicKey,
            100n * token_decimals
        );
        console.log('Explore result of mintTo:');
        console.log(`https://explorer.solana.com/tx/${res}?cluster=devnet`);
        // 1: 2iy7NQ4e2uAFK9NzQkTdmoT6qV4hXji5WudJzjb2fSFpRfN57sW1xhS2Uga4LP2A54VjH93qM7qjg8oxXX9B8Vbp
        // 2: https://explorer.solana.com/tx/5xrjmAYEF6EttEku8CTYH9SYnKBbvfa1YwAH4tFykt8crjroPtP8qwqSRofDCFJ6EzoLjfRFdRms7u5RZ75rpCmP?cluster=devnet



    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()