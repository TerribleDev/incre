#! /usr/bin/env node
var nativeApp = require('../native');
const fs = require('fs/promises')

interface IConfig {
    include?: string[],
    exclude?: string[]
}
// const INCRE_PATH = "incre.config.json"
async function main() : Promise<void> {
    // try {
    //     await fs.stat(INCRE_PATH);
        
    // } catch {
    //     console.error(`Unable to find ${INCRE_PATH}`);
    //     process.exit(1);
    // }
    // const file = await fs.readFile(INCRE_PATH);
    // const json = JSON.parse(file.toString()) as IConfig;
    // if(!json.include || json.include.length < 1) {
    //     console.error("included files not defined")
    //     process.exit(1);
    // }
    nativeApp.init();
    
}

main();