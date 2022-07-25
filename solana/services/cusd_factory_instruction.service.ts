import {
  HashService,
  TokenProgramInstructionService,
  TOKEN_PROGRAM_ID
} from '@coin98/solana-support-library'
import { BorshCoder, Idl } from '@project-serum/anchor'
import {
  AccountMeta,
  PublicKey,
  SystemProgram,
  TransactionInstruction
} from '@solana/web3.js'
import BN from 'bn.js'
import CusdFactoryIdl from '../target/idl/coin98_dollar_mint_burn.json'

const coder = new BorshCoder(CusdFactoryIdl as Idl)

// Requests
interface CreateMinterRequest {
  derivationPath: Buffer
}

interface SetMinterRequest {
  isActive: boolean
  inputTokens: PublicKey[]
  inputDecimals: number[]
  inputPercentages: number[]
  inputPriceFeeds: PublicKey[]
  feePercent: number
  totalMintedLimit: BN
  perPeriodMintedLimit: BN
}

interface CreateBurnerRequest {
  derivationPath: Buffer
}

interface SetBurnerRequest {
  isActive: Boolean
  outputToken: PublicKey
  outputDecimals: number
  outputPriceFeed: PublicKey
  feePercent: number
  totalBurnedLimit: BN
  perPeriodBurnedLimit: BN
}

interface MintRequest {
  amount: BN
  extraInstructions: Buffer
}

interface BurnRequest {
  amount: BN
}

interface WithdrawTokenRequest {
  amount: BN
}

interface UnlockTokenMintRequest {
}

interface CreateAppDataRequest {
}

interface SetAppDataRequest {
  limit: number
}

// Accounts
export interface AppData {
  nonce: number
  signerNonce: number
  limit: number
}

export interface Minter {
  nonce: number
  isActive: boolean
  inputTokens: PublicKey[]
  inputDecimals: number[]
  inputPercentages: number[]
  inputPriceFeeds: PublicKey[]
  feePercent: number
  accumulatedFee: BN
  totalMintedAmount: BN
  totalMintedLimit: BN
  perPeriodMintedAmount: BN
  perPeriodMintedLimit: BN
  lastPeriodTimestamp: BN
}

export interface Burner {
  nonce: number
  isActive: boolean
  outputToken: PublicKey
  outputDecimals: number
  outputPriceFeed: PublicKey
  feePercent: number
  accumulatedFee: BN
  totalBurnedAmount: BN
  totalBurnedLimit: BN
  perPeriodBurnedAmount: BN
  perPeriodBurnedLimit: BN
  lastPeriodTimestamp: BN
}

// Helpers

export interface InputTokenPair {
  priceFeedAddress: PublicKey
  poolTokenAddress: PublicKey
  userTokenAddress: PublicKey
}

export interface InputTokenParams {
  tokenAddress: PublicKey
  decimals: number
  percentage: number
  priceFeedAddress: PublicKey
}

export interface OutputTokenPair {
  priceFeedAddress: PublicKey
  poolTokenAddress: PublicKey
  userTokenAddress: PublicKey
}

export interface OutputTokenParams {
  tokenAddress: PublicKey
  decimals: number
  priceFeedAddress: PublicKey
}

// RPC
export class CusdFactoryInstructionService {

