import { Metaplex, keypairIdentity, bundlrStorage, toMetaplexFile } from "@metaplex-foundation/js";
import { clusterApiUrl, Connection, Keypair } from '@solana/web3.js';
import wallet from '../wba-wallet.json';
import { readFile } from "fs/promises"

// this is the first part of nft series
// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection(
  clusterApiUrl('devnet'),
  'confirmed'
);

const metaplex = Metaplex.make(connection)
    .use(keypairIdentity(keypair))
    .use(bundlrStorage({
        address: 'https://devnet.bundlr.network',
        providerUrl: "https://api.devnet.solana.com",
        timeout: 60000,
    }));

    (async () => {
        try {
            const img = await readFile("cluster1/image/generug.png");
            const metaplexImg = toMetaplexFile(img,"generug.png");
            const imageURI = await metaplex.storage().upload(metaplexImg)
            console.log(imageURI)
    
        } catch(e) {    
            console.error(`Oops, something went wrong: ${e}`)
        }
    })();
// https://arweave.net/MpdcbrVr5EuWj3PXG48x0JNDhf_OkR1M5IZIniqG5c8 