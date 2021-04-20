// Runs all tests in the t directory.
const fs = require("fs");
const path = require("path");

/**
 * Object for declaring test as successful or not successful.
 * @type {{ok: t.ok}}
 */
const t = {
    ok: function() {
        console.log("ok");
    }
};

(function() {
    const rootPath = path.resolve(__dirname, "t");
    fs.readdir(rootPath, function (err, files) {
        files.map(function (file) {
            const test = require(path.resolve(rootPath, file));

            console.log("[" + test.name + "]");
            test.test(t);
        });
    });
})();
