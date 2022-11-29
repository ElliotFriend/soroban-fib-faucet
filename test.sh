#!/bin/bash
CONTRACT_ID=a108

cargo build --target wasm32-unknown-unknown --release

soroban invoke \
    --wasm target/wasm32-unknown-unknown/release/fibonacci_faucet.wasm \
    --id $CONTRACT_ID \
    --account GBEWVFTBMUEJ3XAUYGSPDTHMJQEJP7BEGG26NV2TYKQQIT5DS4HPLOA7 \
    --fn init \
    --arg 10

soroban invoke \
    --id $CONTRACT_ID \
    --fn signups

soroban invoke \
    --id $CONTRACT_ID \
    --account GDESAP6GN5PZDAWSMLILYFXHFUP72GXYUH57FORREKE37LWB7SMLBXAY \
    --fn signup

soroban invoke \
    --id $CONTRACT_ID \
    --account GCMNPBXWEIH2OYBOFP4YYHBAS44LKN2ZBJBEO6O6K7NSM7R3ZIABGAFH \
    --fn signup

soroban invoke \
    --id $CONTRACT_ID \
    --account GCYDLITOV37A4YU2W6WJIMIHAEBV67U76SVCIJXQH7TYJOTKFG7VFO2G \
    --fn signup

soroban invoke \
    --id $CONTRACT_ID \
    --account GB2HZF23DZB4Y4F4U5SULSSNYBZIZRUAXWIWRGBOUNBDO7O3BZXJIZKP \
    --fn signup

soroban invoke \
    --id $CONTRACT_ID \
    --account GCGMUWNF2PB2TL27UJQGTE7D2UFOYZ5ACUK3PRXX7UXWTNZWXWHE5EP6 \
    --fn signup

soroban invoke \
    --id $CONTRACT_ID \
    --account GC63OUTMRMBZV7BNKZNV3AUHAJZSXJ4DY2JHZ4GKNG646RIE7MWBXWF4 \
    --fn signup

soroban invoke \
    --id $CONTRACT_ID \
    --account GDGNUQ2WD3CKBBX2RDAAIHP7F6PAB6UE4T3LWLPOQU3HEX4QDHUUTWAL \
    --fn signup

soroban invoke \
    --id $CONTRACT_ID \
    --account GBQWKZETOLFT2H3QCBFICUEMWNGERJ5VCUPBG5ZK56QNSGYT5HZPUD3V \
    --fn signup

soroban read \
    --id $CONTRACT_ID