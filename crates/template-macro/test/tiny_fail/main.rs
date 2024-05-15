use minijinja::context;
use template_macro::template;

fn main() {
    let data = context!(code => 200, name => "Alex", country => "Nigeria");

    let template = template!(jinja, {
         [hello, 20],
    });

    print!(
        "Content: {:?}",
        template
            .get_template("hello")
            .unwrap()
            .render(data)
            .unwrap()
    );
}