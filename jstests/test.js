// Runs all tests in the t directory.
const fs = require("fs");
const path = require("path");
const lib = require("./lib");

(() => {
    const rootPath = path.resolve(__dirname, "t");
    fs.readdir(rootPath, (err, files) => {
        files.map((file) => {
            const {
                name,
                before,
                after,
                test
            } = require(path.resolve(rootPath, file));

            console.log("[" + name + "]");

            if (before) before();

            test(lib.t, lib.u);

            if (after) after();
        });
    });
})();
