import { clusterApiUrl, Commitment, Connection, Keypair, PublicKey, Transaction, sendAndConfirmTransaction } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { CreateMetadataAccountArgsV3, createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";

// We're going to import our keypair from the wallet file
const walletKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection(clusterApiUrl("devnet"), commitment);

// Define our Mint address
const mint = new PublicKey("6DojqQKrXhyuTzAKNgkVELjfYfkEThyP7p9RoM2itdaA")

// Add the Token Metadata Program
const token_metadata_program_id = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')

// Create PDA for token metadata
const metadata_seeds = [
    Buffer.from('metadata'),
    token_metadata_program_id.toBuffer(),
    mint.toBuffer(),
];
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(metadata_seeds, token_metadata_program_id);

(async () => {
    try {
        const tx = new Transaction().add(
            createCreateMetadataAccountV3Instruction(
                {
                    metadata: metadata_pda,
                    mint: mint,
                    mintAuthority: walletKeypair.publicKey,
                    payer: walletKeypair.publicKey,
                    updateAuthority: walletKeypair.publicKey,
                },
            {    
            // about data filed read in fields section of this site:
            // https://docs.metaplex.com/programs/token-metadata/accounts#metadata
            createMetadataAccountArgsV3: {
                data: {
                    name: "Name for meta?",
                    symbol: "Q2",
                    uri: "",
                    sellerFeeBasisPoints: 100,
                    creators: [{address: walletKeypair.publicKey, verified: true, share: 100}],
                    collection: null,
                    uses: null,
                },
                isMutable: false,
                collectionDetails: null
            }
    }    
    ),
        );
    const tx_hash = await sendAndConfirmTransaction(connection, tx, [walletKeypair]);
    console.log(`Success! Checkout here:\nhttps://explorer.solana.com${tx_hash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();