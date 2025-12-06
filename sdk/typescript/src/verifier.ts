import { Connection, PublicKey } from '@solana/web3.js';
import { ZerisState } from './types';

export class MerkleSyncer {
  private connection: Connection;
  private programId: PublicKey;

  constructor(connection: Connection, programId: PublicKey) {
    this.connection = connection;
    this.programId = programId;
  }

  async syncMerkleRoot(): Promise<Uint8Array> {
    const state = await this.getZerisState();
    return state.merkleRoot;
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

    // Parse account data
    const data = accountInfo.data;
    return {
      lastProofHash: data.slice(8, 40),
      merkleRoot: data.slice(40, 72),
      proofCount: data.readBigUInt64LE(72),
      authority: new PublicKey(data.slice(80, 112)),
    };
  }

  async validateLocalMerkleRoot(localRoot: Uint8Array): Promise<boolean> {
    const onChainRoot = await this.syncMerkleRoot();
    return this.compareRoots(localRoot, onChainRoot);
  }

  private compareRoots(root1: Uint8Array, root2: Uint8Array): boolean {
    if (root1.length !== root2.length) return false;
    for (let i = 0; i < root1.length; i++) {
      if (root1[i] !== root2[i]) return false;
    }
    return true;
  }
}