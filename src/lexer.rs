// ------------------------------------------------------------
// OGMA — Module Lexer (version 0.2)
// ------------------------------------------------------------
//
// Le rôle du lexer est de transformer une chaîne brute en une
// liste de tokens structurés. Le parseur n'analyse plus du texte,
// mais uniquement ces tokens.
//
// Cette version gère :
//   - les identifiants
//   - les nombres
//   - les chaînes entre guillemets
//   - les symboles simples : { } [ ] ( ) : , /
//   - les symboles multi-caractères : == != <= >= -> => :: .. ...
//   - les espaces et retours à la ligne
//
// Le lexer lit caractère par caractère, et peut regarder le
// caractère suivant (peek) pour détecter les symboles composés.
// ------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Token {
    // Valeurs
    Ident(String),
    Number(String),
    StringLiteral(String),

    // Symboles simples
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    LParen,     // (
    RParen,     // )
    Colon,      // :
    Comma,      // ,
    Slash,      // /
    Minus,      // -

    // Symboles multi-caractères
    Eq,         // ==
    Neq,        // !=
    Le,         // <=
    Ge,         // >=
    Arrow,      // ->
    FatArrow,   // =>
    DoubleColon,// ::
    Range,      // ..
    Ellipsis,   // ...

    EOF,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    // Lit le caractère courant et avance
    fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            None
        } else {
            let c = self.chars[self.pos];
            self.pos += 1;
            Some(c)
        }
    }

    // Regarde le caractère suivant sans avancer
    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).cloned()
    }

    // Consomme tant que la condition est vraie
    fn consume_while<F>(&mut self, cond: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.peek_char() {
            if cond(c) {
                result.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        result
    }

    // Fonction principale : transforme l'entrée en tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.next_char() {
            match c {

                // ------------------------------------------------------------
                // ESPACES
                // ------------------------------------------------------------
                c if c.is_whitespace() => continue,

                // ------------------------------------------------------------
                // NOMBRES
                // ------------------------------------------------------------
                c if c.is_ascii_digit() || c == '-' => {
                    // Attention : '-' peut être un symbole ou un nombre
                    if c == '-' && self.peek_char() == Some('>') {
                        self.next_char();
                        tokens.push(Token::Arrow);
                        continue;
                    }

                    let mut number = c.to_string();
                    number.push_str(&self.consume_while(|ch| ch.is_ascii_digit() || ch == '.'));
                    tokens.push(Token::Number(number));
                }

                // ------------------------------------------------------------
                // IDENTIFIANTS
                // ------------------------------------------------------------
                c if c.is_ascii_alphabetic() => {
                    let mut ident = c.to_string();
                    ident.push_str(&self.consume_while(|ch| ch.is_ascii_alphanumeric() || ch == '_'));
                    tokens.push(Token::Ident(ident));
                }

                // ------------------------------------------------------------
                // CHAÎNES "..."
                // ------------------------------------------------------------
                '"' => {
                    let content = self.consume_while(|ch| ch != '"');
                    self.next_char(); // consomme le guillemet fermant
                    tokens.push(Token::StringLiteral(content));
                }

                // ------------------------------------------------------------
                // SYMBOLES MULTI-CARACTÈRES
                // ------------------------------------------------------------
                '=' => {
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        tokens.push(Token::Eq);
                    } else if self.peek_char() == Some('>') {
                        self.next_char();
                        tokens.push(Token::FatArrow);
                    }
                }

                '!' => {
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        tokens.push(Token::Neq);
                    }
                }

                '<' => {
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        tokens.push(Token::Le);
                    }
                }

                '>' => {
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        tokens.push(Token::Ge);
                    }
                }

                ':' => {
                    if self.peek_char() == Some(':') {
                        self.next_char();
                        tokens.push(Token::DoubleColon);
                    } else {
                        tokens.push(Token::Colon);
                    }
                }

                '.' => {
                    if self.peek_char() == Some('.') {
                        self.next_char();
                        if self.peek_char() == Some('.') {
                            self.next_char();
                            tokens.push(Token::Ellipsis);
                        } else {
                            tokens.push(Token::Range);
                        }
                    }
                }

                // ------------------------------------------------------------
                // SYMBOLES SIMPLES
                // ------------------------------------------------------------
                '{' => tokens.push(Token::LBrace),
                '}' => tokens.push(Token::RBrace),
                '[' => tokens.push(Token::LBracket),
                ']' => tokens.push(Token::RBracket),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                ',' => tokens.push(Token::Comma),
                '/' => tokens.push(Token::Slash),

                // ------------------------------------------------------------
                // CARACTÈRE INCONNU
                // ------------------------------------------------------------
                _ => {
                    // On pourrait générer une erreur plus tard
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}