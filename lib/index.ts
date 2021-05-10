#! /usr/bin/env node
var nativeApp = require('../native');

async function main() : Promise<void> {
    nativeApp.init();
    
}

main();