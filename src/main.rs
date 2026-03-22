// ------------------------------------------------------------
// OGMA — Point d'entrée du langage
// ------------------------------------------------------------
//
// Ce fichier assemble les différents modules du langage
// (valeurs, parseur, REPL) et lance la boucle interactive.
//
// La logique interne d’Ogma est répartie dans :
//   - value.rs  : définition des types internes
//   - parser.rs : analyse des lignes de commande
//   - repl.rs   : boucle interactive
//
// main.rs doit rester minimal et lisible.
//
mod value;
mod parser;
mod repl;
mod lexer;

fn main() {
    repl::run_repl();
}