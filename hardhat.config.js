require('dotenv').config()
require("@nomicfoundation/hardhat-toolbox");

module.exports = {
    defaultNetwork: "mumbai",
    networks: {
        mumbai: {
            url: `${process.env.MUMBAI_MATIC_VIGIL_KEY}`,
            accounts: [process.env.WALLET_PRIVATE_KEY]
        }
    },
    solidity: {
        version: "0.8.19",
    },
};