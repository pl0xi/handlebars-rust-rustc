extern crate handlebars;

#[macro_use]
extern crate serde_json;

use handlebars::{handlebars_helper, no_escape, Handlebars};

#[test]
fn test_escape_216() {
    let hbs = Handlebars::new();

    let data = json!({
        "FOO": "foo",
        "BAR": "bar"
    });

    assert_eq!(
        hbs.render_template(r"\\\\ {{FOO}} {{BAR}} {{FOO}}{{BAR}} {{FOO}}#{{BAR}} {{FOO}}//{{BAR}} {{FOO}}\\{{FOO}} {{FOO}}\\\\{{FOO}}\\\{{FOO}} \\\{{FOO}} \{{FOO}} \{{FOO}}", &data).unwrap(),
        r"\\\\ foo bar foobar foo#bar foo//bar foo\foo foo\\\foo\\foo \\foo {{FOO}} {{FOO}}"
    );
}

#[test]
fn test_string_no_escape_422() {
    let mut hbs = Handlebars::new();

    handlebars_helper!(replace: |input: str, from: str, to: str| {
        input.replace(from, to)
    });
    hbs.register_helper("replace", Box::new(replace));

    assert_eq!(
        r#"some\ path"#,
        hbs.render_template(r#"{{replace "some/path" "/" "\\ " }}"#, &())
            .unwrap()
    );

    assert_eq!(
        r#"some\path"#,
        hbs.render_template(r#"{{replace "some/path" "/" "\\" }}"#, &())
            .unwrap()
    );
}

#[test]
fn test_string_whitespace_467() {
    const TEMPLATE_UNQUOTED: &str = r#"{{#each synonyms}}
    {{this.name}} => '{{this.sym}}',
    {{/each}}
"#;

    let mut hbs = Handlebars::new();
    hbs.register_escape_fn(no_escape);
    hbs.register_template_string("perl", TEMPLATE_UNQUOTED)
        .unwrap();

    let r = hbs
        .render("perl", &json!({"synonyms": [{"name": "lt", "sym": "<"}]}))
        .unwrap();
    assert_eq!("    lt => '<',\n", r);
}
