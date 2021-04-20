/**
 * Tests that a node is successfully queried if inserted.
 */
module.exports = {
    name: "Insert Node",
    before: () => {
        console.log("Hello world!");
    },
    test: (t) => {
        t.ok();
    },
    after: () => {
        console.log("Bye world!");
    },
}
