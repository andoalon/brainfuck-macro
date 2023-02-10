#[macro_export]
macro_rules! brainfuck {
    ($($code:tt) *) => {
        {
            let mut state = brainfuck::BrainfuckState::default();
            brainfuck_with_state!(state, $($code)*)
        }
    };    
}

#[macro_export]
macro_rules! brainfuck_with_state {
    ($state:ident, $($code:tt) *) => {
        {
            $(brainfuck_operation!($state, $code);)*
        }
    };
}

#[macro_export]
macro_rules! brainfuck_operation {
    ($state:ident, +) => { $state.plus() };
    ($state:ident, -) => { $state.minus() };
    ($state:ident, .) => { $state.print() };
    ($state:ident, ,) => { $state.input() };
    ($state:ident, <) => { $state.left() };
    ($state:ident, >) => { $state.right() };

    // << and >> need to be handled
    // separately because the tokenizer
    // doesn't treat them as two < or >,
    // but rather as a single shift left/right token 
    ($state:ident, <<) => { brainfuck_operation!($state, <); brainfuck_operation!($state, <) };
    ($state:ident, >>) => { brainfuck_operation!($state, >); brainfuck_operation!($state, >) };

    // This one also needs to be taken care of separately
    // because the tokenizer detects an arrow '->'
    ($state:ident, ->) => { brainfuck_operation!($state, -); brainfuck_operation!($state, >) };

    ($state:ident, [$($loop_body:tt) *]) => {
        while (*$state.get_mut() != 0) {
            $(brainfuck_operation!($state, $loop_body);)*
        }
    };
}

pub struct BrainfuckState {
    pub index: u32,
    pub memory: [u8; Self::MEMORY_SIZE],
}

impl BrainfuckState {
    pub const MEMORY_SIZE: usize = 30_000;
}

impl Default for BrainfuckState {
    fn default() -> Self {
        Self { index: Default::default(), memory: [0; Self::MEMORY_SIZE] }
    }
}

impl BrainfuckState {
    pub fn plus(&mut self) {
        let num = self.get_mut();
        *num = num.checked_add(1).expect("Value overflow");
    }

    pub fn minus(&mut self) {
        let num = self.get_mut();
        *num = num.checked_sub(1).expect("Value underflow");
    }

    pub fn right(&mut self) {
        self.index = self.index.checked_add(1).expect("Index overflow");    
    }

    pub fn left(&mut self) {
        self.index = self.index.checked_sub(1).expect("Index underflow");    
    }

    pub fn print(&mut self) { // mut because I don't feel like making a separate get() (non-mut)
        let num = *self.get_mut();
        print!("{}", num as char);
    }

    pub fn input(&mut self) {
        let mut line = String::new();

        *self.get_mut() = loop {
            while let Err(_) = std::io::stdin().read_line(&mut line)
            {}

            match line.parse() {
                Err(err) => {
                    println!("{}", err);
                    continue;
                },
                Ok(num) => break num,
            }
        };
    }

    pub fn get_mut(&mut self) -> &mut u8 {
        self.memory.get_mut(self.index as usize).expect("Index out of bounds")
    }
}
