import json
import urllib.request

ORGS = {
    "marinade": ["marinade-finance"],
    "lido": ["lidofinance"],
    "helius": ["helius-labs"],
    "triton": ["triton-one", "Triton-One", "tritonone"],
    "anza": ["anza-xyz"],
    "figment": ["figment-networks", "FigmentNetworks", "figment"],
    "saga": ["solana-mobile"],
    "firedancer": ["firedancer-io"],
}

for stem, candidates in ORGS.items():
    for org in candidates:
        try:
            req = urllib.request.Request(
                "https://api.github.com/orgs/" + org,
                headers={"User-Agent": "solana-mahjong"},
            )
            with urllib.request.urlopen(req, timeout=20) as r:
                d = json.load(r)
            print(stem, "|", org, "|", d.get("avatar_url"))
            break
        except Exception as e:
            print(stem, "|", org, "| ERROR", e)
