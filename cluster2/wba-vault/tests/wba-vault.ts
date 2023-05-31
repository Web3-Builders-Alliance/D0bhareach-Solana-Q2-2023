import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {IDL,  WbaVault } from "../target/types/wba_vault";
import { clusterApiUrl, Connection, ConfirmOptions, Commitment, Keypair, PublicKey } from '@solana/web3.js';
import Fs from '@supercharge/fs';
import * as os from 'os';
import path from 'path';
import chai from 'chai';
import chaiAsPromised from 'chai-as-promised';
import { expect } from 'chai';

chai.use(chaiAsPromised);

// need to setup two mints with mint authority would be the wallet.
// need to set up two accounts for exchange one with mint apples and one with mint oranges.
// 

function getAnchorProvider(file:string/*, cluster: any*/): anchor.AnchorProvider {
  const home = os.homedir();
  const commitment: Commitment = 'confirmed';
  const connection: Connection = new Connection("http://localhost:8899",/*clusterApiUrl(cluster),*/ commitment);
  const walletJson = Fs.readJsonSync(path.join(home, file));
  const key  = Keypair.fromSecretKey(new Uint8Array(walletJson));
  const configOpt: ConfirmOptions = {
    skipPreflight: true,
    commitment: commitment,
  }
  const wallet = new anchor.Wallet(key);
  return new anchor.AnchorProvider(connection, wallet, configOpt);
}

describe("wba-vault", () => {
  // if remove this line test will fail!!! Don't know why?
  anchor.setProvider(anchor.AnchorProvider.env());
  let vault_state = null;
  let user = null;
  let provider = null;
  let program = null;
  let vault_auth_seed = null;
  let vault_auth_pubkey = null;
  let vault_seed = null;
  let vault_holder_pubkey = null;

  beforeEach( () => {
  vault_state = anchor.web3.Keypair.generate();
  // use this to own vault
  user = anchor.web3.Keypair.generate();

  provider = getAnchorProvider(".config/solana/id.json"/*, "devnet"*/);
  anchor.setProvider(provider);
  program = new Program(IDL, new PublicKey("HAXGA1FwMfL1pgdAeexVTADUbJ6Fk3ziTWKTD9jyjfbt"), provider);
  
  // string in Buffer must be the same as in lib.rs account  seeds!!!!
  vault_auth_seed = [Buffer.from('auth'), vault_state.publicKey.toBuffer()] 
  vault_auth_pubkey = PublicKey.findProgramAddressSync(vault_auth_seed, program.programId)[0];

  vault_seed = [Buffer.from('holder'), vault_auth_pubkey.toBuffer()] 
  vault_holder_pubkey = PublicKey.findProgramAddressSync(vault_seed, program.programId)[0];
  });

  it("Starts an airdrop and confirms it", async function() {
    this.skip();
    // Airdrop 200 SOL to payer
    const signature = await provider.connection.requestAirdrop(vault_state.publicKey, 0.2 * 1_000_000_000);
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
    {
        signature,
        ...latestBlockhash,
    },
  "finalized");  
  });

  it("init vault", async function() {
    this.skip();
    const signature = await provider.connection.requestAirdrop(vault_state.publicKey, 0.5 * 1_000_000_000);
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
    {
        signature,
        ...latestBlockhash,
    },
  "finalized");  

    let tx = await program.methods.initialize()
    .accounts({
      vaultState: vault_state.publicKey,
      vaultAuth: vault_auth_pubkey,
      vaultHolder:vault_holder_pubkey,
      owner: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    
    })
    .signers([user, vault_state])
    .rpc();
    // console.log(`init transaction: ${tx}`);
    let r_vault = await program.account.vault.fetch(vault_state.publicKey);
    expect(r_vault.score).to.equal(0);
  });

  it("update vault", async function() {
    const signature = await provider.connection.requestAirdrop(vault_state.publicKey, 0.5 * 1_000_000_000);
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
    {
        signature,
        ...latestBlockhash,
    },
  "finalized");  

    await program.methods.initialize()
    .accounts({
      vaultState: vault_state.publicKey,
      vaultAuth: vault_auth_pubkey,
      vaultHolder:vault_holder_pubkey,
      owner: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([user, vault_state])
    .rpc();
    let r_vault = await program.account.vault.fetch(vault_state.publicKey);
    expect(r_vault.score).to.equal(0);
    // get from pda
    const tx = await program.methods.updateScore()
    .accounts({
      vault: vault_state.publicKey,
    })
    .rpc();

    const vault = await program.account.vault.fetch(vault_state.publicKey);
    expect(vault.score).to.equal(1);
  });
});
