use super::*;

#[test]
fn autolink_www() {
    html_opts!(
        [extension.autolink],
        concat!("www.autolink.com\n"),
        concat!("<p><a href=\"http://www.autolink.com\">www.autolink.com</a></p>\n"),
    );
}

#[test]
fn autolink_email() {
    html_opts!(
        [extension.autolink],
        concat!("john@smith.com\n"),
        concat!("<p><a href=\"mailto:john@smith.com\">john@smith.com</a></p>\n"),
    );
}

#[test]
fn autolink_scheme() {
    html_opts!(
        [extension.autolink],
        concat!("https://google.com/search\n", "rdar://localhost.com/blah"),
        concat!(
            "<p><a href=\"https://google.com/search\">https://google.com/search</a>\n",
            "rdar://localhost.com/blah</p>\n"
        ),
    );
}

#[test]
fn autolink_scheme_multiline() {
    html_opts!(
        [extension.autolink],
        concat!("https://google.com/search\nhttps://www.google.com/maps"),
        concat!(
            "<p><a href=\"https://google.com/search\">https://google.\
             com/search</a>\n<a href=\"https://www.google.com/maps\">\
             https://www.google.com/maps</a></p>\n"
        ),
    );
}

#[test]
fn autolink_no_link_bad() {
    html_opts!(
        [extension.autolink],
        concat!("@a.b.c@. x\n", "\n", "n@. x\n"),
        concat!("<p>@a.b.c@. x</p>\n", "<p>n@. x</p>\n"),
    );
}

#[test]
fn autolink_parentheses_balanced() {
    let examples = [
        [
            "http://www.pokemon.com/Pikachu_(Electric)",
            "<p><a href=\"http://www.pokemon.com/Pikachu_(Electric)\">http://www.pokemon.com/Pikachu_(Electric)</a></p>\n",
        ],
        [
            "http://www.pokemon.com/Pikachu_((Electric)",
            "<p><a href=\"http://www.pokemon.com/Pikachu_((Electric)\">http://www.pokemon.com/Pikachu_((Electric)</a></p>\n",
        ],
        [
            "http://www.pokemon.com/Pikachu_(Electric))",
            "<p><a href=\"http://www.pokemon.com/Pikachu_(Electric)\">http://www.pokemon.com/Pikachu_(Electric)</a>)</p>\n",
        ],
        [
            "http://www.pokemon.com/Pikachu_((Electric))",
            "<p><a href=\"http://www.pokemon.com/Pikachu_((Electric))\">http://www.pokemon.com/Pikachu_((Electric))</a></p>\n",
        ],
    ];

    for example in examples {
        html_opts!([extension.autolink], example[0], example[1]);
    }

    for example in examples {
        html_opts!(
            [extension.autolink, parse.relaxed_autolinks],
            example[0],
            example[1]
        );
    }
}

#[test]
fn autolink_brackets_unbalanced() {
    html_opts!(
        [extension.autolink],
        concat!("http://example.com/[abc]]...\n"),
        concat!(
            "<p><a href=\"http://example.com/%5Babc%5D%5D\">http://example.com/[abc]]</a>...</p>\n"
        ),
    );
}

#[test]
fn autolink_ignore_links_in_brackets() {
    let examples = [
        ["[https://foo.com]", "<p>[https://foo.com]</p>\n"],
        ["[[https://foo.com]]", "<p>[[https://foo.com]]</p>\n"],
        [
            "[[Foo|https://foo.com]]",
            "<p>[[Foo|https://foo.com]]</p>\n",
        ],
        [
            "[<https://foo.com>]",
            "<p>[<a href=\"https://foo.com\">https://foo.com</a>]</p>\n",
        ],
    ];

    for example in examples {
        html_opts!([extension.autolink], example[0], example[1], no_roundtrip);
    }
}

