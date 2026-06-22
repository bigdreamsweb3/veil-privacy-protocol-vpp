import { Connection, PublicKey, TransactionSignature } from '@solana/web3.js';

export type AssetVerificationStatus = 'unverified' | 'verified' | 'paused' | 'deprecated';

export interface VeilWallet {
  publicKey: PublicKey;
  signTransaction?: unknown;
  signAllTransactions?: unknown;
}

export interface VeilSDKConfig {
  connection: Connection;
  wallet: VeilWallet;
  programId?: PublicKey;
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
  }

  async wrap(amount: number, underlyingMint: PublicKey): Promise<TransactionSignature> {
    this.assertPositiveAmount(amount);
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    throw new VeilSDKError('wrap is an interface stub pending approved program IDL integration.');
  }

  async unwrap(amount: number, underlyingMint: PublicKey): Promise<TransactionSignature> {
    this.assertPositiveAmount(amount);
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    throw new VeilSDKError('unwrap is an interface stub pending approved program IDL integration.');
  }

  async transferVAsset(
    to: PublicKey,
    amount: number,
    underlyingMint: PublicKey,
  ): Promise<TransactionSignature> {
    this.assertPublicKey(to, 'recipient');
    this.assertPositiveAmount(amount);
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    throw new VeilSDKError('transferVAsset is an interface stub pending Token-2022 confidential transfer integration.');
  }

  async getPrivateBalance(owner: PublicKey, underlyingMint: PublicKey): Promise<number> {
    this.assertPublicKey(owner, 'owner');
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    throw new VeilSDKError('getPrivateBalance requires confidential balance decryption support.');
  }

  async getProtectedBalance(owner: PublicKey, underlyingMint: PublicKey): Promise<number> {
    return this.getPrivateBalance(owner, underlyingMint);
  }

  async getVaultInfo(underlyingMint: PublicKey): Promise<VaultInfo> {
    this.assertPublicKey(underlyingMint, 'underlyingMint');
    throw new VeilSDKError('getVaultInfo is an interface stub pending account fetch implementation.');
  }

  isOfficialVAsset(vaultInfo: VaultInfo, expectedVAssetMint: PublicKey): boolean {
    return (
      vaultInfo.verificationStatus === 'verified' &&
      vaultInfo.vassetMint.equals(expectedVAssetMint) &&
      vaultInfo.isBackedOneToOne
    );
  }

  private assertPositiveAmount(amount: number): void {
    if (!Number.isFinite(amount) || amount <= 0) {
      throw new VeilSDKError('Amount must be a positive finite number.');
    }
  }

  private assertPublicKey(value: PublicKey, name: string): void {
    if (!(value instanceof PublicKey)) {
      throw new VeilSDKError(`${name} must be a Solana PublicKey.`);
    }
  }
}
