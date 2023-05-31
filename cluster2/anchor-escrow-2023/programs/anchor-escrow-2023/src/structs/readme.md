### To run tests.
Tests are running on localnet. Before tests check that program id is correct in
lib.rs, Anchor.toml, tests/anchor-escrow-2023. To get keys: `anchor keys list`.   
Start new terminal and run `solana-test-validator` then, 
return to previous and run `anchor test --skip-local-validator`. If you are not
building program run `anchor test --skip-build --skip-deploy --skip-local-validator`   
it will save deploying and on `devnet` will save SOL.