#[test]
fn autolink_relaxed_links_in_brackets() {
    let examples = [
        [
            "[https://foo.com]",
            "<p>[<a href=\"https://foo.com\">https://foo.com</a>]</p>\n",
        ],
        [
            "[[https://foo.com]]",
            "<p>[[<a href=\"https://foo.com\">https://foo.com</a>]]</p>\n",
        ],
        [
            "[[Foo|https://foo.com]]",
            "<p>[[Foo|<a href=\"https://foo.com\">https://foo.com</a>]]</p>\n",
        ],
        [
            "[<https://foo.com>]",
            "<p>[<a href=\"https://foo.com\">https://foo.com</a>]</p>\n",
        ],
        [
            "[http://foo.com/](url)",
            "<p><a href=\"url\">http://foo.com/</a></p>\n",
        ],
        ["[http://foo.com/](url", "<p>[http://foo.com/](url</p>\n"],
        [
            "[www.foo.com/](url)",
            "<p><a href=\"url\">www.foo.com/</a></p>\n",
        ],
        [
            "{https://foo.com}",
            "<p>{<a href=\"https://foo.com\">https://foo.com</a>}</p>\n",
        ],
        [
            "[this http://and.com that](url)",
            "<p><a href=\"url\">this http://and.com that</a></p>\n",
        ],
        [
            "[this <http://and.com> that](url)",
            "<p><a href=\"url\">this http://and.com that</a></p>\n",
        ],
        [
            "{this http://and.com that}(url)",
            "<p>{this <a href=\"http://and.com\">http://and.com</a> that}(url)</p>\n",
        ],
        [
            "[http://foo.com](url)\n[http://bar.com]\n\n[http://bar.com]: http://bar.com/extra",
            "<p><a href=\"url\">http://foo.com</a>\n<a href=\"http://bar.com/extra\">http://bar.com</a></p>\n",
        ],
    ];

    for example in examples {
        html_opts!(
            [extension.autolink, parse.relaxed_autolinks],
            example[0],
            example[1]
        );
    }
}

#[test]
fn autolink_relaxed_links_brackets_balanced() {
    html_opts!(
        [extension.autolink, parse.relaxed_autolinks],
        concat!("http://example.com/[abc]]...\n"),
        concat!(
            "<p><a href=\"http://example.com/%5Babc%5D\">http://example.com/[abc]</a>]...</p>\n"
        ),
    );
}

#[test]
fn autolink_relaxed_links_curly_braces_balanced() {
    html_opts!(
        [extension.autolink, parse.relaxed_autolinks],
        concat!("http://example.com/{abc}}...\n"),
        concat!(
            "<p><a href=\"http://example.com/%7Babc%7D\">http://example.com/{abc}</a>}...</p>\n"
        ),
    );
}

#[test]
fn autolink_relaxed_links_curly_parentheses_balanced() {
    html_opts!(
        [extension.autolink, parse.relaxed_autolinks],
        concat!("http://example.com/(abc))...\n"),
        concat!("<p><a href=\"http://example.com/(abc)\">http://example.com/(abc)</a>)...</p>\n"),
    );
}

#[test]
fn autolink_relaxed_links_schemes() {
    let examples = [
        [
            "https://foo.com",
            "<p><a href=\"https://foo.com\">https://foo.com</a></p>\n",
        ],
        [
            "smb:///Volumes/shared/foo.pdf",
            "<p><a href=\"smb:///Volumes/shared/foo.pdf\">smb:///Volumes/shared/foo.pdf</a></p>\n",
        ],
        [
            "irc://irc.freenode.net/git",
            "<p><a href=\"irc://irc.freenode.net/git\">irc://irc.freenode.net/git</a></p>\n",
        ],
        [
            "rdar://localhost.com/blah",
            "<p><a href=\"rdar://localhost.com/blah\">rdar://localhost.com/blah</a></p>\n",
        ],
    ];

    for example in examples {
        html_opts!(
            [extension.autolink, parse.relaxed_autolinks],
            example[0],
            example[1]
        );
    }
}

#[test]
fn sourcepos_correctly_restores_context() {
    // There's unsoundness in trying to maintain and adjust sourcepos
    // when doing autolinks in the light of:
    //
    // a) Some source elements introducing a different number of characters
    //    to the content text than they take in source, i.e. smart
    //    punctuation.
    //
    // b) Text node consolidation happening before autolinking.
    //
    // (b) is obviously non-optional, but it means we end up with Text
    // nodes with different byte counts than their sourcepos span lengths.
    //
    // One possible solution would be to actually accumulate multiple
    // sourcepos spans per Text node, each also tracking the number of
    // bytes of content text it's responsible for.  This would work well
    // enough as long as we never had to adjust a sourcepos into a spot
    // within a sourcepos span that had a target text width where it
    // wasn't equal.  That probably wouldn't happen, though -- i.e. we're
    // never autolinking into the middle of a rendered smart punctuation.
    //
    // For now the desired sourcepos is documented in comment.  What we
    // have currently (after backing out the adjustments, having hit the
    // above case) matches cmark-gfm.
    assert_ast_match!(
        [],
        "ab _cde_ f@g.ee h*ijklm* n",
        (document (1:1-1:26) [
            (paragraph (1:1-1:26) [
                (text (1:1-1:3) "ab ")
                (emph (1:4-1:8) [
                    (text (1:5-1:7) "cde")
                ])
                (text (1:9-1:17) " f@g.ee h")
                (emph (1:18-1:24) [
                    (text (1:19-1:23) "ijklm")
                ])
                (text (1:25-1:26) " n")
            ])
        ])
    );

    assert_ast_match!(
        [extension.autolink],
        "ab _cde_ f@g.ee h*ijklm* n",
        (document (1:1-1:26) [
            (paragraph (1:1-1:26) [
                (text (1:1-1:3) "ab ")
                (emph (1:4-1:8) [
                    (text (1:5-1:7) "cde")
                ])
                (text (1:9-1:17) " ")             // (text (1:9-1:9) " ")
                (link (XXX) [                     // (link (1:10-1:15) [
                    (text (XXX) "f@g.ee")             // (text (1:10-1:15) "f@g.ee")
                ])
                (text (XXX) " h")                 // (text (1:16-1:17) " h")
                (emph (1:18-1:24) [
                    (text (1:19-1:23) "ijklm")
                ])
                (text (1:25-1:26) " n")
            ])
        ])
    );
}

