trait Section {
    fn run();
    fn name() -> &'static str;
}

mod web {
    use crate::Section;

    pub enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 }
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed `{}`", c),
            WebEvent::Paste(s) => println!("pasted `{}`", s),
            WebEvent::Click { x, y } => println!("click at x={}, y={}", x, y),
        }
    }

    impl Section for WebEvent {
        fn run() {
            let pressed = WebEvent::KeyPress('x');
            let pasted = WebEvent::Paste(String::from("my text"));
            let click = WebEvent::Click { x: 20, y: 20 };
            let load = WebEvent::PageLoad;
            let unload = WebEvent::PageUnload;

            inspect(pressed);
            inspect(pasted);
            inspect(click);
            inspect(load);
            inspect(unload);
        }

        fn name() -> &'static str {
            module_path!()
        }
    }
}

mod type_aliases {
    use crate::Section;

    #[allow(dead_code)]
    pub enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        #[allow(dead_code)]
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    impl Section for VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run() {
            let _x = Operations::Add;
        }

        fn name() -> &'static str {
            module_path!()
        }
    }
}

fn section<M>() where M: Section {
    println!("{0:-<1$}", M::name(), 32);
    M::run();
}

fn main() {
    section::<web::WebEvent>();
    section::<type_aliases::VeryVerboseEnumOfThingsToDoWithNumbers>();
}
