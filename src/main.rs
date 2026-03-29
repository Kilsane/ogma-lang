// ------------------------------------------------------------
// OGMA — Point d'entrée du langage
// ------------------------------------------------------------

mod value;
mod ast;
mod lexer;
mod parser;
mod repl;

fn main() {
    repl::run_repl();
}