#[test]
fn autolink_cmark_edge_382() {
    html_opts!(
        [extension.autolink],
        "See &lt;&lt;&lt;http://example.com/&gt;&gt;&gt;",
        "<p>See &lt;&lt;&lt;<a href=\"http://example.com/\">http://example.com/</a>&gt;&gt;&gt;</p>\n",
    );
}

#[test]
fn autolink_cmark_edge_388() {
    html_opts!(
        [extension.autolink],
        "http://example.com/src/_mocks_/vscode.js",
        "<p><a href=\"http://example.com/src/_mocks_/vscode.js\">http://example.com/src/_mocks_/vscode.js</a></p>\n",
    );
}

#[test]
fn autolink_cmark_edge_423() {
    html_opts!(
        [extension.autolink, extension.strikethrough],
        concat!(
            "Here's an autolink: ",
            "https://www.unicode.org/review/pri453/feedback.html#:~:text=Fri%20Jun%2024%2009:56:01%20CDT%202022",
            " and another one ",
            "https://www.unicode.org/review/pri453/feedback.html#:~:text=Fri%20Jun%2024%2009:56:01%20CDT%202022",
            ".",
        ),
        concat!(
            "<p>Here's an autolink: ",
            r#"<a href="https://www.unicode.org/review/pri453/feedback.html#:~:text=Fri%20Jun%2024%2009:56:01%20CDT%202022">"#,
            "https://www.unicode.org/review/pri453/feedback.html#:~:text=Fri%20Jun%2024%2009:56:01%20CDT%202022",
            "</a> and another one ",
            r#"<a href="https://www.unicode.org/review/pri453/feedback.html#:~:text=Fri%20Jun%2024%2009:56:01%20CDT%202022">"#,
            "https://www.unicode.org/review/pri453/feedback.html#:~:text=Fri%20Jun%2024%2009:56:01%20CDT%202022",
            "</a>.</p>\n",
        ),
    );
}

#[test]
fn autolink_cmark_edge_58() {
    html_opts!(
        [extension.autolink, extension.superscript],
        "https://www.wolframalpha.com/input/?i=x^2+(y-(x^2)^(1/3))^2=1",
        concat!(
            "<p>",
            r#"<a href="https://www.wolframalpha.com/input/?i=x%5E2+(y-(x%5E2)%5E(1/3))%5E2=1">"#,
            "https://www.wolframalpha.com/input/?i=x^2+(y-(x^2)^(1/3))^2=1",
            "</a></p>\n",
        ),
    );
}

#[test]
fn autolink_failing_spec_image() {
    html_opts!(
        [extension.autolink],
        "![http://inline.com/image](http://inline.com/image)",
        "<p><img src=\"http://inline.com/image\" alt=\"http://inline.com/image\" /></p>\n",
    );
}

#[test]
fn autolink_failing_spec_underscores() {
    html_opts!(
        [extension.autolink],
        "Underscores not allowed in host name www.xxx.yyy._zzz",
        "<p>Underscores not allowed in host name www.xxx.yyy._zzz</p>\n",
    );
}

#[test]
fn autolink_fuzz_leading_colon() {
    html_opts!(
        [extension.autolink, parse.relaxed_autolinks],
        "://-",
        "<p><a href=\"://-\">://-</a></p>\n",
        no_roundtrip,
    );
}

#[test]
fn autolink_fuzz_we() {
    html_opts!(
        [extension.autolink, parse.relaxed_autolinks],
        "we://w",
        "<p><a href=\"we://w\">we://w</a></p>\n",
        no_roundtrip,
    );
}
