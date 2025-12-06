# Python example

This is a minimal Python example that demonstrates using `solana-py` to query the RPC version and fetch a balance for a public key.

Prerequisites:
- Python 3.9+
- `pip` available

Run locally (PowerShell):

```powershell
cd examples/python-example
python -m venv .venv
.\.venv\Scripts\Activate.ps1
python -m pip install -r requirements.txt
python src/example.py
```

Or using the repo root `requirements.txt`:

```powershell
cd d:\Project\SDK-DATA\zeris-sdk
python -m venv examples\python-example\.venv
.\examples\python-example\.venv\Scripts\Activate.ps1
python -m pip install -r requirements.txt
python examples\python-example\src\example.py
```

You can set a custom RPC endpoint by exporting `SOLANA_RPC` environment variable.
