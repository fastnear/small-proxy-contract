# Small proxy contract

A small contract to proxy calls from the owner's account ID.

The size of the proxy contract is only `1769` bytes.
With account state and owner's account ID state, the total size is `1942` bytes.

It allows deploying proxy contracts for less than `0.02` NEAR!

## Features

The proxy contract only respects calls from the owner. Usually, it's a contract with more logic than the proxy itself.
The input to the contract is following Borsh schema, so it's easy to implement logic on the factory contracts.

For this example we take JSON input, then serialize it to Borsh, and pass it to the proxy contract.

## Usage

Examples given for the testnet (based on near-cli-rs).

Create a new owner's account and create a new proxy contract:

```bash
export FACTORY_ACCOUNT_ID=small-proxy-factory.testnet
export CURRENT_TIMESTAMP=$(date +%s)
export PROXY_ACCOUNT_ID=$CURRENT_TIMESTAMP.$FACTORY_ACCOUNT_ID
export OWNER_ACCOUNT_ID=$CURRENT_TIMESTAMP-proxy-owner-tester.testnet

# Create owner's account
near account create-account sponsor-by-faucet-service $OWNER_ACCOUNT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

# Awaiting for keychain to sync
sleep 5

# Deploy proxy contract from the factory
near contract call-function as-transaction $FACTORY_ACCOUNT_ID deploy_proxy json-args '{"proxy_account_id": "'$PROXY_ACCOUNT_ID'"}' prepaid-gas '15.0 Tgas' attached-deposit '0.02 NEAR' sign-as $OWNER_ACCOUNT_ID network-config testnet sign-with-keychain send
```

## Example

Call your proxy to ping aurora pool on testnet:

```bash
near contract call-function as-transaction $FACTORY_ACCOUNT_ID call_proxy json-args '{"proxy_account_id": "'$PROXY_ACCOUNT_ID'", "input": [{
  "account_id": "aurora.pool.f863973.m0",
  "actions": [
    {
      "FunctionCall": {
        "method_name": "ping",
        "args": "e30=",
        "deposit": "0",
        "gas": "50000000000000"
      }
    }
  ]
}]}' prepaid-gas '60.0 Tgas' attached-deposit '0 NEAR' sign-as $OWNER_ACCOUNT_ID network-config testnet sign-with-keychain send
```

## Development

Recompile proxy contract locally (needs `nightly`):

```bash
small-proxy/build.sh
```

Recompile proxy contract locally:

```bash
small-proxy-factory/build.sh
```

Or you can compile with Docker:

```bash
./build_docker.sh
```

Create a new account and deploy the proxy factory:

```bash
export FACTORY_ACCOUNT_ID=$(date +%s)-proxy-factory.testnet

# Create factory account
near account create-account sponsor-by-faucet-service $FACTORY_ACCOUNT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

# Awaiting for keychain to sync
sleep 5

# Deploy
near contract deploy $FACTORY_ACCOUNT_ID use-file small-proxy-factory/res/small_proxy_factory.wasm with-init-call init json-args {} prepaid-gas '10.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
```

In case you need to re-deploy:

```bash
# Re-deploy
near contract deploy $FACTORY_ACCOUNT_ID use-file small-proxy-factory/res/small_proxy_factory.wasm without-init-call network-config testnet sign-with-keychain send
```
