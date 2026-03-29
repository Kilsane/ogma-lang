// ------------------------------------------------------------
// OGMA — Module Parser (legacy REPL 0.2, corrigé pour 0.3.11)
// ------------------------------------------------------------
//
// Le parseur transforme une liste de tokens (produits par le
// lexer) en structures internes appelées Command.
//
// Version actuelle :
//   - reconnaissance des valeurs simples (int, string, bool…)
//   - reconnaissance des chemins a.b.c (corrigé, plus de a/b/c)
//
// Le parseur NE lit PAS la chaîne brute : il travaille
// exclusivement sur les tokens. Cela permet d'ajouter plus tard :
//   - les blocs { ... }
//   - les listes [ ... ]
//   - les objets { a: 1 }
//   - les expressions
//   - les appels de fonctions
//   - les opérations
//
// Ce module reste un parseur REPL minimal, pas le parser 0.3.11
// complet décrit dans la spec officielle.
// ------------------------------------------------------------

use crate::value::OgmaValue;
use crate::lexer::Token;

// ------------------------------------------------------------
// Command : structure syntaxique interne
// ------------------------------------------------------------
//
// Chaque ligne entrée dans Ogma est transformée en une Command.
// Pour l'instant :
//   - Value(...) : une valeur simple
//   - Path([...]) : un chemin a.b.c
//   - Block([...]) : un bloc { ... } très simplifié
//
// Plus tard (dans le vrai parser 0.3.11) :
//   - Assign(name, expr)
//   - Call(func, args)
//   - Block(exprs)
//   - Object(fields)
//   - etc.
// ------------------------------------------------------------
#[derive(Debug)]
pub enum Command {
    Value(OgmaValue),
    Path(Vec<String>),
    Block(Vec<Command>),
}

// ------------------------------------------------------------
// parse_path : reconnaissance d'un chemin a.b.c
// ------------------------------------------------------------
//
// Un chemin est une suite :
//   Ident "." Ident "." Ident...
//
// Le lexer fournit déjà les tokens nécessaires :
//   Ident("a"), Dot, Ident("b"), Dot, Ident("c")
//
// Si la structure correspond, on renvoie Command::Path([...]).
// Sinon, on renvoie None pour laisser le parseur essayer autre chose.
// ------------------------------------------------------------
fn parse_path(tokens: &[Token]) -> Option<Command> {
    let mut parts = Vec::new();
    let mut i = 0;

    // Le premier token doit être un identifiant
    match tokens.get(i) {
        Some(Token::Ident(name)) => {
            parts.push(name.clone());
            i += 1;
        }
        _ => return None, // pas un chemin
    }

    // Ensuite : (Dot Ident)* répété
    while i + 1 < tokens.len() {
        match (&tokens[i], &tokens[i + 1]) {
            (Token::Dot, Token::Ident(name)) => {
                parts.push(name.clone());
                i += 2;
            }
            _ => break,
        }
    }

    // Si on a au moins 2 éléments, c’est un vrai chemin
    if parts.len() >= 2 {
        Some(Command::Path(parts))
    } else {
        None
    }
}

// ------------------------------------------------------------
// parse_block : reconnaissance d'un bloc { ... }
// ------------------------------------------------------------
//
// Un bloc commence par '{' et se termine par '}'.
// Il contient plusieurs commandes, séparées par des retours à la ligne.
//
// Exemple :
// {
//     int 1
//     int 2
// }
//
// Résultat : Block([Value(1), Value(2)])
//
// ⚠️ Version simplifiée : ce n’est PAS encore le bloc 0.3.11 officiel.
// ------------------------------------------------------------
fn parse_block(tokens: &[Token]) -> Option<(Command, usize)> {
    let mut i = 0;

    // Le premier token doit être '{'
    match tokens.get(i) {
        Some(Token::LBrace) => i += 1,
        _ => return None,
    }

    let mut commands = Vec::new();

    // Lire les commandes jusqu'à '}'
    while i < tokens.len() {
        match tokens.get(i) {
            Some(Token::RBrace) => {
                // Fin du bloc
                return Some((Command::Block(commands), i + 1));
            }
            _ => {
                // On parse une commande interne (très simplifié)
                let cmd = parse_tokens(&tokens[i..]).ok()?;
                commands.push(cmd);

                // Avancer d'un token (simplifié pour l'instant)
                i += 1;
            }
        }
    }

    None // pas de '}' trouvé
}

