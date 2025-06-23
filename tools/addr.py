#!/usr/bin/env python3
import hashlib
import sys
import argparse

# from pycoin.coins.bitcoin.networks import BitcoinTestnet, BitcoinMainnet

from pycoin.networks.registry import network_for_netcode

from pycoin.symbols.xtn import network as testnet_network

from pycoin.symbols.btc import network as mainnet_network

import client

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--testnet', action='store_true')
    parser.add_argument('address', nargs='+')
    args = parser.parse_args()

    if args.testnet:
        port = 60002
        network = testnet_network
    else:
        port = 50001
        network = mainnet_network        

    conn = client.Connection(('localhost', port))
    for addr in args.address:
        script = network.parse.address(addr).script()
        print('Address {} has script {}'.format(addr, script.hex()))
        script_hash = hashlib.sha256(script).digest()[::-1].hex()
        reply = conn.call('blockchain.scripthash.get_balance', script_hash)
        result = reply['result']
        print('{} has {} satoshis'.format(addr, result))


if __name__ == '__main__':
    main()
