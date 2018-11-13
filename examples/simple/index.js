
(global => {
    import("./build/dommer_example_simple").then(ex => {
        global.app = ex.run();
    });
})(window);

