// ------------------------------------------------------------
// OGMA — Module Lexer (aligné 0.3.11, version simple)
// ------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Identifiants et mots-clés
    Ident(String),

    // Littéraux
    Number(String),
    StringLiteral(String),
    CharLiteral(char),

    // Mots-clés structurants
    Import,
    As,
    Module,
    Fn,
    Object,
    Mut,
    BlockKw, // mot-clé 'block'

    // Booléens (optionnel : on peut aussi les laisser en Ident)
    // True,
    // False,

    // Symboles simples
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    LParen,     // (
    RParen,     // )
    Colon,      // :
    Comma,      // ,
    Dot,        // .
    Equal,      // =
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

    fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            None
        } else {
            let c = self.chars[self.pos];
            self.pos += 1;
            Some(c)
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).cloned()
    }

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

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.next_char() {
            match c {
                // ESPACES
                c if c.is_whitespace() => continue,

                // NOMBRES (entiers / décimaux, avec éventuel signe -)
                c if c.is_ascii_digit() || c == '-' => {
                    // '-' peut être un symbole ou le début d'un nombre
                    if c == '-' && self.peek_char() == Some('>') {
                        self.next_char();
                        tokens.push(Token::Arrow);
                        continue;
                    }

                    let mut number = c.to_string();
                    number.push_str(&self.consume_while(|ch| ch.is_ascii_digit() || ch == '.'));
                    tokens.push(Token::Number(number));
                }

                // IDENTIFIANTS / MOTS-CLÉS
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let mut ident = c.to_string();
                    ident.push_str(&self.consume_while(|ch| ch.is_ascii_alphanumeric() || ch == '_'));

                    let tok = match ident.as_str() {
                        "import" => Token::Import,
                        "as" => Token::As,
                        "module" => Token::Module,
                        "fn" => Token::Fn,
                        "object" => Token::Object,
                        "mut" => Token::Mut,
                        "block" => Token::BlockKw,
                        // "true" => Token::True,
                        // "false" => Token::False,
                        _ => Token::Ident(ident),
                    };

                    tokens.push(tok);
                }

                // CHAÎNES "..."
                '"' => {
                    let content = self.consume_while(|ch| ch != '"');
                    self.next_char(); // guillemet fermant
                    tokens.push(Token::StringLiteral(content));
                }

                // CARACTÈRES 'A'
                '\'' => {
                    let content = self.next_char();
                    let _ = self.next_char(); // consomme l'apostrophe fermante (naïf)
                    if let Some(ch) = content {
                        tokens.push(Token::CharLiteral(ch));
                    }
                }

                // SYMBOLES MULTI-CARACTÈRES
                '=' => {
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        tokens.push(Token::Eq);
                    } else if self.peek_char() == Some('>') {
                        self.next_char();
                        tokens.push(Token::FatArrow);
                    } else {
                        tokens.push(Token::Equal);
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
                    } else {
                        tokens.push(Token::Dot);
                    }
                }

                // SYMBOLES SIMPLES
                '{' => tokens.push(Token::LBrace),
                '}' => tokens.push(Token::RBrace),
                '[' => tokens.push(Token::LBracket),
                ']' => tokens.push(Token::RBracket),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                ',' => tokens.push(Token::Comma),

                // CARACTÈRE INCONNU
                _ => {
                    // tu pourras plus tard générer une erreur lexicale
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}
