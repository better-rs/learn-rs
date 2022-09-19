use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // TODO X: templates/ 路径和 src/ 平级. 
// to the `templates` dir in the crate root
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, /* the field name should match the variable name
                    * in your template */
}

fn main() {
    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    println!("🚀 {}", hello.render().unwrap()); // then render it.
}
