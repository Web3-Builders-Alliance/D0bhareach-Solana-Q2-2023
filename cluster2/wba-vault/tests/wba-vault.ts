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

async function getAnchorProvider(file:string/*, cluster: any*/): Promise<anchor.AnchorProvider> {
  const home = os.homedir();
  const commitment: Commitment = 'confirmed';
  const connection: Connection = new Connection("http://localhost:8899",/*clusterApiUrl(cluster),*/ commitment);
  const walletJson = await Fs.readJson(path.join(home, file));
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
  let program = null;
  let programProvider = null;
  beforeEach(async function(): Promise<void> {
        const provider = await getAnchorProvider(".config/solana/id.json"/*, "devnet"*/);
        anchor.setProvider(provider);
        program = new Program(IDL, new PublicKey("HAXGA1FwMfL1pgdAeexVTADUbJ6Fk3ziTWKTD9jyjfbt"), programProvider);
        programProvider = provider;
    });

  it("init vault", async () => {
    const owner_keypair = programProvider.wallet;
const vault_auth_seed = [Buffer.from('auth'), programProvider.wallet.publicKey.toBuffer()] 
const vault_auth_pubkey = PublicKey.findProgramAddressSync(vault_auth_seed, program.programId)[0];

const vault_seed = [Buffer.from('vault'), vault_auth_pubkey.toBuffer()] 
const vault_state = PublicKey.findProgramAddressSync(vault_seed, program.programId)[0];

    const tx = await program.methods.initialize()
    .accounts({
      vaultAuth: vault_auth_pubkey,
      vaultHolder: vault_state,
      owner: owner_keypair
    })
    .rpc();
    console.log(`transaction: ${tx}`);

  });

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
});
