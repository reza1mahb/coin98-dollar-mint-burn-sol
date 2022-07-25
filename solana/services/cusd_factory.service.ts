import {
  HashService,
  sendTransaction,
  SolanaService,
  TokenProgramService
} from '@coin98/solana-support-library';
import {
  Connection,
  Keypair,
  PublicKey,
  Transaction
} from '@solana/web3.js';
import BN from 'bn.js';
import {
  Burner,
  CusdFactoryInstructionService,
  InputTokenPair,
  InputTokenParams,
  Minter,
  OutputTokenPair,
  OutputTokenParams
} from './cusd_factory_instruction.service';

export class CusdFactoryService {

  static async createMinter(
    connection: Connection,
    payerAccount: Keypair,
    name: string,
    isActive: boolean,
    inputParams: InputTokenParams[],
    feePercent: number,
    totalMintedLimit: BN,
    perPeriodMintedLimit: BN,
    cusdFactoryProgramId: PublicKey,
  ): Promise<PublicKey> {

    const derivationPath = HashService.sha256(name).slice(0, 8)

    const transaction = new Transaction()

    const createMinterInstruction = CusdFactoryInstructionService.createMinter(
      payerAccount.publicKey,
      derivationPath,
      cusdFactoryProgramId,
    )
    transaction.add(createMinterInstruction)

    const [minterAddress,] = this.findMinterAddress(
      derivationPath,
      cusdFactoryProgramId,
    )
    const setMinterInstruction = CusdFactoryInstructionService.setMinter(
      payerAccount.publicKey,
      minterAddress,
      isActive,
      inputParams,
      feePercent,
      totalMintedLimit,
      perPeriodMintedLimit,
      cusdFactoryProgramId,
    )
    transaction.add(setMinterInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ])
    console.info(`Created Minter ${minterAddress.toBase58()}`, '---', txSign, '\n')
    return minterAddress
  }

  static async setMinter(
    connection: Connection,
    payerAccount: Keypair,
    minterAddress: PublicKey,
    isActive: boolean,
    inputParams: InputTokenParams[],
    feePercent: number,
    totalMintedLimit: BN,
    perPeriodMintedLimit: BN,
    cusdFactoryProgramId: PublicKey,
  ): Promise<boolean> {

    const transaction = new Transaction()

    const setMinterInstruction = CusdFactoryInstructionService.setMinter(
      payerAccount.publicKey,
      minterAddress,
      isActive,
      inputParams,
      feePercent,
      totalMintedLimit,
      perPeriodMintedLimit,
      cusdFactoryProgramId,
    )
    transaction.add(setMinterInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ])
    console.info(`Updated Minter ${minterAddress.toBase58()}`, '---', txSign, '\n')
    return true
  }

  static async createBurner(
    connection: Connection,
    payerAccount: Keypair,
    name: string,
    isActive: boolean,
    outputParams: OutputTokenParams,
    feePercent: number,
    totalBurnedLimit: BN,
    perPeriodBurnedLimit: BN,
    cusdFactoryProgramId: PublicKey,
  ): Promise<PublicKey> {

    const derivationPath = HashService.sha256(name).slice(0, 8)

    const transaction = new Transaction()

    const createBurnerInstruction = CusdFactoryInstructionService.createBurner(
      payerAccount.publicKey,
      derivationPath,
      cusdFactoryProgramId,
    )
    transaction.add(createBurnerInstruction)

    const [burnerAddress,] = this.findBurnerAddress(
      derivationPath,
      cusdFactoryProgramId,
    )
    const setMinterInstruction = CusdFactoryInstructionService.setBurner(
      payerAccount.publicKey,
      burnerAddress,
      isActive,
      outputParams,
      feePercent,
      totalBurnedLimit,
      perPeriodBurnedLimit,
      cusdFactoryProgramId,
    )
    transaction.add(setMinterInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ])
    console.info(`Created Burner ${burnerAddress.toBase58()}`, '---', txSign, '\n')
    return burnerAddress
  }

  static async setBurner(
    connection: Connection,
    payerAccount: Keypair,
    burnerAddress: PublicKey,
    isActive: boolean,
    outputParams: OutputTokenParams,
    feePercent: number,
    totalBurnedLimit: BN,
    perPeriodBurnedLimit: BN,
    cusdFactoryProgramId: PublicKey,
  ): Promise<boolean> {

    const transaction = new Transaction()

    const setBurnerInstruction = CusdFactoryInstructionService.setBurner(
      payerAccount.publicKey,
      burnerAddress,
      isActive,
      outputParams,
      feePercent,
      totalBurnedLimit,
      perPeriodBurnedLimit,
      cusdFactoryProgramId,
    )
    transaction.add(setBurnerInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ])
    console.info(`Updated Burner ${burnerAddress.toBase58()}`, '---', txSign, '\n')
    return true
  }

  static async mint(
    connection: Connection,
    payerAccount: Keypair,
    minterAddress: PublicKey,
    cusdTokenMintAddress: PublicKey,
    amount: BN,
    userCusdTokenAddress: PublicKey,
    chainlinkProgramId: PublicKey,
    cusdFactoryProgramId: PublicKey,
  ): Promise<boolean> {

    const [rootSignerAddress,] = CusdFactoryInstructionService.findRootSignerAddress(
      cusdFactoryProgramId,
    )
    const minterInfo = await this.getMinterAccountInfo(
      connection,
      minterAddress,
    )

    const transaction = new Transaction()

    const inputTokens: InputTokenPair[] = minterInfo.inputTokens.map((tokenMintAddress, index) => {
      const priceFeedAddress = minterInfo.inputPriceFeeds[index]
      const poolTokenAddress = TokenProgramService.findAssociatedTokenAddress(
        rootSignerAddress,
        tokenMintAddress,
      )
      const userTokenAddress = TokenProgramService.findAssociatedTokenAddress(
        payerAccount.publicKey,
        tokenMintAddress,
      )
      return <InputTokenPair>{
        priceFeedAddress,
        poolTokenAddress,
        userTokenAddress,
      }
    })

    const mintInstruction = CusdFactoryInstructionService.mint(
      payerAccount.publicKey,
      minterAddress,
      cusdTokenMintAddress,
      inputTokens,
      amount,
      userCusdTokenAddress,
      chainlinkProgramId,
      cusdFactoryProgramId,
    )
    transaction.add(mintInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ])
    console.info(`Minted ${amount.toString()} CUSD`, '---', txSign, '\n')
    return true
  }

  static async burn(
    connection: Connection,
    payerAccount: Keypair,
    burnerAddress: PublicKey,
    cusdTokenMintAddress: PublicKey,
    userCusdTokenAddress: PublicKey,
    amount: BN,
    userTokenAddress: PublicKey,
    chainlinkProgramId: PublicKey,
    cusdFactoryProgramId: PublicKey,
  ): Promise<boolean> {

    const [rootSignerAddress,] = CusdFactoryInstructionService.findRootSignerAddress(
      cusdFactoryProgramId,
    )
    const burnerInfo = await this.getBurnerAccountInfo(
      connection,
      burnerAddress,
    )

    const transaction = new Transaction()

    const poolTokenAddress = TokenProgramService.findAssociatedTokenAddress(
      rootSignerAddress,
      burnerInfo.outputToken,
    )
    const outputToken: OutputTokenPair = {
      priceFeedAddress: burnerInfo.outputPriceFeed,
      poolTokenAddress,
      userTokenAddress,
    }

    const burnInstruction = CusdFactoryInstructionService.burn(
      payerAccount.publicKey,
      burnerAddress,
      cusdTokenMintAddress,
      userCusdTokenAddress,
      amount,
      outputToken,
      chainlinkProgramId,
      cusdFactoryProgramId,
    )
    transaction.add(burnInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ])
    console.info(`Burned ${amount.toString()} CUSD`, '---', txSign, '\n')
    return true
  }

  static async withdrawToken(
    connection: Connection,
    payerAccount: Keypair,
    poolTokenAddress: PublicKey,
    recipientAddress: PublicKey,
    amount: BN,
    cusdFactoryProgramId: PublicKey,
  ): Promise<boolean> {

    const transaction = new Transaction()

    const payerTokenAccountInfo = await TokenProgramService.getTokenAccountInfo(
      connection,
      poolTokenAddress,
    )
    let [recipientTokenAddress, createATAInstruction] = await TokenProgramService.findRecipientTokenAddress(
      connection,
      payerAccount.publicKey,
      recipientAddress,
      payerTokenAccountInfo.mint,
    )
    if(createATAInstruction) {
      transaction.add(createATAInstruction)
    }

    const withdrawInstruction = CusdFactoryInstructionService.withdrawToken(
      payerAccount.publicKey,
      poolTokenAddress,
      recipientTokenAddress,
      amount,
      cusdFactoryProgramId,
    )
    transaction.add(withdrawInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ])
    console.info(`Withdraw ${amount.toString()} tokens`, '---', txSign, '\n')
    return true
  }

  static async unlockTokenMint(
    connection: Connection,
    payerAccount: Keypair,
    tokenMintAddress: PublicKey,
    cusdFactoryProgramId: PublicKey,
  ): Promise<boolean> {

    const transaction = new Transaction()

    const unlockTokenMintInstruction = CusdFactoryInstructionService.unlockTokenMint(
      payerAccount.publicKey,
      tokenMintAddress,
      cusdFactoryProgramId,
    )
    transaction.add(unlockTokenMintInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ])
    console.info(`Token Mint ${tokenMintAddress.toBase58()}'s authority unlocked to owner address`, '---', txSign, '\n')
    return true
  }

  static async initAppData(
    connection: Connection,
    payerAccount: Keypair,
    limit: number,
    cusdFactoryProgramId: PublicKey,
  ): Promise<boolean> {

    const transaction = new Transaction()

    const [appDataAddress,] = CusdFactoryInstructionService.findAppDataAddress(
      cusdFactoryProgramId,
    )

    if (await SolanaService.isAddressAvailable(connection, appDataAddress)) {
      const createAppDataInstruction = CusdFactoryInstructionService.createAppData(
        payerAccount.publicKey,
        cusdFactoryProgramId,
      )
      transaction.add(createAppDataInstruction)
    }

    const setAppDataInstruction = CusdFactoryInstructionService.setAppData(
      payerAccount.publicKey,
      limit,
      cusdFactoryProgramId,
    )
    transaction.add(setAppDataInstruction)

    const txSign = await sendTransaction(connection, transaction, [
      payerAccount,
    ])
    console.info(`Updated AppData at ${appDataAddress.toBase58()}`, '---', txSign, '\n')
    return true
  }

  static async getMinterAccountInfo(
    connection: Connection,
    minterAddress: PublicKey,
  ): Promise<Minter> {
    const accountInfo = await connection.getAccountInfo(minterAddress)
    const data = CusdFactoryInstructionService.decodeMinterData(accountInfo.data)
    return data
  }

  static async getBurnerAccountInfo(
    connection: Connection,
    burnerAddress: PublicKey,
  ): Promise<Burner> {
    const accountInfo = await connection.getAccountInfo(burnerAddress)
    const data = CusdFactoryInstructionService.decodeBurnerData(accountInfo.data)
    return data
  }

  static findAppDataAddress(
    cusdFactoryProgramId: PublicKey,
  ): [PublicKey, number] {
    return CusdFactoryInstructionService.findAppDataAddress(
      cusdFactoryProgramId,
    )
  }

  static findRootSignerAddress(
    cusdFactoryProgramId: PublicKey,
  ): [PublicKey, number] {
    return CusdFactoryInstructionService.findRootSignerAddress(
      cusdFactoryProgramId,
    )
  }

  static findMinterAddress(
    params: string | Buffer,
    cusdFactoryProgramId: PublicKey,
  ): [PublicKey, number] {
    const derivationPath = (typeof(params) === 'string')
      ? HashService.sha256(params).slice(0, 8)
      : params
    return CusdFactoryInstructionService.findMinterAddress(
      derivationPath,
      cusdFactoryProgramId,
    )
  }

  static findBurnerAddress(
    params: string | Buffer,
    cusdFactoryProgramId: PublicKey,
  ): [PublicKey, number] {
    const derivationPath = (typeof(params) === 'string')
      ? HashService.sha256(params).slice(0, 8)
      : params
    return CusdFactoryInstructionService.findBurnerAddress(
      derivationPath,
      cusdFactoryProgramId,
    )
  }
}
