from stellar_sdk import Server

server = Server(horizon_url="https://horizon-testnet.stellar.org")
account_id = "GAZGIEXTFUKW32FXKGKLTWJKB5AE4J2CGNOELDEBI3RHRXXYU2H6AYSM"

account = server.accounts().account_id(account_id).call()
print(account)