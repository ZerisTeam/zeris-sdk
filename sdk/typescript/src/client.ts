import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
  Keypair,
  SystemProgram,
} from '@solana/web3.js';
import { sha256 } from 'js-sha256';
import { ProofData, ZerisState, ZerisEvent } from './types';
import { ProofBuilder } from './proof';
import { MerkleSyncer } from './verifier';

export class ZerisClient {
  private connection: Connection;
  private programId: PublicKey;
  private proofBuilder: ProofBuilder;
  private merkleSyncer: MerkleSyncer;

  constructor(
    connection: Connection,
    programId: PublicKey = new PublicKey('ZER1S11111111111111111111111111111111111111')
  ) {
    this.connection = connection;
    this.programId = programId;
    this.proofBuilder = new ProofBuilder();
    this.merkleSyncer = new MerkleSyncer(connection, programId);
  }

  async submitProof(
    signer: Keypair,
    proofData: ProofData
  ): Promise<string> {
    const [statePda] = PublicKey.findProgramAddressSync(
      [Buffer.from('zeris-state')],
      this.programId
    );

    const instruction = new TransactionInstruction({
      keys: [
        { pubkey: statePda, isSigner: false, isWritable: true },
        { pubkey: signer.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: this.programId,
      data: Buffer.concat([
        Buffer.from([0]), // submit_proof instruction
        Buffer.from(proofData.proof),
        Buffer.from(proofData.publicSignals),
      ]),
    });

    const transaction = new Transaction().add(instruction);
    const signature = await sendAndConfirmTransaction(
      this.connection,
      transaction,
      [signer]
    );

    return signature;
  }

  async verifyProof(proofHash: Uint8Array): Promise<void> {
    const [statePda] = PublicKey.findProgramAddressSync(
      [Buffer.from('zeris-state')],
      this.programId
    );

    const instruction = new TransactionInstruction({
      keys: [
        { pubkey: statePda, isSigner: false, isWritable: true },
      ],
      programId: this.programId,
      data: Buffer.concat([
        Buffer.from([1]), // verify_proof instruction
        Buffer.from(proofHash),
      ]),
    });

    // Note: This instruction doesn't require a signer as it's read-only verification
    // In practice, you might want to add a signer for logging purposes
  }

  async validateMerkleRoot(rootHash: Uint8Array): Promise<void> {
    const [statePda] = PublicKey.findProgramAddressSync(
      [Buffer.from('zeris-state')],
      this.programId
    );

    const instruction = new TransactionInstruction({
      keys: [
        { pubkey: statePda, isSigner: false, isWritable: false },
      ],
      programId: this.programId,
      data: Buffer.concat([
        Buffer.from([2]), // validate_merkle_root instruction
        Buffer.from(rootHash),
      ]),
    });

    // Similar to verifyProof
  }

  async getZerisState(): Promise<ZerisState> {
    const [statePda] = PublicKey.findProgramAddressSync(
      [Buffer.from('zeris-state')],
      this.programId
    );

    const accountInfo = await this.connection.getAccountInfo(statePda);
    if (!accountInfo) {
      throw new Error('Zeris state account not found');
    }

    // Parse account data (simplified)
    const data = accountInfo.data;
    return {
      lastProofHash: data.slice(8, 40),
      merkleRoot: data.slice(40, 72),
      proofCount: data.readBigUInt64LE(72),
      authority: new PublicKey(data.slice(80, 112)),
    };
  }

  async listenForEvents(callback: (event: ZerisEvent) => void): Promise<void> {
    // Set up WebSocket listeners for program logs
    this.connection.onLogs(this.programId, (logs) => {
      // Parse logs for events (simplified)
      // In practice, you'd parse the log messages for event data
      console.log('Event logs:', logs);
    });
  }

  getProofBuilder(): ProofBuilder {
    return this.proofBuilder;
  }

  getMerkleSyncer(): MerkleSyncer {
    return this.merkleSyncer;
  }
}