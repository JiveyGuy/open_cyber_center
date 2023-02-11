const language_core_1 = require("@volar/language-core");
const presetInitialIndentBrackets = {
    json: ['{', '}'],
    jsonc: ['{', '}'],
    html: ['<template>', '</template>'],
    markdown: ['<template>', '</template>'],
};
const plugin = () => {
    return {
        version: 1,
        getEmbeddedFileNames(fileName, sfc) {
            const names = [];
            for (let i = 0; i < sfc.customBlocks.length; i++) {
                const customBlock = sfc.customBlocks[i];
                names.push(fileName + '.customBlock_' + customBlock.type + '_' + i + '.' + customBlock.lang);
            }
            return names;
        },
        resolveEmbeddedFile(_fileName, sfc, embeddedFile) {
            const match = embeddedFile.fileName.match(/^(.*)\.customBlock_([^_]+)_(\d+)\.([^.]+)$/);
            if (match) {
                const index = parseInt(match[3]);
                const customBlock = sfc.customBlocks[index];
                embeddedFile.capabilities = Object.assign(Object.assign({}, language_core_1.FileCapabilities.full), { documentFormatting: {
                        initialIndentBracket: presetInitialIndentBrackets[customBlock.lang],
                    } });
                embeddedFile.content.push([
                    customBlock.content,
                    customBlock.name,
                    0,
                    language_core_1.FileRangeCapabilities.full,
                ]);
            }
        },
    };
};
module.exports = plugin;
//# sourceMappingURL=vue-sfc-customblocks.js.map