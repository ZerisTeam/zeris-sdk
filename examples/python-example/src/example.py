from solana.rpc.api import Client
from solana.publickey import PublicKey
import os


def main():
    """Simple example: query RPC version and get balance for a sample pubkey."""
    rpc = os.getenv("SOLANA_RPC", "https://api.devnet.solana.com")
    client = Client(rpc)

    version = client.get_version()
    print("RPC version:", version)

    # Example public key (System Program placeholder)
    pubkey = PublicKey("11111111111111111111111111111111")
    balance = client.get_balance(pubkey)
    print(f"Balance for {pubkey}: {balance}")


if __name__ == "__main__":
    main()
