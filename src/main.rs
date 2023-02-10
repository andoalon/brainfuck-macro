mod brainfuck;

fn main() {
    brainfuck!(+++++ +++++[- >+++ +++<] >
        //>><< <+-> // Actually does nothing but exercises the <<, >> and -> detection
    +++++ . + .);
}
