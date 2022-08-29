OWNER_CONTRACT_ID := devgenerate.testnet
NFT_CONTRACT_ID := pray.devgenerate.testnet
COLLECTION_DIR := "../../generative-art-nft/output/edition test"
COLLECTION_CID := QmQskW3RWhbiYyebrgJTAA6BwkUcSuxbmAMKyVQbo27zRq


test:
	(cd nft-contract; cargo test $(case))

build:
	yarn build

reset:
	near call $(NFT_CONTRACT_ID) drop_state '{}' --accountId $(NFT_CONTRACT_ID) --gas=290000000000000
	near delete $(NFT_CONTRACT_ID) $(OWNER_CONTRACT_ID)
	near create-account $(NFT_CONTRACT_ID) --masterAccount $(OWNER_CONTRACT_ID) --initialBalance 10

deploy: build
	near deploy \
		--wasmFile out/main.wasm \
		--accountId $(NFT_CONTRACT_ID) \
		--initFunction "new_default_meta" \
		--initArgs '{"owner_id": "'$(NFT_CONTRACT_ID)'", "collection_size": 1}'

update: build
	near deploy --force \
		--wasmFile out/main.wasm \
		--accountId $(NFT_CONTRACT_ID)

prepare_metadata:
	(cd scripts; poetry run python prepare_metadata.py --dir=$(COLLECTION_DIR) --cid=$(COLLECTION_CID) --batch-size=250)

add_metadata: prepare_metadata
	for file in $(shell ls scripts/output/$(COLLECTION_CID)) ; do \
		encrypted_metadata=$$(cat scripts/output/$(COLLECTION_CID)/$$file); \
		near call \
			$(NFT_CONTRACT_ID) \
			append_encrypted_metadata \
			'{"encrypted_metadata": "'$$encrypted_metadata'"}' \
			--accountId $(NFT_CONTRACT_ID) \
			--gas=290000000000000; \
	done
	near call $(NFT_CONTRACT_ID) set_collection_state '{"collection_state": "Published"}' --accountId $(NFT_CONTRACT_ID)

mint:
	near call $(NFT_CONTRACT_ID) nft_mint '{"receiver_id": "'$(OWNER_CONTRACT_ID)'"}' --accountId $(NFT_CONTRACT_ID) --amount 0.1

reveal:
	set -e; \
	continue=true; \
	while [ $$continue = true ]; do \
		continue=$$(near call $(NFT_CONTRACT_ID) reveal '{"password": "password"}' --accountId $(NFT_CONTRACT_ID) --gas=290000000000000 | tail -1); \
		echo $$continue; \
	done
	near call $(NFT_CONTRACT_ID) set_collection_state '{"collection_state": "Revealed"}' --accountId $(NFT_CONTRACT_ID)
