// importing crates
extern crate pulldown_cmark;
extern crate html5ever;
// importaing modules
use pulldown_cmark::(Parser, Options, html);
use std::io::(self, Read);


// the purpose of this function is to read the markdown file and convert it to html
fn main() {

    // read the markdown file
    let mut input = String::new();

    //read from standard input until EOF is reached and append it to input, if an error occurs, panic with the error message
    io::stdin().read_to_string(&mut input).unwrap();

    // create a parser from the input and convert it to html
    let parser = Parser::new_ext(&input, Options::all());

    // create a string to store the html output
    let mut html_output = String::new();

    // push the html output to the string
    html::push_html(&mut html_output, parser);
    
    // print the html output
    println!("{}", html_output)
}