import { PublicKey } from '@solana/web3.js';

export interface ProofData {
  proof: Uint8Array;
  publicSignals: Uint8Array;
}

export interface ZerisState {
  lastProofHash: Uint8Array;
  merkleRoot: Uint8Array;
  proofCount: number;
  authority: PublicKey;
}

export interface SubmitProofEvent {
  proofHash: Uint8Array;
  submitter: PublicKey;
}

export interface ProofVerifiedEvent {
  proofHash: Uint8Array;
  isValid: boolean;
}

export interface MerkleRootUpdatedEvent {
  newRoot: Uint8Array;
}

export interface MerkleRootValidatedEvent {
  rootHash: Uint8Array;
  isValid: boolean;
}

export type ZerisEvent =
  | SubmitProofEvent
  | ProofVerifiedEvent
  | MerkleRootUpdatedEvent
  | MerkleRootValidatedEvent;