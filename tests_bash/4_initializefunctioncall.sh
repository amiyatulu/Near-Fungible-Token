set -e
cd ..
export TOKEN=`cat neardev/dev-account`
near call $TOKEN new '{"owner_id":"avrit.testnet", "total_supply":"1000000000000000000"}' --accountId amiyatulu.testnet
