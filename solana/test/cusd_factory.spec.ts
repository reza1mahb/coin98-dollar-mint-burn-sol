import {
  Connection,
  Keypair,
  PublicKey
} from '@solana/web3.js'
import { SolanaConfigService } from '@coin98/solana-support-library/config'
import { BN } from 'bn.js'

describe('chainlink_dfeed_local_test', function() {

  const PROGRAM_ID = new PublicKey('CFvHYH4afBtK97rAwKkZtpnEQGqx8AmS6SWmYZd6JdmE')
  const CHAINLINK_PROGRAM_ID = new PublicKey('HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny')

  const connection = new Connection('http://localhost:8899', 'confirmed')
  let defaultAccount: Keypair

  before(async function() {
    defaultAccount = await SolanaConfigService.getDefaultAccount()
  })
})
