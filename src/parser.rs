// ------------------------------------------------------------
// OGMA — Parser 0.3.11
// ------------------------------------------------------------

use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<&Token> {
        let t = self.tokens.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn match_token(&mut self, expected: &Token) -> bool
    where
        Token: PartialEq,
    {
        if let Some(t) = self.peek() {
            if t == expected {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String>
    where
        Token: PartialEq + std::fmt::Debug,
    {
        match self.next() {
            Some(t) if t == expected => Ok(()),
            other => Err(format!("Attendu {:?}, trouvé {:?}", expected, other)),
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.next() {
            Some(Token::Ident(name)) => Ok(name.clone()),
            other => Err(format!("Identifiant attendu, trouvé {:?}", other)),
        }
    }

    // --------------------------------------------------------
    // Entrée principale : un fichier .ogma
    // --------------------------------------------------------
    pub fn parse_module_file(&mut self, name: String) -> Result<ModuleFile, String> {
        let imports = self.parse_imports()?;
        let mut declarations = Vec::new();

        while let Some(tok) = self.peek() {
            match tok {
                Token::EOF | Token::RBrace => break,
                _ => {
                    let decl = self.parse_declaration()?;
                    declarations.push(decl);
                }
            }
        }

        Ok(ModuleFile {
            name,
            imports,
            declarations,
        })
    }

    // --------------------------------------------------------
    // Imports
    // --------------------------------------------------------
    fn parse_imports(&mut self) -> Result<Vec<Import>, String> {
        let mut imports = Vec::new();

        loop {
            match self.peek() {
                Some(Token::Import) => {
                    self.next(); // 'import'
                    let module_name = self.expect_ident()?;

                    let alias = if self.match_token(&Token::As) {
                        Some(self.expect_ident()?)
                    } else {
                        None
                    };

                    imports.push(Import {
                        module: module_name,
                        alias,
                    });
                }
                _ => break,
            }
        }

        Ok(imports)
    }

    // --------------------------------------------------------
    // Déclarations (variable, fonction, objet, module)
    // --------------------------------------------------------
    fn parse_declaration(&mut self) -> Result<Decl, String> {
        match self.peek() {
            Some(Token::Module) => self.parse_module_decl(),
            Some(Token::Ident(_)) => self.parse_toplevel_named_decl(),
            Some(Token::Object) => self.parse_object_decl_toplevel(),
            other => Err(format!("Déclaration inattendue : {:?}", other)),
        }
    }

    // nom: ... au sommet
    fn parse_toplevel_named_decl(&mut self) -> Result<Decl, String> {
        let name = self.expect_ident()?;
        self.expect(&Token::Colon)?;

        match self.peek() {
            Some(Token::Fn) => {
                self.next(); // 'fn'
                self.parse_function_decl_after_name(name)
            }
            Some(Token::Object) => {
                self.next(); // 'object'
                self.parse_object_decl_after_name(name)
            }
            _ => {
                let type_hint = self.parse_type_ref()?;
                self.expect(&Token::Equal)?;
                let value = self.parse_expression()?;
                Ok(Decl::Variable(VarDecl {
                    name,
                    type_hint,
                    value,
                }))
            }
        }
    }

    // module math { ... }
    fn parse_module_decl(&mut self) -> Result<Decl, String> {
        self.next(); // 'module'
        let name = self.expect_ident()?;
        self.expect(&Token::LBrace)?;

        let mut declarations = Vec::new();
        while !self.match_token(&Token::RBrace) {
            let decl = self.parse_declaration()?;
            declarations.push(decl);
        }

        Ok(Decl::Module(NestedModuleDecl { name, declarations }))
    }

    // nom: fn(...) -> type { ... }
    fn parse_function_decl_after_name(&mut self, name: String) -> Result<Decl, String> {
        self.expect(&Token::LParen)?;
        let params = self.parse_params()?;
        self.expect(&Token::RParen)?;

        let return_type = if self.match_token(&Token::Arrow) {
            Some(self.parse_type_ref()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(Decl::Function(FuncDecl {
            name,
            params,
            return_type,
            body,
        }))
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();

        loop {
            match self.peek() {
                Some(Token::Ident(_)) => {
                    let name = self.expect_ident()?;
                    self.expect(&Token::Colon)?;
                    let type_hint = self.parse_type_ref()?;
                    params.push(Param { name, type_hint });

                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
                _ => break,
            }
        }

        Ok(params)
    }

    // object au sommet sans nom (on ne le supporte pas pour l’instant)
    fn parse_object_decl_toplevel(&mut self) -> Result<Decl, String> {
        self.next(); // 'object'
        Err("object sans nom au sommet non supporté dans cette version".to_string())
    }

    // nom: object { ... }
    fn parse_object_decl_after_name(&mut self, name: String) -> Result<Decl, String> {
        let fields = self.parse_object_fields()?;
        Ok(Decl::Object(ObjectDecl { name, fields }))
    }

    fn parse_object_fields(&mut self) -> Result<Vec<ObjectField>, String> {
        self.expect(&Token::LBrace)?;
        let mut fields = Vec::new();

        while !self.match_token(&Token::RBrace) {
            let mutable = self.match_token(&Token::Mut);
            let name = self.expect_ident()?;
            self.expect(&Token::Colon)?;
            let type_hint = self.parse_type_ref()?;
            self.expect(&Token::Equal)?;
            let value = self.parse_expression()?;

            fields.push(ObjectField {
                mutable,
                name,
                type_hint,
                value,
            });
        }

        Ok(fields)
    }

    // --------------------------------------------------------
    // Types
    // --------------------------------------------------------
    fn parse_type_ref(&mut self) -> Result<TypeRef, String> {
        match self.peek() {
            Some(Token::Ident(name)) if name == "ognum" => {
                self.next(); // 'ognum'
                self.expect(&Token::LParen)?;
                let prec = self.parse_int_literal()? as i32;
                self.expect(&Token::Comma)?;
                let scale = self.parse_int_literal()? as i32;
                self.expect(&Token::RParen)?;
                Ok(TypeRef::Ognum {
                    precision: prec,
                    scale,
                })
            }
            Some(Token::Ident(name)) => {
                let n = name.clone();
                self.next();
                Ok(TypeRef::Simple(n))
            }
            other => Err(format!("Type attendu, trouvé {:?}", other)),
        }
    }

    fn parse_int_literal(&mut self) -> Result<i64, String> {
        match self.next() {
            Some(Token::Number(n)) => n
                .parse::<i64>()
                .map_err(|_| "Nombre entier attendu".to_string()),
            other => Err(format!("Nombre entier attendu, trouvé {:?}", other)),
        }
    }

    // --------------------------------------------------------
    // Blocs
    // --------------------------------------------------------
    fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(&Token::LBrace)?;

        let mut statements = Vec::new();
        let mut last_expr: Option<Expr> = None;

        while !self.match_token(&Token::RBrace) {
            if self.is_start_of_decl_in_block() {
                let decl = self.parse_block_var_decl()?;
                statements.push(Stmt::Decl(decl));
            } else {
                let expr = self.parse_expression()?;
                // on considère que c’est une expression "statement"
                statements.push(Stmt::Expr(Box::new(expr.clone())));
                last_expr = Some(expr);
            }
        }

        Ok(Block {
            statements,
            last_expr,
        })
    }

    fn is_start_of_decl_in_block(&mut self) -> bool {
        matches!(self.peek(), Some(Token::Ident(_)) | Some(Token::Mut))
    }

    // x: int = expr  ou  mut x: int = expr
    fn parse_block_var_decl(&mut self) -> Result<VarDecl, String> {
        let _mutable = self.match_token(&Token::Mut);
        let name = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let type_hint = self.parse_type_ref()?;
        self.expect(&Token::Equal)?;
        let value = self.parse_expression()?;

        Ok(VarDecl {
            name,
            type_hint,
            value,
        })
    }

    // --------------------------------------------------------
    // Expressions (version minimale)
    // --------------------------------------------------------
    fn parse_expression(&mut self) -> Result<Expr, String> {
        // Pour l’instant : pas de gestion des opérateurs binaires,
        // seulement primary + suffixes (chemin, appel).
        self.parse_call_or_path_or_primary()
    }

    fn parse_call_or_path_or_primary(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.peek() {
                Some(Token::Dot) => {
                    self.next(); // '.'
                    let segment = self.expect_ident()?;

                    if self.match_token(&Token::LParen) {
                        let args = self.parse_args()?;
                        self.expect(&Token::RParen)?;
                        expr = Expr::MethodCall(MethodCall {
                            target: Box::new(expr),
                            method: segment,
                            args,
                        });
                    } else {
                        expr = match expr {
                            Expr::Path(mut p) => {
                                p.segments.push(segment);
                                Expr::Path(p)
                            }
                            Expr::Identifier(name) => Expr::Path(Path {
                                segments: vec![name, segment],
                            }),
                            other => {
                                return Err(format!(
                                    "Chemin invalide après expression {:?}",
                                    other
                                ))
                            }
                        };
                    }
                }
                Some(Token::LParen) => {
                    self.next(); // '('
                    let args = self.parse_args()?;
                    self.expect(&Token::RParen)?;
                    expr = Expr::Call(Call {
                        function: Box::new(expr),
                        args,
                    });
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        loop {
            match self.peek() {
                Some(Token::RParen) => break,
                _ => {
                    let expr = self.parse_expression()?;
                    args.push(expr);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
        }

        Ok(args)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Token::Number(_)) => {
                let lit = self.parse_number_literal()?;
                Ok(Expr::Literal(lit))
            }
            Some(Token::StringLiteral(_)) => {
                let lit = self.parse_string_literal()?;
                Ok(Expr::Literal(lit))
            }
            Some(Token::CharLiteral(_)) => {
                let lit = self.parse_char_literal()?;
                Ok(Expr::Literal(lit))
            }
            Some(Token::Ident(name)) if name == "true" || name == "false" => {
                let lit = self.parse_bool_literal()?;
                Ok(Expr::Literal(lit))
            }
            Some(Token::Ident(_)) => {
                let name = self.expect_ident()?;
                Ok(Expr::Identifier(name))
            }
            Some(Token::LBrace) => {
                let block = self.parse_block()?;
                Ok(Expr::Block(Box::new(block)))
            }
            Some(Token::BlockKw) => {
                self.next(); // 'block'
                let block = self.parse_block()?;
                Ok(Expr::ExplicitBlock(Box::new(block)))
            }
            Some(Token::LBracket) => {
                let dialect = self.parse_dialect_block()?;
                Ok(Expr::Dialect(dialect))
            }
            other => Err(format!("Expression inattendue : {:?}", other)),
        }
    }

    fn parse_number_literal(&mut self) -> Result<Literal, String> {
        match self.next() {
            Some(Token::Number(n)) => {
                if n.contains('.') {
                    n.parse::<f64>()
                        .map(Literal::Decimal)
                        .map_err(|_| "Décimal invalide".to_string())
                } else {
                    n.parse::<i64>()
                        .map(Literal::Int)
                        .map_err(|_| "Entier invalide".to_string())
                }
            }
            other => Err(format!("Nombre attendu, trouvé {:?}", other)),
        }
    }

    fn parse_string_literal(&mut self) -> Result<Literal, String> {
        match self.next() {
            Some(Token::StringLiteral(s)) => Ok(Literal::String(s.clone())),
            other => Err(format!("Chaîne attendue, trouvé {:?}", other)),
        }
    }

    fn parse_char_literal(&mut self) -> Result<Literal, String> {
        match self.next() {
            Some(Token::CharLiteral(c)) => Ok(Literal::Char(*c)),
            other => Err(format!("Caractère attendu, trouvé {:?}", other)),
        }
    }

    fn parse_bool_literal(&mut self) -> Result<Literal, String> {
        match self.next() {
            Some(Token::Ident(name)) if name == "true" => Ok(Literal::Bool(true)),
            Some(Token::Ident(name)) if name == "false" => Ok(Literal::Bool(false)),
            other => Err(format!("bool attendu, trouvé {:?}", other)),
        }
    }

    fn parse_dialect_block(&mut self) -> Result<DialectBlock, String> {
        self.expect(&Token::LBracket)?;
        let mut lines = Vec::new();

        while !self.match_token(&Token::RBracket) {
            match self.next() {
                Some(Token::Ident(s)) => lines.push(s.clone()),
                Some(Token::StringLiteral(s)) => lines.push(s.clone()),
                Some(Token::Number(n)) => lines.push(n.clone()),
                Some(Token::RBracket) => break,
                other => {
                    return Err(format!("Token inattendu dans dialecte : {:?}", other));
                }
            }
        }

        Ok(DialectBlock { lines })
    }
}
