/// Daftar HTTP Method yang didukung
///
/// jika sebuah method tidak dikenali, maka akan digunakan [`HttpMethod::UNKNOWN`].
// #[derive(Debug, PartialEq, Clone)]
// pub enum HttpMethod {
//     /// HTTP GET request
//     GET,
//     /// HTTP POST request
//     POST,
//     /// HTTP PUT request
//     PUT,
//     /// HTTP PATCH request
//     PATCH,
//     /// HTTP DELETE request
//     DELETE,
//     /// Method yang tidak dikenal oleh lexer
//     UNKNOWN,
// }

/// Daftar token yang dapat dihasilkan oleh lexer dari source code.
///
/// digunakan untuk pengkategorian token
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Karakter `@`
    /// digunakan untuk mendefinisikan HTTP method.
    At,
    /// Karakter `#`
    /// digunakan sebagai label.
    Hash,
    /// Karakter `=`
    Equals,
    /// Karakter `:`
    Colon,
    /// Karakter `;`
    SemiColon,
    /// Karakter `,`
    Comma,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,

    TemplateStart,
    TemplateEnd,

    /// Identifier umum, misalnya nama variabel atau nama field.
    Identifier(String),

    /// HTTP method.
    ///
    /// cek enum [`HttpMethod`] untuk melihat datfar
    /// method yang didukung.
    // Method(HttpMethod),

    /// Literal string `"..."`.
    StringLiteral(String),

    /// Label pada setiap Http request yang dibuat
    Label(String),

    /// Literal angka, contoh: `123` atau `42`.
    NumberLiteral(i64),
}
