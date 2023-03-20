Object.defineProperty(exports, "__esModule", { value: true });
exports.createLanguageService = void 0;
const base = require("@volar/typescript");
const vue = require("@volar/vue-language-core");
function createLanguageService(host) {
    const languageService = base.createLanguageService(host, vue.createLanguageModules(host.getTypeScriptModule(), host.getCompilationSettings(), vue.resolveVueCompilerOptions(host.getVueCompilationSettings())));
    const getCompletionsAtPosition = languageService.getCompletionsAtPosition;
    languageService.getCompletionsAtPosition = (fileName, position, options) => {
        const result = getCompletionsAtPosition(fileName, position, options);
        if (result) {
            result.entries = result.entries.filter(entry => entry.name.indexOf('__VLS_') === -1);
        }
        return result;
    };
    return languageService;
}
exports.createLanguageService = createLanguageService;
//# sourceMappingURL=index.js.map