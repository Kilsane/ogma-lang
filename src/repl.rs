// ------------------------------------------------------------
// OGMA — Module REPL
// ------------------------------------------------------------
//
// Le REPL (Read–Eval–Print Loop) est la boucle interactive
// d’Ogma. Il lit une ligne, la transforme en tokens via le lexer,
// puis en Command via le parseur, et affiche le résultat.
//
// Cette version 0.1 ne gère que des valeurs simples, mais la
// structure est prête pour accueillir :
//
//   - des commandes internes
//   - des blocs multi-lignes
//   - des erreurs plus détaillées
//   - un environnement d'exécution
// ------------------------------------------------------------

use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::{parse_tokens, Command};

pub fn run_repl() {
    println!("Ogma 0.0.1 - Prototype pédagogique");
    println!("Tape : int 42");
    println!("Ctrl+C pour quitter.\n");

    loop {
        let mut input = String::new();

        // Invite
        print!("ogma> ");
        std::io::stdout().flush().unwrap();

        // Lecture de la ligne
        io::stdin().read_line(&mut input).expect("Erreur de lecture");

        // Étape 1 : lexer → tokens
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();

        // Étape 2 : parseur → Command
        match parse_tokens(&tokens) {

    Ok(Command::Value(v)) => {
        println!("→ Valeur Ogma : {:?}", v);
    }

    Ok(Command::Path(parts)) => {
        println!("→ Chemin Ogma : {:?}", parts);
    }

    Ok(Command::Block(cmds)) => {
        println!("→ Bloc Ogma : {:?}", cmds);
    }

    Err(e) => {
        println!("Erreur : {}", e);
    }
}
    }
}