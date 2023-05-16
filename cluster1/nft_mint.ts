import { Metaplex, keypairIdentity, bundlrStorage } from "@metaplex-foundation/js";
import { clusterApiUrl, Connection, Keypair } from '@solana/web3.js';
import wallet from '../wba-wallet.json';
import rug_metadata from './rug_metadata.json';

// This is second part of nft part.
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
        const { uri } = await metaplex.nfts().uploadMetadata({
            ...rug_metadata,
            name: "My Rug Metadata",
        });
        console.log(uri);

        const { nft } = await metaplex.nfts().create(
            {
                uri,
                name: "D0bhareach Rug",
                symbol: "D0BH",
                creators: [{
                    address: keypair.publicKey,
                    share: 100,
                }],
                sellerFeeBasisPoints: 500,
                isMutable: true
            }
        )
        console.log(nft.address);

    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
// 2WzMz3gu8d5opE1bMoYQHszTeaBefSiMZF5LL2SJgyeJ