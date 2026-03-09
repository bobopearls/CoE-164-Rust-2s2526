use std::cell::RefCell;
use std::rc::Rc; // https://doc.rust-lang.org/std/rc/
use std::io::Read;
use std::io;

// https://fkohlgrueber.github.io/blog/tree-structure-of-file-systems/
fn main() {
    // Read entire input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Build filesystem
    let fs = FileSystem::from(&input);

    // Print files
    println!("[FILES]");
    for file in fs.iter_files() {
        println!("{}", file);
    }

    // Print directories
    println!("\n[DIRS]");
    for dir in fs.iter_dirs() {
        println!("{}", dir);
    }
}

///////////////////////////////////////////////////////////
// NODE DEFINITIONS
///////////////////////////////////////////////////////////

enum Node<'a> {
    File(&'a str),
    Dir {
        name: &'a str,
        children: RefCell<Vec<Rc<Node<'a>>>>,
        //https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.borrow_mut
    }
}

///////////////////////////////////////////////////////////
// FILESYSTEM STRUCT
///////////////////////////////////////////////////////////

struct FileSystem<'a> {
    root: Rc<Node<'a>>
}

impl<'a> FileSystem<'a> {
    // Build filesystem from input
    fn from(input: &'a String) -> Self {
        // virtual root
        let root = Rc::new(Node::Dir {
            name: "/", // looks like [/, ] and if we add DIR root, it will be [/, root]
            children: std::cell::RefCell::new(Vec::new()),
        });

        // then let the stack track the directory
        let stack = vec![Rc::clone(&root)];

        for line in input.lines() {
            let trimmed_line = line.trim();
            if trimmed_line == "END" {
                continue;
            }
            if trimmed_line == "@"{
                break;
            }
            if let Some(name) = trimmed_line.strip_prefix("DIR "){
                // if we have DIR root -> new_dir with the root and no children
                // then after add new_dir to the children of /
                // then push new_dir to the stack
                // so if we have a next line that is FILE a.txt, because DIR root was above,
                // then a.txt becomes one of root's children
                let new_dir = Rc::new(Node::Dir{
                    name: name, // root
                    children: std::cell::RefCell::new(Vec::new()), // empty unless FILE
                });
                // then link the parent which is the last one that was added to the stack
                // [/, root] => root is the last added, it is now the current parent of the stack
                if let Some(parent) = stack.last(){
                    if let Node::Dir{children,..} =&**parent{
                        // needed to re-reference the actual Rc node data using &**
                        children.borrow_mut().push(Rc::clone(&new_dir)); // child from Node pushed to parent Vec
                    }
                }
            }
            else if let Some(name) = trimmed_line.strip_prefix("FILE "){
                let new_file = Rc::new(Node::File(name));

                if let Some(parent) = stack.last(){
                    if let Node::Dir{children, ..} = &**parent{
                        children.borrow_mut().push(new_file);
                        // Don't clone because the file does not need to have another file in it as a parent
                    }
                }
            }
        }
        FileSystem { root }
    }

    ////////////////////////////////////////////////////////
    // ITERATORS
    ////////////////////////////////////////////////////////

    fn iter_files(&self) -> FileIterator<'a> {
        FileIterator {
            stack: vec![Rc::clone(&self.root)]
        }
    }

    fn iter_dirs(&self) -> DirIterator<'a> {
        DirIterator {
            stack: vec![Rc::clone(&self.root)]
        }
    }
}

///////////////////////////////////////////////////////////
// FILE ITERATOR
///////////////////////////////////////////////////////////

struct FileIterator<'a> {
    stack: Vec<Rc<Node<'a>>>
}

impl<'a> Iterator for FileIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match &*node {
                Node::File(name) => {
                    return Some(name);
                }

                Node::Dir { name: _, children } => {
                    // read children then push children to stack
                    let children_borrowed = children.borrow();
                    for child in children_borrowed.iter().rev() {
                        self.stack.push(Rc::clone(child));
                    }
                }
            }
        }
        None
    }
}

///////////////////////////////////////////////////////////
// DIR ITERATOR
///////////////////////////////////////////////////////////

struct DirIterator<'a> {
    stack: Vec<Rc<Node<'a>>>
}

impl<'a> Iterator for DirIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match &*node {
                Node::File(_) => {
                    continue;
                }
                Node::Dir {name, children } => {
                    let children_borrowed = children.borrow();
                    for child in children_borrowed.iter().rev() {
                        self.stack.push(Rc::clone(child));
                    }
                    if *name != "/"{
                        return Some(name);
                    }

                }
            }
        }
        None
    }
}