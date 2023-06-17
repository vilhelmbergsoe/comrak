#![no_main]

use libfuzzer_sys::fuzz_target;

use comrak::{
    markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, Plugins,
    RenderPlugins,
};

// Note that we end up fuzzing Syntect here.

fuzz_target!(|s: &str| {
    let adapter = SyntectAdapter::new("base16-ocean.dark");
    let plugins = Plugins {
        render: RenderPlugins {
            codefence_syntax_highlighter: Some(&adapter),
            ..Default::default()
        },
    };

    markdown_to_html_with_plugins(s, &Default::default(), &plugins);
});