// ------------------------------------------------------------
// parse_tokens : parseur principal (REPL minimal)
// ------------------------------------------------------------
//
// Reçoit une liste de tokens et tente de reconnaître :
//   1) un bloc { ... }
//   2) un chemin a.b.c
//   3) une commande simple (int, string, bool…)
//   4) sinon → erreur
//
// L'ordre est important :
//   - un bloc commence par '{'
//   - un chemin commence par un identifiant
//   - donc on doit tester "bloc" puis "chemin" AVANT "commande"
// ------------------------------------------------------------
pub fn parse_tokens(tokens: &[Token]) -> Result<Command, String> {
    if tokens.is_empty() {
        return Err("Ligne vide".to_string());
    }

    // 1) Tentative : est-ce un bloc ?
    if let Some((cmd, _consumed)) = parse_block(tokens) {
        return Ok(cmd);
    }

    // 2) Tentative : est-ce un chemin ?
    if let Some(cmd) = parse_path(tokens) {
        return Ok(cmd);
    }

    // 3) Commandes simples (int, string, bool, decimal, ognum)
    match &tokens[0] {
        // int 42
        Token::Ident(cmd) if cmd == "int" => {
            if tokens.len() != 2 {
                return Err("Usage : int <nombre>".to_string());
            }
            match &tokens[1] {
                Token::Number(n) => match n.parse::<i64>() {
                    Ok(v) => Ok(Command::Value(OgmaValue::Int(v))),
                    Err(_) => Err("Impossible de convertir en entier.".to_string()),
                },
                _ => Err("int attend un nombre".to_string()),
            }
        }

        // string "hello"
        Token::Ident(cmd) if cmd == "string" => {
            if tokens.len() != 2 {
                return Err("Usage : string \"texte\"".to_string());
            }
            match &tokens[1] {
                Token::StringLiteral(s) => Ok(Command::Value(OgmaValue::String(s.clone()))),
                _ => Err("string attend une chaîne entre guillemets".to_string()),
            }
        }

        // bool true / false
        Token::Ident(cmd) if cmd == "bool" => {
            if tokens.len() != 2 {
                return Err("Usage : bool <true|false>".to_string());
            }
            match &tokens[1] {
                Token::Ident(v) if v == "true" => Ok(Command::Value(OgmaValue::Bool(true))),
                Token::Ident(v) if v == "false" => Ok(Command::Value(OgmaValue::Bool(false))),
                _ => Err("bool attend true ou false".to_string()),
            }
        }

        // decimal 3.14
        Token::Ident(cmd) if cmd == "decimal" => {
            if tokens.len() != 2 {
                return Err("Usage : decimal <nombre>".to_string());
            }
            match &tokens[1] {
                Token::Number(n) => match n.parse::<f64>() {
                    Ok(v) => Ok(Command::Value(OgmaValue::Decimal(v))),
                    Err(_) => Err("Impossible de convertir en décimal".to_string()),
                },
                _ => Err("decimal attend un nombre".to_string()),
            }
        }

        // ognum 123.45 (forme REPL, pas la forme type ognum(p,s))
        Token::Ident(cmd) if cmd == "ognum" => {
            if tokens.len() != 2 {
                return Err("Usage : ognum <nombre>".to_string());
            }
            match &tokens[1] {
                Token::Number(n) => Ok(Command::Value(OgmaValue::Ognum(n.clone()))),
                _ => Err("ognum attend un nombre".to_string()),
            }
        }

        // 4) Rien ne correspond → erreur
        _ => Err("Commande inconnue".to_string()),
    }
}
