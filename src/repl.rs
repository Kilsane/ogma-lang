// ------------------------------------------------------------
// OGMA — REPL 0.3.11 (prototype)
// ------------------------------------------------------------
//
// Pour l’instant :
// - lit une ligne
// - la tokenize
// - la parse comme un "mini-fichier" Ogma
// - affiche l’AST résultant
// ------------------------------------------------------------

use std::io::{self, Write};

use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn run_repl() {
    println!("Ogma 0.3.11 - REPL prototype");
    println!("Tape une déclaration Ogma (ex: x: int = 10)");
    println!("Ctrl+C pour quitter.\n");

    loop {
        let mut input = String::new();

        print!("ogma> ");
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut input).is_err() {
            println!("Erreur de lecture");
            continue;
        }

        // ignorer les lignes vides
        if input.trim().is_empty() {
            continue;
        }

        // 1) Lexer
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();

        // 2) Parser
        let mut parser = Parser::new(tokens);
        match parser.parse_module_file("repl".to_string()) {
            Ok(module) => {
                println!("→ AST : {:#?}", module);
            }
            Err(e) => {
                println!("Erreur de parsing : {}", e);
            }
        }
    }
}
