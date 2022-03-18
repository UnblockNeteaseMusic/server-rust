const UNM = require('..');

const ctx = { enableFlac: true };

async function main() {
    const searchResult = await UNM.batchSearch(Object.values(UNM.Engine), {
        id: "12345",
        name: "青花瓷",
        artists: [
            {
                id: "114514",
                name: "周杰伦",
            },
        ],
    }, { enableFlac: true });
    console.log(searchResult);
    
    const retrieveResult = await UNM.retrieve(searchResult, ctx);
    console.log(retrieveResult);
}

main();