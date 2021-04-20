/**
 * Object for declaring a test as successful or not successful with an error.
 * @type {{ok: t.ok}}
 */
const t = {
    ok: () => {
        console.log("ok");
    }
};

/**
 * Object that contains common test operations, such as cleanup operations.
 * @type {{}}
 */
const u = {

}

module.exports = {
    t,
    u
}
