ACCOUNT_ID := devgenerate.testnet
OWNER_CONTRACT_ID := pray.devgenerate.testnet
COLLECTION_CONTRACT_ID := collection.pray.devgenerate.testnet
LOCATION_CONTRACT_ID := location.pray.devgenerate.testnet
COLLECTION_DIR := "../../generative-art-nft/output/edition test"
COLLECTION_CID := QmQskW3RWhbiYyebrgJTAA6BwkUcSuxbmAMKyVQbo27zRq


test:
	(cd character-contract; cargo test $(case))

build:
	yarn build

reproducible_build:
	docker run \
		--mount type=bind,source=`pwd`,target=/host \
		--cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
		-i -t nearprotocol/contract-builder \
		bash -c " \
			cd /host && rustup target add wasm32-unknown-unknown && \
			(cd ./character-contract && ./build.sh) && \
			(cd ./location-contract && ./build.sh)"

reset:
	near call $(COLLECTION_CONTRACT_ID) drop_state '{}' --accountId $(OWNER_CONTRACT_ID) --gas=290000000000000
	near delete $(COLLECTION_CONTRACT_ID) $(OWNER_CONTRACT_ID)
	near create-account $(COLLECTION_CONTRACT_ID) --masterAccount $(OWNER_CONTRACT_ID) --initialBalance 10
	near delete $(LOCATION_CONTRACT_ID) $(OWNER_CONTRACT_ID)
	near create-account $(LOCATION_CONTRACT_ID) --masterAccount $(OWNER_CONTRACT_ID) --initialBalance 10

deploy: build
	near deploy \
		--wasmFile out/character.wasm \
		--accountId $(COLLECTION_CONTRACT_ID) \
		--initFunction "new_default_meta" \
		--initArgs '{"owner_id": "'$(OWNER_CONTRACT_ID)'", "collection_size": 1}'
	near deploy \
		--wasmFile out/location.wasm \
		--accountId $(LOCATION_CONTRACT_ID) \
		--initFunction "new" \
		--initArgs '{"owner_id": "'$(OWNER_CONTRACT_ID)'", "name": "Abandoned Ruins", "rate": 1}'

update: build
	near deploy --force \
		--wasmFile out/character.wasm \
		--accountId $(COLLECTION_CONTRACT_ID)
	near deploy --force \
		--wasmFile out/location.wasm \
		--accountId $(LOCATION_CONTRACT_ID)

prepare_metadata:
	(cd scripts; poetry install; poetry run python prepare_metadata.py --dir=$(COLLECTION_DIR) --cid=$(COLLECTION_CID) --batch-size=250)

add_metadata: prepare_metadata
	for file in $(shell ls scripts/out/$(COLLECTION_CID)) ; do \
		encrypted_metadata=$$(cat scripts/out/$(COLLECTION_CID)/$$file); \
		near call \
			$(COLLECTION_CONTRACT_ID) \
			append_encrypted_metadata \
			'{"encrypted_metadata": "'$$encrypted_metadata'"}' \
			--accountId $(OWNER_CONTRACT_ID) \
			--gas=290000000000000; \
	done
	near call $(COLLECTION_CONTRACT_ID) set_collection_state '{"collection_state": "Published"}' --accountId $(OWNER_CONTRACT_ID)

mint:
	near call $(COLLECTION_CONTRACT_ID) nft_mint '{"receiver_id": "'$(ACCOUNT_ID)'"}' --accountId $(ACCOUNT_ID) --amount 0.1

reveal:
	set -e; \
	continue=true; \
	while [ $$continue = true ]; do \
		continue=$$(near call $(COLLECTION_CONTRACT_ID) reveal '{"password": "password"}' --accountId $(OWNER_CONTRACT_ID) --gas=290000000000000 | tail -1); \
		echo $$continue; \
	done
	near call $(COLLECTION_CONTRACT_ID) set_collection_state '{"collection_state": "Revealed"}' --accountId $(OWNER_CONTRACT_ID)

enter_location:
	near call $(LOCATION_CONTRACT_ID) enter '{"character_id": "0"}' --accountId $(ACCOUNT_ID)

leave_location:
	near call $(LOCATION_CONTRACT_ID) leave '{"character_id": "0"}' --accountId $(ACCOUNT_ID)
