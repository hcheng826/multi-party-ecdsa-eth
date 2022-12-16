const ethers = require("ethers");
const axios = require("axios");
const { resolveProperties } = require("@ethersproject/properties");
const { serialize } = require("@ethersproject/transactions");
const { splitSignature, hexZeroPad } = require("@ethersproject/bytes");
const { keccak256 } = require("@ethersproject/keccak256");

const local_share = require('./local-share1.json');

function getAddress() {
    return ethers.utils.computeAddress("0x" + Buffer.from(local_share.y_sum_s.point).toString("hex"))
}

const main = async () => {
  const rpcUrl = "https://rpc.ankr.com/eth_goerli";
    // const rpcUrl = "http://localhost:8545";
  const provider = ethers.getDefaultProvider(rpcUrl);

  const tinyValueToSend = 1;
  const txData = {
    type: 2,
    chainId: 5,
    nonce: await provider.getTransactionCount(getAddress()),
    maxPriorityFeePerGas: "0xf1013241",
    maxFeePerGas: "0xf1013241",
    gasLimit: "0x0927c0",
    to: "0xA28B81e10d78a38A9C1D4dD599145355577354f6",
    value: "0x" + tinyValueToSend.toString(16),
  };

  const serializedTx = ethers.utils.serializeTransaction(txData);
  const resolvedTx = await resolveProperties(txData);

  const signature = await axios.post(
    "http://localhost:8001/send-tx",
    keccak256(serializedTx).slice(2),
    {
      headers: {
        "Content-Type": "text/plain",
      },
    }
  );

  const parsedSigature = {
    recoveryParam: signature.data.recid,
    r: hexZeroPad(
      "0x" + Buffer.from(signature.data.r.scalar).toString("hex"),
      32
    ),
    s: hexZeroPad(
      "0x" + Buffer.from(signature.data.s.scalar).toString("hex"),
      32
    ),
  };

  const theSplitSignature = splitSignature(parsedSigature);
  const serializeTx = serialize(resolvedTx, theSplitSignature);

  console.log(ethers.utils.parseTransaction(serializeTx));
  const sentTx = await provider.sendTransaction(serializeTx);
  const rc = await sentTx.wait();
};

main();
