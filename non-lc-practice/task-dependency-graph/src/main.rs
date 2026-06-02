use std::cell::RefCell;
use std::rc::Rc;

pub struct BuildTarget {
    pub name: String,
    pub is_compiled: bool,
    // A target can have multiple dependencies.
    // Multiple targets might depend on the SAME target, hence Rc<RefCell<...>>
    pub dependencies: Vec<Rc<RefCell<BuildTarget>>>,
}

impl BuildTarget {
    // Helper to create a new target
    pub fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            name: name.to_string(),
            is_compiled: false,
            dependencies: Vec::new(),
        }))
    }
}

// TODO: Implement the recursive compile function.
pub fn compile_target(target: Rc<RefCell<BuildTarget>>) {
    // 1. Presence:
    //    Notice the signature is `Rc`, not `Option<Rc>`. It is guaranteed to exist.
    //    You do NOT need `if let Some(...)` here.

    // 2. Permission (Read):
    //    Borrow the target and check `is_compiled`. If it is already true, return early.
    if target.borrow().is_compiled {
        return;
    }

    // 3. Ownership (Recursion):
    //    Borrow the target to access its `dependencies`.
    //    Iterate through the dependencies.
    //    For each dependency, you must pass it to `compile_target`.
    //    Remember: When passing into a function, you must do what to the Rc?
    let deps = target.borrow().dependencies.clone();
    for dep in deps {
        compile_target(dep);
    }
    // 4. Permission (Write):
    //    Borrow the target mutably and set `is_compiled = true`.
    //    Print a message like: println!("Compiled: {}", target.borrow().name);
    target.borrow_mut().is_compiled = true;
    println!("Compiled: {}", target.borrow().name);
}

fn main() {
    // Create targets
    let libc = BuildTarget::new("libc");
    let regex = BuildTarget::new("regex");
    let main_bin = BuildTarget::new("main_bin");

    // regex depends on libc
    regex.borrow_mut().dependencies.push(libc.clone());

    // main_bin depends on BOTH libc and regex
    // Notice how libc is shared! Two targets own a pointer to it.
    main_bin.borrow_mut().dependencies.push(libc.clone());
    main_bin.borrow_mut().dependencies.push(regex.clone());

    println!("Starting build...");
    compile_target(main_bin);
    println!("Build complete!");
}
