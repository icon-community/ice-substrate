## Parity Signer

This is the recommended tool for safely managing accounts and signing transanctions on Polkadot networks.&#x20;

Parity signer turns your mobile phone into cold storage, and enables transaction signing in air-gapped mode.

### Create new account

For first-time setup and creating a new account see instructions [here](https://support.polkadot.network/support/solutions/articles/65000180512)

> Backup your seed phrase on paper or use [Banana Split](https://github.com/paritytech/banana\_split) for maximum security

### Adding ICE networks&#x20;

For Arctic Testnet and other custom networks, we need to load the network spec and metadata using signed QR using the scanner.\
\
Scan the QR below to add Arctic testnet&#x20;

![add_specs_arctic-testnet-sr25519](https://user-images.githubusercontent.com/1651925/177788715-96040777-8d57-4342-a4b6-010f575fdae0.png)

Download the animated QR below and open it with an image viewer. This is a multipart data scan, scroll continuously through the images until the scan is complete.

![load_metadata_arctic-testnetV1](https://user-images.githubusercontent.com/1651925/177788759-dee20d4c-a4f7-4fc3-a366-226308b3c1fc.png)

### Add New Network

This section will walk you through adding new network spec with Parity Signer on OS or Android and Rust command line to sign and load network metadata.&#x20;

Parity-signer maintains a hot database of signed network endpoints published at [metadata portal](https://metadata.parity.io/). The following is a summary of the commands used to add the Arctic network, for a complete guide follow this [tutorial](https://paritytech.github.io/parity-signer/tutorials/Start.html) instead.

#### Verify existing network

```
argo run show -specs arctic-testnet-sr25519
```

#### Generate spec file from an existing network

```
cargo run add_specs -f -n arctic-testnet-sr25519
```

#### New network spec

```
cargo run add_specs -u wss://arctic-rpc.icenetwork.io -sr25519
```

This will generate unsigned spec file under \`parity-signer/rust/files/in\_progress'

#### Sign network spec

```
cat <spec-file> | subkey sign --suri <seed-phrase-and-derivation>
e.g. // e.g.
cat ../files/in_progress/sign_me_add_specs_arctic-testnet_sr25519 | subkey sign --suri "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice"
```

This will return a `<signature>` you need to make a signed QR.



> TODO Complete netwwork setup instrudctions
