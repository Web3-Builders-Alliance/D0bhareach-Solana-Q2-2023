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

  const vault_state = anchor.web3.Keypair.generate();
  // use this to own vault
  const user = anchor.web3.Keypair.generate();

  const provider = getAnchorProvider(".config/solana/id.json"/*, "devnet"*/);
  anchor.setProvider(provider);
  const program = new Program(IDL, new PublicKey("HAXGA1FwMfL1pgdAeexVTADUbJ6Fk3ziTWKTD9jyjfbt"), provider);
  
  // string in Buffer must be the same as in lib.rs account  seeds!!!!
  const vault_auth_seed = [Buffer.from('auth'), vault_state.publicKey.toBuffer()] 
  const vault_auth_pubkey = PublicKey.findProgramAddressSync(vault_auth_seed, program.programId)[0];

  const vault_seed = [Buffer.from('holder'), vault_auth_pubkey.toBuffer()] 
  const vault_holder_pubkey = PublicKey.findProgramAddressSync(vault_seed, program.programId)[0];

  it("Starts an airdrop and confirms it", async () => {
    // Airdrop 200 SOL to payer
    const signature = await provider.connection.requestAirdrop(vault_state.publicKey, 0.2 * 1_000_000_000);
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
    {
        signature,
        ...latestBlockhash,
    },
  "finalized");  
  console.log(`Drop air to vaultState.`);
  });

  it("init vault", async () => {

    const tx = await program.methods.initialize()
    .accounts({
      vaultState: vault_state.publicKey,
      vaultAuth: vault_auth_pubkey,
      vaultHolder:vault_holder_pubkey,
      owner: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    
    })
    .signers([user, vault_state])
    .rpc();
    console.log(`init transaction: ${tx}`);
    let vault = await program.account.vault.fetch(vault_state.publicKey);
    expect(vault.score).to.equal(0);
  });

  /*
  it("update vault", async () => {
const vault_auth_seed = [Buffer.from('auth'), programProvider.wallet.publicKey.toBuffer()] 
const vault_auth_pubkey = PublicKey.findProgramAddressSync(vault_auth_seed, program.programId)[0];

const vault_seed = [Buffer.from('vault'), vault_auth_pubkey.toBuffer()] 
const vault_state = PublicKey.findProgramAddressSync(vault_seed, program.programId)[0];
console.log(`provider: ${programProvider.wallet.publicKey}`);

    await program.methods.initialize()
    .accounts({
      vaultAuth: vault_auth_pubkey,
      vaultHolder: vault_state,
      owner: programProvider.wallet
    })
    .rpc();

    const tx = await program.methods.updateScore()
    .accounts({
      vault: vault_state,
    })
    .signers([programProvider.wallet])
    .rpc();
    console.log("Your transaction signature", tx);
    let vault = await program.account.vault.fetch(vault_state);
    expect(vault.score).to.equal(1);
  });
  */
});
