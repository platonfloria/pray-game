# PRAY Game

Welcome to the PRAY universe.
Here you will find the tools necessary to deploy all the smart contracts required to enjoy your epic adventure and discover the secrets of the Red Tower.
Please refer to Makefile for technical details behind actions used in this README.

## Prerequisites

* [NEAR Wallet Account](wallet.testnet.near.org)
* [Rust Toolchain](https://docs.near.org/develop/prerequisites)
* [NEAR-CLI](https://docs.near.org/tools/near-cli#setup)
* [yarn](https://classic.yarnpkg.com/en/docs/install#mac-stable)

# Quick-Start

```=bash
git clone https://github.com/platonfloria/near-nft-collection.git
make build
```

Now that you've cloned and built the contract we can try a few things.

## Collection preparation

Before collection is ready to be minted, we have to deploy the contract and seed it with encrypted metadata.

### Deploy Your Contract

```=bash
make deploy
```

### Redeploy Your Contract

```=bash
make update
```

### Seed medatada

```=bash
make add_metadata
```

### Minting Token

```bash=
make mint
```

After you've minted the token go to wallet.testnet.near.org to `your-account.testnet` and look in the collections tab and check out your new NFT!

## Collection reveal

When entire collection was minted we can reveal it.

### Revealing collection

```bash=
make reveal
```

After collection was revealed go to wallet.testnet.near.org to `your-account.testnet` and look in the collections tab and check out your revealed NFT!

## View NFT Information

After you've minted your NFT you can make a view call to get a response containing the `token_id` `owner_id` and the `metadata`

```bash=
near view $NFT_CONTRACT_ID nft_tokens_for_owner '{"account_id": "`your-account.testnet`"}'
```

## Transfering NFTs

To transfer an NFT go ahead and make another [testnet wallet account](https://wallet.testnet.near.org).

Then run the following
```bash=
MAIN_ACCOUNT_2=your-second-wallet-account.testnet
```

Verify the correct variable names with this
```=bash
echo $NFT_CONTRACT_ID

echo $MAIN_ACCOUNT

echo $MAIN_ACCOUNT_2
```

To initiate the transfer..
```bash=
near call $NFT_CONTRACT_ID nft_transfer '{"receiver_id": "$MAIN_ACCOUNT_2", "token_id": "token-1", "memo": "Go Team :)"}' --accountId $MAIN_ACCOUNT --depositYocto 1
```

In this call you are depositing 1 yoctoNEAR for security and so that the user will be redirected to the NEAR wallet.

## Errata
