import * as vue from '@volar/vue-language-core';
export declare function createLanguageService(host: vue.VueLanguageServiceHost): {
    __internal__: {
        languageService: import("typescript/lib/tsserverlibrary").LanguageService;
        context: {
            typescript: {
                languageServiceHost: import("typescript/lib/tsserverlibrary").LanguageServiceHost;
            };
            virtualFiles: {
                allSources(): import("@volar/language-core").Source[];
                updateSource(fileName: string, snapshot: import("typescript/lib/tsserverlibrary").IScriptSnapshot): import("@volar/language-core").VirtualFile | undefined;
                deleteSource(fileName: string): void;
                getSource(fileName: string): import("@volar/language-core").Source | undefined;
                hasSource: (fileName: string) => boolean;
                getMirrorMap: (file: import("@volar/language-core").VirtualFile) => import("@volar/language-core").MirrorMap | undefined;
                getMaps: (virtualFile: import("@volar/language-core").VirtualFile) => [string, import("@volar/source-map").SourceMap<import("@volar/language-core").FileRangeCapabilities>][];
                hasVirtualFile(fileName: string): boolean;
                getVirtualFile(fileName: string): readonly [import("@volar/language-core").VirtualFile, import("@volar/language-core").Source] | readonly [undefined, undefined];
            };
        };
    };
} & import("typescript/lib/tsserverlibrary").LanguageService;
