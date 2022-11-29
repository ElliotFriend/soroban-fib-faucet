# Fibonacci Faucet Notes

## Local Network Setup

```bash
export SOROBAN_SECRET_KEY=SAO7ZJOBGFYBK3H5GVE6VXCIZKTFU77JRQGWUKTLWH74SQT3LJEWHAHZ
export SOROBAN_RPC_URL=http://localhost:8000/soroban/rpc
export SOROBAN_NETWORK_PASSPHRASE='Standalone Network ; February 2017'
```

```bash
unset SOROBAN_SECRET_KEY
unset SOROBAN_RPC_URL
unset SOROBAN_NETWORK_PASSPHRASE
```

## Administrator Account

Public Key GCMC6ANNFIW2TFK7EQKE7NSMSHB434QX2GN3TYEXUVYOOIGXOJVZSKF3
Secret Key SAO7ZJOBGFYBK3H5GVE6VXCIZKTFU77JRQGWUKTLWH74SQT3LJEWHAHZ

http :8000/friendbot?addr=GCMC6ANNFIW2TFK7EQKE7NSMSHB434QX2GN3TYEXUVYOOIGXOJVZSKF3

## Other Funded Accounts

Public Key GBWRGE4BFYM7DUQ7X7ZFFB6SWRHYA7APRGCVPHOPGJ3PEANPEIQ3BSNB
Secret Key SCEFH2S77ZSQLHUUIGBYURQSEPMJVT67FM5RWG3HQJRNTTIAWG7KLZUG

Public Key GBFBUXLDG5WUWBQ5BCRDESLCCDSTX5VT3EXY7V5VFEPCDKDCV2NS4ZLL
Secret Key SAN5T5DZR6XGPWUDJ5TWN2PLQ4TH2TLWCGZAEOZYJKXN27BX7ERRFLCK

Public Key GAAFWNDP2WCTZXFL4V477QRVQM3CE7A2S3SWPQIE2WSC2QJA3YG6QWQ6
Secret Key SDTB6EEQLYJODPBQPXRZUGRTFZXI2S4SWWCTPKZQIX6U24F3CO2X35AL

Public Key GBFW4XI6WNQOTMTXLLLJXXS66CJMRORDWSMQZGCLZBDBEWEBR7HM7QZR
Secret Key SBGK6CA4XDF7YWE2AXDVA4WKS5L5KIOLB26IMQ4WSIVOIIPV2W6632O5

Public Key GBDOIIGAFMD5YIEJ7PVHBDOXJG5L4X5Q3HFTY6XIDFILDF3V5ITEKDK2
Secret Key SDTFX2FT3XYN6FVPH44DZ4H4YSI7IW2TMT2SELTWDFX5YOEUCOZFYLEL

```bash
http :8000/friendbot?addr=GBWRGE4BFYM7DUQ7X7ZFFB6SWRHYA7APRGCVPHOPGJ3PEANPEIQ3BSNB && \
http :8000/friendbot?addr=GBFBUXLDG5WUWBQ5BCRDESLCCDSTX5VT3EXY7V5VFEPCDKDCV2NS4ZLL && \
http :8000/friendbot?addr=GAAFWNDP2WCTZXFL4V477QRVQM3CE7A2S3SWPQIE2WSC2QJA3YG6QWQ6 && \
http :8000/friendbot?addr=GBFW4XI6WNQOTMTXLLLJXXS66CJMRORDWSMQZGCLZBDBEWEBR7HM7QZR && \
http :8000/friendbot?addr=GBDOIIGAFMD5YIEJ7PVHBDOXJG5L4X5Q3HFTY6XIDFILDF3V5ITEKDK2
```

## Creating Token Contract

```bash
soroban token create \
    --admin GCMC6ANNFIW2TFK7EQKE7NSMSHB434QX2GN3TYEXUVYOOIGXOJVZSKF3 \
    --decimal 0 \
    --name "Fibonacci Faucet" \
    --symbol "FIB" \
    --salt 0112358132134558914423337761098715972584418167651094617711286570
```

contractId: `a0f324cf3da5bc707965b9c2d5930c0ac77660103e94b1f9b51deedc577ef75f`

## Reading Contract Data

```bash
soroban read \
    --id a0f324cf3da5bc707965b9c2d5930c0ac77660103e94b1f9b51deedc577ef75f
```

## Mint `FIB` Asset to an Account

```bash
# This works!! :grin:
soroban invoke \
    --wasm target/wasm32-unknown-unknown/release/fibonacci_faucet.wasm \
    --id a0f324cf3da5bc707965b9c2d5930c0ac77660103e94b1f9b51deedc577ef75f \
    --account GCMC6ANNFIW2TFK7EQKE7NSMSHB434QX2GN3TYEXUVYOOIGXOJVZSKF3 \
    --fn mint \
    --arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
    --arg 0 \
    --arg '{"object":{"vec":[{"symbol":"Account"},{"object":{"accountId":{"publicKeyTypeEd25519":"6d1313812e19f1d21fbff25287d2b44f807c0f8985579dcf3276f201af2221b0"}}}]}}' \
    --arg 100
```

```bash
soroban invoke \
    --id a0f324cf3da5bc707965b9c2d5930c0ac77660103e94b1f9b51deedc577ef75f \
    --account GBWRGE4BFYM7DUQ7X7ZFFB6SWRHYA7APRGCVPHOPGJ3PEANPEIQ3BSNB \
    --fn balance \
    --arg '{"object":{"vec":[{"symbol":"Account"},{"object":{"accountId":{"publicKeyTypeEd25519":"6d1313812e19f1d21fbff25287d2b44f807c0f8985579dcf3276f201af2221b0"}}}]}}'
```

## Deploy the FibFaucet Contract

```bash
soroban deploy \
    --wasm target/wasm32-unknown-unknown/release/fibonacci_faucet.wasm
```

contractId: `0e3881bfa1a260496e2b47d86f537021fe294b7388e4b8ea9a94088ecb6b1b59`

## Make the Faucet a Token Admin?

```bash
soroban invoke \
    --id a0f324cf3da5bc707965b9c2d5930c0ac77660103e94b1f9b51deedc577ef75f \
    --account GCMC6ANNFIW2TFK7EQKE7NSMSHB434QX2GN3TYEXUVYOOIGXOJVZSKF3 \
    --fn set_admin \
    --arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
    --arg 0 \
    --arg '{"object":{"vec":[{"symbol":"Contract"},{"object":{"bytes":"0e3881bfa1a260496e2b47d86f537021fe294b7388e4b8ea9a94088ecb6b1b59"}}]}}'
```

## Invoke the Faucet's `gimme` Function

```bash
soroban invoke \
    --id 0e3881bfa1a260496e2b47d86f537021fe294b7388e4b8ea9a94088ecb6b1b59 \
    --fn gimme \
    --arg a0f324cf3da5bc707965b9c2d5930c0ac77660103e94b1f9b51deedc577ef75f \
    --arg '{"object":{"vec":[{"symbol":"Account"},{"object":{"accountId":{"publicKeyTypeEd25519":"4a1a5d63376d4b061d08a232496210e53bf6b3d92f8fd7b5291e21a862ae9b2e"}}}]}}' \
    --arg 75
```
