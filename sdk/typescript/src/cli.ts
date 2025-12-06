#!/usr/bin/env node

import { Command } from 'commander';
import { Connection, clusterApiUrl } from '@solana/web3.js';
import { ZerisClient } from './client';
import * as fs from 'fs';

const program = new Command();

program
  .name('zeris-cli')
  .description('CLI for Zeris Zero-Knowledge Resolution & Integrity System')
  .version('0.1.0');

program
  .command('submit-proof')
  .description('Submit a zero-knowledge proof')
  .requiredOption('-k, --keypair <path>', 'Path to keypair JSON file')
  .requiredOption('-p, --proof <path>', 'Path to proof JSON file')
  .option('-c, --cluster <cluster>', 'Solana cluster', 'devnet')
  .action(async (options) => {
    try {
      const connection = new Connection(clusterApiUrl(options.cluster as any));
      const client = new ZerisClient(connection);

      const keypairData = JSON.parse(fs.readFileSync(options.keypair, 'utf8'));
      const keypair = {
        publicKey: keypairData.slice(32),
        secretKey: keypairData,
      } as any; // Simplified

      const proofData = JSON.parse(fs.readFileSync(options.proof, 'utf8'));
      const proof = client.getProofBuilder().buildProofFromJson(proofData);

      const signature = await client.submitProof(keypair, proof);
      console.log('Proof submitted successfully. Transaction signature:', signature);
    } catch (error) {
      console.error('Error submitting proof:', error);
      process.exit(1);
    }
  });

program
  .command('verify-proof')
  .description('Verify a proof hash')
  .requiredOption('-h, --hash <hash>', 'Proof hash (base58)')
  .option('-c, --cluster <cluster>', 'Solana cluster', 'devnet')
  .action(async (options) => {
    try {
      const connection = new Connection(clusterApiUrl(options.cluster as any));
      const client = new ZerisClient(connection);

      const hash = Buffer.from(options.hash, 'base64'); // Simplified
      await client.verifyProof(hash);
      console.log('Proof verified successfully');
    } catch (error) {
      console.error('Error verifying proof:', error);
      process.exit(1);
    }
  });

program
  .command('get-state')
  .description('Get current Zeris state')
  .option('-c, --cluster <cluster>', 'Solana cluster', 'devnet')
  .action(async (options) => {
    try {
      const connection = new Connection(clusterApiUrl(options.cluster as any));
      const client = new ZerisClient(connection);

      const state = await client.getZerisState();
      console.log('Zeris State:');
      console.log('Last Proof Hash:', Buffer.from(state.lastProofHash).toString('hex'));
      console.log('Merkle Root:', Buffer.from(state.merkleRoot).toString('hex'));
      console.log('Proof Count:', state.proofCount);
      console.log('Authority:', state.authority.toString());
    } catch (error) {
      console.error('Error getting state:', error);
      process.exit(1);
    }
  });

program.parse();