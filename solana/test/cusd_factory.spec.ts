import {
  ChainlinkDfeedService
} from '@coin98/chainlink-dfeed-js'
import {
  SolanaService, SystemProgramService, TokenProgramService
} from '@coin98/solana-support-library'
import {
  SolanaConfigService,
  TestAccountService
} from '@coin98/solana-support-library/config'
import {
  Connection,
  Keypair,
  PublicKey
} from '@solana/web3.js'
import { BN } from 'bn.js'
import { CusdFactoryService } from '../services/cusd_factory.service'
import { InputTokenParams, OutputTokenParams } from '../services/cusd_factory_instruction.service'

describe('chainlink_dfeed_local_test', function() {

  const PROGRAM_ID = new PublicKey('CFvHYH4afBtK97rAwKkZtpnEQGqx8AmS6SWmYZd6JdmE')
  const CHAINLINK_DFEED_PROGRAM_ID = new PublicKey('HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny')

  const connection = new Connection('http://localhost:8899', 'confirmed')
  let defaultAccount: Keypair
  let testAccount1: Keypair
  let c98TokenAccount: Keypair
  let cusdTokenAccount: Keypair
  let usdcTokenAccount: Keypair
  const [c98PriceFeedAddress,] = ChainlinkDfeedService.findFeedAddress(
    'C98-USD',
    CHAINLINK_DFEED_PROGRAM_ID,
  )
  const [usdcPriceFeedAddress,] = ChainlinkDfeedService.findFeedAddress(
    'USDC-USD',
    CHAINLINK_DFEED_PROGRAM_ID,
  )
  const usdcOnlyMinterName: string = (Math.random() * 1000).toString()
  const [usdcOnlyMinterAddress,] = CusdFactoryService.findMinterAddress(
    usdcOnlyMinterName,
    PROGRAM_ID,
  )
  const usdcBurnerName: string = (Math.random() * 1000).toString()
  const [usdcBurnerAddress,] = CusdFactoryService.findBurnerAddress(
    usdcBurnerName,
    PROGRAM_ID,
  )

  before(async function() {
    defaultAccount = await SolanaConfigService.getDefaultAccount()
    testAccount1 = await TestAccountService.getAccount(1)
    c98TokenAccount = await TestAccountService.getTokenAccount(2)
    cusdTokenAccount = await TestAccountService.getTokenAccount(1)
    usdcTokenAccount = await TestAccountService.getTokenAccount(3)
    const [rootSignerAddress,] = CusdFactoryService.findRootSignerAddress(
      PROGRAM_ID,
    )
    if(await SolanaService.isAddressAvailable(connection, c98TokenAccount.publicKey)) {
      await TokenProgramService.createTokenMint(
        connection,
        defaultAccount,
        c98TokenAccount,
        6,
        defaultAccount.publicKey,
        null,
      )
    }
    if(await SolanaService.isAddressAvailable(connection, cusdTokenAccount.publicKey)) {
      await TokenProgramService.createTokenMint(
        connection,
        defaultAccount,
        cusdTokenAccount,
        6,
        rootSignerAddress,
        null,
      )
    }
    if(await SolanaService.isAddressAvailable(connection, usdcTokenAccount.publicKey)) {
      await TokenProgramService.createTokenMint(
        connection,
        defaultAccount,
        usdcTokenAccount,
        6,
        defaultAccount.publicKey,
        null,
      )
    }
    if(await SolanaService.isAddressAvailable(connection, c98PriceFeedAddress)) {
      await ChainlinkDfeedService.cteateFeed(
        connection,
        defaultAccount,
        'C98-USD',
        25,
        75,
        'C98-USD',
        8,
        10,
        CHAINLINK_DFEED_PROGRAM_ID,
      )
    }
    if(await SolanaService.isAddressAvailable(connection, usdcPriceFeedAddress)) {
      await ChainlinkDfeedService.cteateFeed(
        connection,
        defaultAccount,
        'USDC-USD',
        25,
        75,
        'USDC-USD',
        6,
        10,
        CHAINLINK_DFEED_PROGRAM_ID,
      )
    }
    // Ensure test account has lamports to invoke contracts
    await SystemProgramService.transfer(
      connection,
      defaultAccount,
      testAccount1.publicKey,
      1000000,
    )
    // Initialize Coin98DollarMintBurn internal state
    await CusdFactoryService.initAppData(
      connection,
      defaultAccount,
      24,
      PROGRAM_ID,
    )
    // Initialize all token account for Coin98DollarMintBurn
    await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      rootSignerAddress,
      cusdTokenAccount.publicKey,
    )
    await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      rootSignerAddress,
      c98TokenAccount.publicKey,
    )
    await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      rootSignerAddress,
      usdcTokenAccount.publicKey,
    )
  })

  it('create USDC only minter', async function() {
    await CusdFactoryService.createMinter(
      connection,
      defaultAccount,
      usdcOnlyMinterName,
      true,
      [
        <InputTokenParams>{
          tokenAddress: usdcTokenAccount.publicKey,
          priceFeedAddress: usdcPriceFeedAddress,
          decimals: 6,
          percentage: 10000,
        },
      ],
      30,
      new BN("1000000000000"),
      new BN("1000000000"),
      PROGRAM_ID,
    )
  })

  it('mint 100 CUSD from 100 USDC', async function() {
    await TokenProgramService.mint(
      connection,
      defaultAccount,
      usdcTokenAccount.publicKey,
      testAccount1.publicKey,
      new BN("100000000"),
    )
    const testAccount1CusdTokenAddress = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      testAccount1,
      testAccount1.publicKey,
      cusdTokenAccount.publicKey,
    )
    await ChainlinkDfeedService.submitFeed(
      connection,
      defaultAccount,
      usdcPriceFeedAddress,
      new BN("1000000"),
      CHAINLINK_DFEED_PROGRAM_ID,
    )
    await CusdFactoryService.mint(
      connection,
      testAccount1,
      usdcOnlyMinterAddress,
      cusdTokenAccount.publicKey,
      new BN("100000000"),
      testAccount1CusdTokenAddress,
      PROGRAM_ID,
    )
  })

  it('create USDC burner', async function() {
    await CusdFactoryService.createBurner(
      connection,
      defaultAccount,
      usdcBurnerName,
      true,
      <OutputTokenParams>{
        tokenAddress: usdcTokenAccount.publicKey,
        priceFeedAddress: usdcPriceFeedAddress,
        decimals: 6,
      },
      30,
      new BN("1000000000000"),
      new BN("1000000000"),
      PROGRAM_ID,
    )
  })
})
