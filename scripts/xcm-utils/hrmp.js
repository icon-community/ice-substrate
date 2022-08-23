// Import the API
const { WsProvider, ApiPromise } = require('@polkadot/api')
require('@polkadot/api-augment')
const { blake2AsU8a } = require('@polkadot/util-crypto')
const { BN, stringToU8a, bnToU8a, u8aConcat, u8aToHex } = require('@polkadot/util')
const { decodeAddress, encodeAddress, Keyring } = require('@polkadot/keyring')

const EMPTY_U8A_32 = new Uint8Array(32)
const XCM_FEE = 2500000000000000

const relayWs = "ws://127.0.0.1:9955"

async function fund(api, toAddress, amount) {
  const keyring = new Keyring({ type: 'sr25519' })
  const sender = keyring.addFromUri('//Charlie')
 
  let nonce = await nextNonce(api, sender)
  console.log(`Fund address ${toAddress} ${amount}`)

  const transfer = api.tx.balances.transfer(toAddress, amount)
  const hash = await transfer.signAndSend(sender)
}

const createAddress = (id) =>
  encodeAddress(u8aConcat(stringToU8a(`modl${id}`), EMPTY_U8A_32).subarray(0, 32))

const get_parachain_soveriegn_account = (paraId) =>
  encodeAddress(
    u8aConcat(stringToU8a('para'), bnToU8a(paraId, 32, true), EMPTY_U8A_32).subarray(0, 32)
  )

 const getApi = async (endpoint) => {  
    return ApiPromise.create({
      provider: new WsProvider(endpoint)
    })
  }
  
 const getRelayApi = async (endpoint) => {  
    return ApiPromise.create({
      provider: new WsProvider(endpoint)
    })
  }
  
const nextNonce = async (api, signer) => {
    return await api.rpc.system.accountNextIndex(signer.address)
}
  
const createXcm = (encoded, refundAccount) => {
  return {
    V2: [
      {
        WithdrawAsset: [
          {
            id: {
              Concrete: {
                parents: 0,
                interior: 'Here'
              }
            },
            fun: {
              Fungible: XCM_FEE
            }
          }
        ]
      },
      {
        BuyExecution: {
          fees: {
            id: {
              Concrete: {
                parents: 0,
                interior: 'Here'
              }
            },
            fun: {
              Fungible: XCM_FEE
            }
          },
          weightLimit: 'Unlimited'
        }
      },
      {
        Transact: {
          originType: 'Native',
          requireWeightAtMost: '20000000000',
          call: {
            encoded
          }
        }
      },
      'RefundSurplus',
      {
        DepositAsset: {
          assets: {
            Wild: {
              AllOf: {
                id: {
                  Concrete: {
                    parents: 0,
                    interior: 'Here'
                  }
                },
                fun: 'Fungible'
              }
            }
          },
          maxAssets: 1,
          beneficiary: {
            parents: 0,
            interior: {
              X1: {
                AccountId32: {
                  network: 'Any',
                  id: u8aToHex(decodeAddress(refundAccount))
                }
              }
            }
          }
        }
      }
    ]
  }
}

async function open(sender, recipient, paraWs = "ws://127.0.0.1:9944") {   
  const relayApi = await getRelayApi(relayWs.toString())
  const api = await getApi(paraWs.toString())

  const Alice = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'
  const signer = new Keyring({ type: 'sr25519' }).addFromUri(
    `${process.env.PARA_CHAIN_SUDO_KEY || '//Alice'}`
   )
 
  console.log(get_parachain_soveriegn_account(sender))

  const configuration = await relayApi.query.configuration.activeConfig()

  const encoded = relayApi.tx.hrmp
    .hrmpInitOpenChannel(
      recipient,
      configuration.hrmpChannelMaxCapacity,
      configuration.hrmpChannelMaxMessageSize
    )
    .toHex()

  console.log("Encoded hrmpInitOpenChannel request: ", encoded, configuration.hrmpChannelMaxCapacity, configuration.hrmpChannelMaxMessageSize.toNumber())

  const proposal = api.tx.polkadotXcm.send(
    {
      V1: {
        parents: 1,
        interior: 'Here'
      }
    },
    createXcm(`0x${encoded.slice(6)}`, get_parachain_soveriegn_account(sender))
  )

  const tx = api.tx.sudo.sudo(proposal)

  await tx.signAndSend(signer, { nonce: await nextNonce(api, signer) })
        .then(() => process.exit(0))
        .catch(err => {
          logger.error(err.message)
          process.exit(1)
        })   

}

async function accept(sender, recipient, paraWs = "ws://127.0.0.1:9988") {
  const relayApi = await getRelayApi(relayWs.toString())
  const api = await getApi(paraWs.toString())

  const count = await relayApi.query.hrmp.hrmpOpenChannelRequestCount(sender)
  console.log("Open channel requests ", count.toNumber()) 
  
  if (count < 1) {
    return;
  }

  const Alice = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'
  const signer = new Keyring({ type: 'sr25519' }).addFromUri(
    `${process.env.PARA_CHAIN_SUDO_KEY || '//Alice'}`
   )
 

  const encoded = relayApi.tx.hrmp.hrmpAcceptOpenChannel(sender).toHex()

  console.log("Encoded hrmpAcceptOpenChannel request: ", encoded, sender)

  const proposal = api.tx.polkadotXcm.send(
    {
      V1: {
        parents: 1,
        interior: 'Here'
      }
    },
    createXcm(`0x${encoded.slice(6)}`, get_parachain_soveriegn_account(recipient))
  )
  
  const tx = api.tx.sudo.sudo(proposal)

  await tx.signAndSend(signer, { nonce: await nextNonce(api, signer) })
        .then(() => process.exit(0))
        .catch(err => {
          logger.error(err.message)
          process.exit(1)
        })  

}

async function main() {  
  const relayWs = "ws://127.0.0.1:9955"
  const api = await getApi(relayWs)
  
  //console.log( await api.query.hrmp.hrmpChannels({"sender":3015, "recepient": 2000}) )
  // Fund parachains soveriegn account on relay chain
  await fund(api, get_parachain_soveriegn_account(2000), new BN(9*1e15)) // *
  await fund(api, get_parachain_soveriegn_account(3015), new BN(9*1e15))
  await fund(api, get_parachain_soveriegn_account(2001), new BN(9*1e15)) // *
  
  await open(3015, 2000, "ws://127.0.0.1:9944")
  await open(2001, 2000, "ws://127.0.0.1:9944") // *
  // wait for hrmpInitOpenChannel notification
  await accept(3015, 2000,  "ws://127.0.0.1:9988")
  await accept(2001, 2000,  "ws://127.0.0.1:9988") // *
  
  await open(2001, 3015, "ws://127.0.0.1:9988")
  await open(2000, 2001, "ws://127.0.0.1:9988") // *
  await accept(2000, 3015, "ws://127.0.0.1:9944")
  await accept(2000, 2001, "ws://127.0.0.1:9944") // *
  
  process.exit(0);
}

main().catch(console.error);
