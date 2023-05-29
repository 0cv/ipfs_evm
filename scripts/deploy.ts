import { ethers } from "hardhat";
import { writeFile } from 'node:fs/promises';

async function main() {
    try {
        const CIDStore = await ethers.getContractFactory("CIDStore");
        const cidstore = await CIDStore.deploy();
        await cidstore.deployed();

        console.log("Contract deployed to:", cidstore.address);

        await writeFile(".contract-address.env", cidstore.address);
    } catch (err) {
        console.log("error deploying:", err);
    }
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
