// @ts-nocheck
import * as cosmwasm from '@cosmjs/cosmwasm-stargate';
import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing';
import { GasPrice } from '@cosmjs/stargate';

export default async (argv, { decryptMnemonic }) => {
  const mnemonic = decryptMnemonic(argv.ENCRYPTED_MNEMONIC);
  const [prefix, contractAddress, report_hash] = [process.env.PREFIX, argv.address, argv.reportHash];
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    prefix
  });
  const [firstAccount] = await wallet.getAccounts();
  const client = await cosmwasm.SigningCosmWasmClient.connectWithSigner(process.env.RPC_URL, wallet, {
    gasPrice: GasPrice.fromString(`${process.env.GAS_PRICES}${prefix}`),
    prefix
  });

  await client.execute(
    firstAccount.address,
    contractAddress,
    {
      update_proof: {
        report_hash,
        ai_provider: 'Oraichain'
      }
    },
    'auto'
  );

  console.log(await client.queryContractSmart(contractAddress, { proof: { report_hash } }));
};
