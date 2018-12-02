use std::env;
use std::collections::HashMap;

fn main() {
    let mut args = env::args();
    args.next();

    let modules = create_modules();
    while let Some(arg) = args.next() {
        modules.run(&arg);
    }
}

type Run = fn();

struct Modules {
    runners: HashMap<String, Run>,
}

impl Modules {
    fn add_module(&mut self, name: String, func: Run) {
        self.runners.insert(name, func);
    }

    fn run(&self, name: &String) {
        let runner = self.runners.get(name).unwrap();
        runner();
    }
}

macro_rules! modules {
    ($($mod:ident,)*) => {
        $( mod $mod; )*

        fn create_modules() -> Modules {
            let mut modules = Modules { runners: HashMap::new() };
            $( modules.add_module(stringify!($mod).to_string(), $mod::run); )*
            modules
        }
    };
}

modules![
    day_01,
];
