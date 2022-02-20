// #![crate_type = "lib"]
// #![crate_name = "ex02"]
#[test]
fn ex02_crate() {
    //
    pub fn public_function() {
        println!("called `public_function()`");
    }

    fn private_function() {
        println!("called `private_function()`");
    }

    pub fn indirect_access() {
        println!("Called `indirect_access()`, that\n> ");
        private_function()
    }

    //
    public_function();
    indirect_access();
}
