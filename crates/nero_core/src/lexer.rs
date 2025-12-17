use crate::token::Token;
use std::char;
use std::fmt;
use std::str::FromStr;

///
#[derive(Debug)]
pub enum LexerError {
    /// error jika template string tidak ditutup dengan (})
    UnclosedTemplateString,
    /// error jika string tidak ditutup dengan (")
    UnclosedStringLiteral,
    /// error jika input character yang tidak dikenali
    UnknownCharacter(char),
    // error jika statement tidak ditutup dengan titik koma (;)
    // MissingSemicolon,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnknownCharacter(ch) => {
                write!(f, "Unknown character '{}'", ch)
            }
            LexerError::UnclosedStringLiteral => {
                write!(f, "Unclosed string literal")
            }
            LexerError::UnclosedTemplateString => {
                write!(f, "Unclosed template string")
            }
        }
    }
}

/// Lexer bertugas memecah source code menjadi token-token
/// yang nantinya dipakai oleh parser.
///
/// fungsi untuk memecah source code tersebut disimpan di fungsi [`Lexer::tokenize`]
///
/// didalam `Struct` ini juga menyimpan berbagai fungsi yang membantu membaca identifier,
/// string, number dan lain lain
pub struct Lexer;

impl Lexer {
    /// Membaca identifier (huruf, angka, _, atau -) mulai dari indeks `start`.
    ///
    /// # Parameter
    /// - `chars` daftar karakter dari source code
    /// - `start` posisi awal
    ///
    /// # Return
    /// Mengembalikan tuple `(identifier, next_index)`
    /// - `identifier` adalah string hasil pembacaan
    /// - `next_index` adalah posisi setelah identifier selesai dibaca
    fn read_identifier(chars: &[char], start: usize) -> (String, usize) {
        let mut i = start;
        let mut ident: String = String::new();

        while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '-')
        {
            ident.push(chars[i]);
            i += 1;
        }

        (ident, i)
    }

    /// Membaca string literal dari source code
    ///
    /// # Return
    /// Mengembalikan index setelah string_literal selesai dibaca
    fn read_string(
        tokens: &mut Vec<Token>,
        chars: &Vec<char>,
        start: usize,
    ) -> Result<usize, LexerError> {
        let mut part = String::new();
        let mut i = start + 1;

        while i < chars.len() {
            let ch = chars[i];

            if ch == '$' && i + 1 < chars.len() && chars[i + 1] == '{' {
                if !part.is_empty() {
                    tokens.push(Token::StringLiteral(part.clone()));
                    part.clear();
                }

                tokens.push(Token::TemplateStart);
                i += 2;

                let (ident, next) = Lexer::read_identifier(chars, i);
                tokens.push(Token::Identifier(ident));
                i = next;

                if chars[i] != '}' {
                    return Err(LexerError::UnclosedTemplateString);
                }

                tokens.push(Token::TemplateEnd);
                i += 1;
                continue;
            }
            if ch == '"' {
                if !part.is_empty() {
                    tokens.push(Token::StringLiteral(part.clone()));
                }
                return Ok(i + 1);
            }
            part.push(ch);
            i += 1;
        }
        Err(LexerError::UnclosedStringLiteral)
    }

    /// Membaca number literal dari source code
    ///
    /// # Return
    /// Mengembalikan tuple `(number, next_index)`
    /// - `number` adalah string hasil pembacaan
    /// - `next_index` adalah posisi setelah number selesai dibaca
    fn read_number(chars: &Vec<char>, start: usize) -> (i64, usize) {
        let mut i = start;
        let mut num_str: String = String::new();

        while i < chars.len() && chars[i].is_ascii_digit() {
            num_str.push(chars[i]);
            i += 1;
        }

        let num: i64 = i64::from_str(&num_str).unwrap();

        (num, i)
    }

    /// Fungsi untuk membantu menentukan HTTP Method dari `String`
    ///
    /// # Return
    /// Mengembalikan enum [`HttpMethod`]
    // fn read_method(method: &String) -> HttpMethod {
    //     let m = method.to_uppercase();
    //     if &m == "GET" {
    //         return HttpMethod::GET;
    //     }
    //     if &m == "POST" {
    //         return HttpMethod::POST;
    //     }
    //     if &m == "DELETE" {
    //         return HttpMethod::DELETE;
    //     }
    //     if &m == "PATCH" {
    //         return HttpMethod::PATCH;
    //     }
    //     if &m == "PUT" {
    //         return HttpMethod::PUT;
    //     }
    //     return HttpMethod::UNKNOWN;
    // }

    /// Fungsi untuk melakukan tokenisasi dan pengkategorian tipe token
    ///
    /// lexer akan:
    /// - melewati whitespace
    /// - membaca simbol seperti `@`, `{}`, `[]`, `#`, `:`, `=`, `,`
    /// - membaca method setelah karakter `@`
    /// - membaca string literal `"..."`
    /// - membaca angka
    /// - membaca identifier
    ///
    /// # Contoh
    /// ```
    /// let source_code = "port = 3000";
    /// let tokens: Vec<Token> = tokenize(source_code);
    /// ```
    ///
    /// # Return
    /// Vector berisi enum [`Token`] hasil lexing. Contoh:
    /// ```
    /// Identifier("port")
    /// Equals
    /// NumberLiteral("3000")
    /// ```
    pub fn tokenize(source_code: &str) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();

        let chars: Vec<char> = source_code.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            if ch.is_whitespace() {
                i += 1;
                continue;
            }

            match ch {
                '@' => {
                    tokens.push(Token::At);
                    // let (m, next) = Self::read_identifier(&chars, i + 1);
                    // tokens.push(Token::Method(Self::read_method(&m)));
                    i += 1;
                    continue;
                }
                '#' => {
                    tokens.push(Token::Hash);
                    i += 1;
                    continue;
                }
                '=' => {
                    tokens.push(Token::Equals);
                    i += 1;
                    continue;
                }
                ':' => {
                    tokens.push(Token::Colon);
                    i += 1;
                    continue;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    i += 1;
                    continue;
                }
                '{' => {
                    tokens.push(Token::OpenBrace);
                    i += 1;
                    continue;
                }
                '}' => {
                    tokens.push(Token::CloseBrace);
                    i += 1;
                    continue;
                }
                '[' => {
                    tokens.push(Token::OpenBracket);
                    let (label, next) = Self::read_identifier(&chars, i + 1);
                    tokens.push(Token::Label(label));
                    i = next;
                    continue;
                }
                ']' => {
                    tokens.push(Token::CloseBracket);
                    i += 1;
                    continue;
                }
                ';' => {
                    tokens.push(Token::SemiColon);
                    i += 1;
                    continue;
                }
                _ => {}
            };

            if ch == '"' {
                i = Self::read_string(&mut tokens, &chars, i)?;
                continue;
            }

            if ch.is_numeric() {
                let (num, next) = Self::read_number(&chars, i);
                tokens.push(Token::NumberLiteral(num));
                i = next;
                continue;
            }

            if ch.is_alphabetic() || ch == '_' {
                let (ident, next) = Self::read_identifier(&chars, i);
                tokens.push(Token::Identifier(ident));
                i = next;
                continue;
            }
            return Err(LexerError::UnknownCharacter(ch));
        }
        Ok(tokens)
    }
}