  static createMinter(
    rootAddress: PublicKey,
    derivationPath: Buffer,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    const request: CreateMinterRequest = {
      derivationPath,
    }

    const data = coder.instruction.encode('createMinter', request)

    const [minterAddress,] = this.findMinterAddress(
      derivationPath,
      cusdFactoryProgramId,
    )

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: true },
      <AccountMeta>{ pubkey: minterAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static setMinter(
    rootAddress: PublicKey,
    minterAddress: PublicKey,
    isActive: boolean,
    inputParams: InputTokenParams[],
    feePercent: number,
    totalMintedLimit: BN,
    perPeriodMintedLimit: BN,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    const inputTokens = inputParams.map(param => param.tokenAddress)
    const inputDecimals = inputParams.map(param => param.decimals)
    const inputPercentages = inputParams.map(param => param.percentage)
    const inputPriceFeeds = inputParams.map(param => param.priceFeedAddress)
    const request: SetMinterRequest = {
      isActive,
      inputTokens,
      inputDecimals,
      inputPercentages,
      inputPriceFeeds,
      feePercent,
      totalMintedLimit,
      perPeriodMintedLimit,
    }

    const data = coder.instruction.encode('setMinter', request)

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: minterAddress, isSigner: false, isWritable: true },
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static createBurner(
    rootAddress: PublicKey,
    derivationPath: Buffer,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    const request: CreateBurnerRequest = {
      derivationPath,
    }

    const data = coder.instruction.encode('createBurner', request)

    const [burnerAddress,] = this.findBurnerAddress(
      derivationPath,
      cusdFactoryProgramId,
    )

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: true },
      <AccountMeta>{ pubkey: burnerAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static setBurner(
    rootAddress: PublicKey,
    burnerAddress: PublicKey,
    isActive: Boolean,
    outputParams: OutputTokenParams,
    feePercent: number,
    totalBurnedLimit: BN,
    perPeriodBurnedLimit: BN,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    const request: SetBurnerRequest = {
      isActive,
      outputToken: outputParams.tokenAddress,
      outputDecimals: outputParams.decimals,
      outputPriceFeed: outputParams.priceFeedAddress,
      feePercent,
      totalBurnedLimit,
      perPeriodBurnedLimit,
    }

    const data = coder.instruction.encode('setBurner', request)

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: burnerAddress, isSigner: false, isWritable: true },
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static mint(
    userAddress: PublicKey,
    minterAddress: PublicKey,
    cusdTokenMintAddress: PublicKey,
    inputTokens: InputTokenPair[],
    amount: BN,
    userCusdTokenAddress: PublicKey,
    chainlinkProgramId: PublicKey,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    let extraAccounts: AccountMeta[] = []
    let extraInstructions: number[] = []
    for (const inputToken of inputTokens) {
      const index1 = extraAccounts.findIndex(meta => meta.pubkey.toBase58() === inputToken.priceFeedAddress.toBase58())
      if (index1 === -1) {
        extraInstructions.push(extraAccounts.length)
        extraAccounts.push(
          <AccountMeta>{ pubkey: inputToken.priceFeedAddress, isSigner: false, isWritable: false },
        )
      }
      else {
        extraInstructions.push(index1)
      }

      const index2 = extraAccounts.findIndex(meta => meta.pubkey.toBase58() === inputToken.userTokenAddress.toBase58())
      if (index2 === -1) {
        extraInstructions.push(extraAccounts.length)
        extraAccounts.push(
          <AccountMeta>{ pubkey: inputToken.userTokenAddress, isSigner: false, isWritable: true },
        )
      }
      else {
        extraInstructions.push(index2)
      }

      const index3 = extraAccounts.findIndex(meta => meta.pubkey.toBase58() === inputToken.poolTokenAddress.toBase58())
      if (index3 === -1) {
        extraInstructions.push(extraAccounts.length)
        extraAccounts.push(
          <AccountMeta>{ pubkey: inputToken.poolTokenAddress, isSigner: false, isWritable: true },
        )
      }
      else {
        extraInstructions.push(index3)
      }
    }

    const request: MintRequest = {
      amount,
      extraInstructions: Buffer.from(extraInstructions),
    }

    const data = coder.instruction.encode('mint', request)

    const [appDataAddress,] = this.findAppDataAddress(
      cusdFactoryProgramId,
    )
    const [rootSignerAddress,] = this.findRootSignerAddress(
      cusdFactoryProgramId,
    )

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: userAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: appDataAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: rootSignerAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: cusdTokenMintAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: minterAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: userCusdTokenAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: chainlinkProgramId, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      ...extraAccounts
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static burn(
    userAddress: PublicKey,
    burnerAddress: PublicKey,
    cusdTokenMintAddress: PublicKey,
    userCusdTokenAddress: PublicKey,
    amount: BN,
    outputToken: OutputTokenPair,
    chainlinkProgramId: PublicKey,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    let extraAccounts: AccountMeta[] = [
      <AccountMeta>{ pubkey: outputToken.priceFeedAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: outputToken.poolTokenAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: outputToken.userTokenAddress, isSigner: false, isWritable: true },
    ]

    const request: BurnRequest = {
      amount,
    }

    const data = coder.instruction.encode('burn', request)

    const [appDataAddress,] = this.findAppDataAddress(
      cusdFactoryProgramId,
    )
    const [rootSignerAddress,] = this.findRootSignerAddress(
      cusdFactoryProgramId,
    )
    const poolCusdTokenAddress = TokenProgramInstructionService.findAssociatedTokenAddress(
      rootSignerAddress,
      cusdTokenMintAddress,
    )

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: userAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: appDataAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: rootSignerAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: cusdTokenMintAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: burnerAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: poolCusdTokenAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: userCusdTokenAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: chainlinkProgramId, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      ...extraAccounts
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static withdrawToken(
    rootAddress: PublicKey,
    poolTokenAddress: PublicKey,
    recipientTokenAddress: PublicKey,
    amount: BN,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    const request: WithdrawTokenRequest = {
      amount,
    }

    const data = coder.instruction.encode('withdrawToken', request)

    const [appDataAddress,] = this.findAppDataAddress(
      cusdFactoryProgramId,
    )
    const [rootSignerAddress,] = this.findRootSignerAddress(
      cusdFactoryProgramId,
    )

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: appDataAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: rootSignerAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: poolTokenAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: recipientTokenAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static unlockTokenMint(
    rootAddress: PublicKey,
    tokenMintAddress: PublicKey,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    const request: UnlockTokenMintRequest = { }

    const data = coder.instruction.encode('unlockTokenMint', request)

    const [appDataAddress,] = this.findAppDataAddress(
      cusdFactoryProgramId,
    )
    const [rootSignerAddress,] = this.findRootSignerAddress(
      cusdFactoryProgramId,
    )

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: appDataAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: rootSignerAddress, isSigner: false, isWritable: false },
      <AccountMeta>{ pubkey: tokenMintAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static createAppData(
    rootAddress: PublicKey,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    const request: CreateAppDataRequest = { }

    const data = coder.instruction.encode('createAppData', request)

    const [appDataAddress,] = this.findAppDataAddress(
      cusdFactoryProgramId,
    )

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: true },
      <AccountMeta>{ pubkey: appDataAddress, isSigner: false, isWritable: true },
      <AccountMeta>{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static setAppData(
    rootAddress: PublicKey,
    limit: number,
    cusdFactoryProgramId: PublicKey,
  ): TransactionInstruction {

    const request: SetAppDataRequest = {
      limit,
     }

    const data = coder.instruction.encode('setAppData', request)

    const [appDataAddress,] = this.findAppDataAddress(
      cusdFactoryProgramId,
    )

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: rootAddress, isSigner: true, isWritable: false },
      <AccountMeta>{ pubkey: appDataAddress, isSigner: false, isWritable: true },
    ]

    return new TransactionInstruction({
      data,
      keys,
      programId: cusdFactoryProgramId,
    })
  }

  static decodeAppDataData(
    data: Buffer
  ): AppData {
    return coder.accounts.decode('AppData', data)
  }

  static decodeMinterData(
    data: Buffer
  ): Minter {
    return coder.accounts.decode('Minter', data)
  }

  static decodeBurnerData(
    data: Buffer
  ): Burner {
    return coder.accounts.decode('Burner', data)
  }

  static findAppDataAddress(
    cusdFactoryProgramId: PublicKey,
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        HashService.sha256('Program').slice(0, 8),
        HashService.sha256('AppData').slice(0, 8),
      ],
      cusdFactoryProgramId,
    )
  }

  static findRootSignerAddress(
    cusdFactoryProgramId: PublicKey,
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        HashService.sha256('Signer').slice(0, 8),
        HashService.sha256('Root').slice(0, 8),
      ],
      cusdFactoryProgramId,
    )
  }

  static findMinterAddress(
    derivationPath: Buffer,
    cusdFactoryProgramId: PublicKey,
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        HashService.sha256('Minter').slice(0, 8),
        derivationPath,
      ],
      cusdFactoryProgramId,
    )
  }

  static findBurnerAddress(
    derivationPath: Buffer,
    cusdFactoryProgramId: PublicKey,
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        HashService.sha256('Burner').slice(0, 8),
        derivationPath,
      ],
      cusdFactoryProgramId,
    )
  }
}
