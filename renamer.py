import requests
import hashlib

if __name__ == "__main__":
    from argparse import ArgumentParser

    parser = ArgumentParser()
    parser.add_argument("apiKey")
    parser.add_argument("filePath")

    args = parser.parse_args()

    print(args.apiKey, args.filePath)

    with open(args.filePath, 'r') as f:
        code = f.read()    

    rsp = requests.post(
        "https://renamer.mshq.dev/api/rename",
        headers={
            "User-Agent": "Renamer wrapper for Python",
            "ApiKey": args.apiKey,
            "Content-Type": "application/json"
        },
        json={
            "code": code,
            "hash": hashlib.sha256(code.encode()).digest().hex(),
            "filename": args.filePath.split("/")[-1].split("\\")[-1]
        }
    )
    rsp.raise_for_status()
    rsp = rsp.json()
    with open(args.filePath.replace(".lua", "-renamed.lua"), 'w') as f:
        f.write(rsp["outputCode"])
    print(f"Tokens used: {rsp['totalTokens']}")