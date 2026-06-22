import { Connection, PublicKey, TransactionSignature } from '@solana/web3.js';

export type AssetVerificationStatus = 'unverified' | 'verified' | 'paused' | 'deprecated';
export type SupportedAssetMint = PublicKey;

export interface VeilWallet {
  publicKey: PublicKey;
  signTransaction?: unknown;
  signAllTransactions?: unknown;
}

export interface VaultInfo {
  underlyingMint: PublicKey;
  vassetMint: PublicKey;
  reserveVault: PublicKey;
  totalLocked: bigint;
  totalVAssetMinted: bigint;
  isBackedOneToOne: boolean;
  verificationStatus: AssetVerificationStatus;
}

export interface ConfidentialTransferProofData {
  commitment: Uint8Array;
  token2022ProofInstructionData?: Uint8Array;
}

export interface WrapParams {
  amount: bigint | number;
  underlyingMint: PublicKey;
}

export interface UnwrapParams {
  amount: bigint | number;
  underlyingMint: PublicKey;
}

export interface TransferVAssetParams {
  to: PublicKey;
  amount: bigint | number;
  underlyingMint: PublicKey;
  proof: ConfidentialTransferProofData;
}

export interface VeilInstructionAdapter {
  wrap(params: WrapParams): Promise<TransactionSignature>;
  unwrap(params: UnwrapParams): Promise<TransactionSignature>;
  transferVAsset(params: TransferVAssetParams): Promise<TransactionSignature>;
  getVaultInfo(underlyingMint: PublicKey): Promise<VaultInfo>;
  getPrivateBalance(owner: PublicKey, underlyingMint: PublicKey): Promise<number>;
}

export interface VeilSDKConfig {
  connection: Connection;
  wallet: VeilWallet;
  programId?: PublicKey;
  instructions?: VeilInstructionAdapter;
}

export class VeilSDKError extends Error {
  constructor(message: string, readonly cause?: unknown) {
    super(message);
    this.name = 'VeilSDKError';
  }
}

export class VeilSDK {
  readonly connection: Connection;
  readonly wallet: VeilWallet;
  readonly programId?: PublicKey;
  private readonly instructions?: VeilInstructionAdapter;

  constructor(config: VeilSDKConfig) {
    if (!config.connection) {
      throw new VeilSDKError('A Solana connection is required.');
    }
    if (!config.wallet?.publicKey) {
      throw new VeilSDKError('A wallet with a publicKey is required.');
    }

    this.connection = config.connection;
    this.wallet = config.wallet;
    this.programId = config.programId;
    this.instructions = config.instructions;
  }

  async wrap(amount: number, underlyingMint: SupportedAssetMint): Promise<TransactionSignature> {
    const normalizedAmount = this.normalizeAmount(amount);
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    return this.requiredInstructions().wrap({ amount: normalizedAmount, underlyingMint });
  }

  async unwrap(amount: number, underlyingMint: SupportedAssetMint): Promise<TransactionSignature> {
    const normalizedAmount = this.normalizeAmount(amount);
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    return this.requiredInstructions().unwrap({ amount: normalizedAmount, underlyingMint });
  }

  async transferVAsset(
    to: PublicKey,
    amount: number,
    underlyingMint: SupportedAssetMint,
    proof: ConfidentialTransferProofData,
  ): Promise<TransactionSignature> {
    this.assertPublicKey(to, 'recipient');
    const normalizedAmount = this.normalizeAmount(amount);
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    this.assertCommitment(proof.commitment);
    return this.requiredInstructions().transferVAsset({
      to,
      amount: normalizedAmount,
      underlyingMint,
      proof,
    });
  }

  async getPrivateBalance(owner: PublicKey, underlyingMint: SupportedAssetMint): Promise<number> {
    this.assertPublicKey(owner, 'owner');
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    return this.requiredInstructions().getPrivateBalance(owner, underlyingMint);
  }

  async getProtectedBalance(owner: PublicKey, underlyingMint: SupportedAssetMint): Promise<number> {
    return this.getPrivateBalance(owner, underlyingMint);
  }

  async getVaultInfo(underlyingMint: SupportedAssetMint): Promise<VaultInfo> {
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    return this.requiredInstructions().getVaultInfo(underlyingMint);
  }

  isOfficialVAsset(vaultInfo: VaultInfo, expectedVAssetMint: PublicKey): boolean {
    return (
      vaultInfo.verificationStatus === 'verified' &&
      vaultInfo.vassetMint.equals(expectedVAssetMint) &&
      vaultInfo.isBackedOneToOne
    );
  }

  private requiredInstructions(): VeilInstructionAdapter {
    if (!this.instructions) {
      throw new VeilSDKError(
        'VeilSDK requires a VeilInstructionAdapter generated from the deployed Anchor IDL before transactions can be sent.',
      );
    }
    return this.instructions;
  }

  private normalizeAmount(amount: number): bigint {
    if (!Number.isFinite(amount) || amount <= 0 || !Number.isSafeInteger(amount)) {
      throw new VeilSDKError('Amount must be a positive safe integer in base units.');
    }
    return BigInt(amount);
  }

  private assertPublicKey(value: PublicKey, name: string): void {
    if (!(value instanceof PublicKey)) {
      throw new VeilSDKError(`${name} must be a Solana PublicKey.`);
    }
  }

  private assertCommitment(commitment: Uint8Array): void {
    if (!(commitment instanceof Uint8Array) || commitment.length !== 32) {
      throw new VeilSDKError('Confidential transfer commitment must be 32 bytes.');
    }
  }
}
