# Logo sources & brand notes

Game use is nominative / fan use in a free educational game. Marks remain
trademarks of their owners. Do not imply endorsement or partnership.

## Masters already fetched (tools/logo_src/)

| Tile | Source (official brand kit) | File used | Terms |
|------|-----------------------------|-----------|-------|
| SOL | Solana branding page https://solana.com/branding | `sol/mark.svg` (official logomark, rendered to 256px with resvg) | No shadows/outlines/stretch; keep contrast |
| RAY | Raydium docs repo https://github.com/raydium-io/raydium-docs-v1 (`/logo/`), rules at https://docs.raydium.io/resources/brand-kit | `ray/mark-test.png` (`logo/raydium-r.png`, transparent) | Never recolor; light/dark master per background; clear space = R height; min symbol 24px |
| JUP | Jupiter brand kit https://github.com/jup-ag/docs/tree/main/static/files/brand-kit (`developers.jup.ag/docs/resources/brand-kit`) | `jup/kit/JupiterTokens/Token-256x256.svg` (JUP token mark) | Label APIs correctly per kit |
| ORCA | Orca brand folder https://docs.orca.so/reference/brand (Google Drive) | `orca/logomark.svg` (official logomark) | Do not edit/distort/recolor without permission |

## Stylization (tools/stylize_logos.py)

- Marks are never stretched or recolored — uniform scale + center only.
- Badge canvas 256x256 transparent; theme-color disc R92 + darker ring
  (colors from src/theme/solana.rs); white inner disc R66 for line marks
  (SOL, RAY); coin marks placed standalone (JUP, ORCA).
- Previews go to tools/preview/; final run with `--apply` overwrites
  assets/tiles/<stem>.png (same names as src/renderer/tiles.rs expects).

## Batch 2 — tokens (CoinGecko, coin page images)

| Tile | CoinGecko ID | File used |
|------|--------------|-----------|
| USDC | usd-coin | `cg/usdc.png` (official Circle mark) |
| USDT | tether | `cg/usdt.png` (Tether mark, white-disc mode — diamond shape) |
| BONK | bonk | `cg/bonk.jpg` (photo-style mark, circular crop) |
| WIF | dogwifcoin | `cg/wif.jpg` (token photo, circular crop) |
| PYTH | pyth-network | `cg/pyth.png` (circular crop, opaque source) |
| JTO / Jito | jito-governance-token | `cg/jto.png` (same Jito brand for both tiles; captions differ) |

Fetched via `https://api.coingecko.com/api/v3/coins/markets` (free public API),
raw response kept at `tools/logo_src/cg-tokens.json`.

## Batch 2 — companies/validators (GitHub org avatars, org-controlled)

| Tile | Org | Avatar |
|------|-----|--------|
| Marinade | marinade-finance | teal coin, white wave mark |
| Lido | lidofinance | blue droplet mark (circular crop) |
| Helius | helius-labs | orange sun mark (circular crop) |
| Anza | anza-xyz | dark geometric mark (circular crop) |
| Saga | solana-mobile | Solana Mobile wordmark (circular crop) |
| Firedancer | firedancer-io | teal flame on navy coin |

Fallbacks (org has no custom avatar — GitHub identicon, unusable):
Figment, Triton → keep monogram badges from `gen_unified_logos.py`.

## Still to fetch (batch 2)

USDC (Circle brand), USDT (Tether brand), BONK, WIF, PYTH, JTO/Jito,
Saga, Firedancer, Marinade, Lido, Rocket*, Figment, Anza, Triton, Helius.
Generic concepts stay as monogram badges: Swap, Stake, Lend, Yield,
Bridge, DAO, NFT, Mint, Pool, Farm, Token2022, Program, Account, Block,
Vote, Wallet.

(*) Rocket Pool is Ethereum-centric — candidate for replacement.
