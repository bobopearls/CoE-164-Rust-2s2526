/*
TODO:
    CONSTRAINTS:
    ■ No cloning of strings allowed
    ■ No use of .to_string(), .to_owned(), or String::from()
    ■ All returned slices must borrow directly from input
    ■ The @ terminator should not be included in the output
    Input:
    Alice
    Bob
    Carol
    @       looks like: Alice\nBob\nCarol\n@\n when stored in input
    then when we parse, it should look like:
    [A, l, i, c, e, \n, B, o, b, \n, C, a, r, o, l, \n, @, \n]
    \n is what determines the slices, @ determines the termination

NOTE:
    when doing inputs in the terminal, need to still do Ctrl + Z after "@" to show the output
    If the input is:
    Zara
    Alysa
    @
    Laufey, the @ will only take Zara and Alysa and display 2 counts! SO the @ is just a line break!

    * at this point, no need to use lifetime annotations since rust can do it automatically
    (in this case only)
*/
use std::io::Read;

fn main() {
    let mut input = String::new(); // Owner
    let mut stdin = std::io::stdin();

    if let Ok(_) = stdin.read_to_string(&mut input) {
        let names = parse_names(&input);
        let count = count_names(&names);
        // Owner input is still alive at this point, which is why it can be used
        println!("[NAMES]");
        for name in &names{
            println!("{}", name);
        }

        println!("\n[COUNT]");
        println!("{}", count);
    }
} // input Owner lifetime lasts till here, functions are borrowers!

fn parse_names(input: &str) -> Vec<&str>{ // the function borrows the stuff of the Owner
    //TODO: parsing through the names means that you need to iterate through the string input
    // using the fact that \n slices through them (use .lines()). Return &str slices from the input.
    // if the line has @, then terminate, else just push the line to the names vec
    // Vec copies not the actual contents, but just the pointers to the real data
    // Library Card: just copying the page number and line on a book (&str),
    // and you are not allowed to photocopy the book
    let mut names = Vec::new();

    for line in input.lines() {
        if line == "@"{
            break;
        }
        if !line.is_empty() {
            names.push(line);
        }
    }
    names // then the Owner string will last till here only?
}

fn count_names(names: &[&str]) -> usize {
    names.len() // just return the count
}