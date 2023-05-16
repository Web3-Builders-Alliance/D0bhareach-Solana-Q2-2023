import { clusterApiUrl, Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer, AccountLayout, TOKEN_PROGRAM_ID } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const walletKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection(clusterApiUrl("devnet"), commitment);

// Mint address
const mint = new PublicKey("6DojqQKrXhyuTzAKNgkVELjfYfkEThyP7p9RoM2itdaA");

// Recipient address
// const to = new PublicKey("<receiver address>");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const mintTokenFrom = await getOrCreateAssociatedTokenAccount(
            connection,
            walletKeypair,
            mint,
            walletKeypair.publicKey,

        );

        console.log(`Associated Token Account: ${mintTokenFrom.address.toBase58()}`);
        
        // create associated token account from my mint managed by other wallet.
        // NEED PUBLIC KEY FOR OTHER PERSON.
        // is it safe???
        const mintTokenTo = await getOrCreateAssociatedTokenAccount(
            connection,
            walletKeypair,
            mint,
            new PublicKey("<insert friends acc>"), // to account.
        );

        // Transfer the new token to the "toTokenAccount" we just created
        const res = await transfer(
            connection,
            walletKeypair,
            mintTokenFrom.address,
            mintTokenTo.address,
            walletKeypair.publicKey,
            2 * 1e6
        );

        console.log(`Success! Checkout here:\nhttps://explorer.solana.com${res}?cluster=devnet`)

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();