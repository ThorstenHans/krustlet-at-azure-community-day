#!/bin/bash

aksName=aks-wasm
rgName=rg-aks-wasm
az aks nodepool add --cluster-name $aksName -g $rgName -n wasmpool -c 1 --workload-runtime wasmwasi

