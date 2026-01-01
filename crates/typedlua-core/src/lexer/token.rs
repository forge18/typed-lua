use crate::span::Span;

/// Token kind representing different types of lexical elements
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Const,
    Local,
    Function,
    Return,
    If,
    Elseif,
    Else,
    Then,
    End,
    While,
    Do,
    For,
    In,
    Break,
    Continue,
    Repeat,
    Until,
    And,
    Or,
    Not,
    True,
    False,
    Nil,
    Interface,
    Type,
    Enum,
    Export,
    Import,
    From,
    As,
    Match,
    When,
    Class,
    Extends,
    Implements,
    Public,
    Private,
    Protected,
    Static,
    Abstract,
    Readonly,

    // Identifiers and Literals
    Identifier(String),
    Number(String),
    String(String),
    TemplateString(Vec<TemplatePart>),

    // Operators
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Caret,        // ^
    Hash,         // #
    Ampersand,    // &
    Pipe,         // |
    Tilde,        // ~
    LessThan,     // <
    LessEqual,    // <=
    GreaterThan,  // >
    GreaterEqual, // >=
    Equal,        // =
    EqualEqual,   // ==
    BangEqual,    // !=
    TildeEqual,   // ~=
    Dot,          // .
    DotDot,       // ..
    DotDotDot,    // ...
    Arrow,        // ->
    FatArrow,     // =>
    PipeOp,       // |>
    Question,     // ?
    Colon,        // :
    ColonColon,   // ::
    Bang,         // !
    At,           // @

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;

    // Special
    Eof,
    Unknown(char),
}

/// Part of a template literal
#[derive(Debug, Clone, PartialEq)]
pub enum TemplatePart {
    String(String),
    Expression(Vec<Token>),
}

/// A token with its kind and location
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn eof(position: usize) -> Self {
        Self {
            kind: TokenKind::Eof,
            span: Span::new(position, position, 0, 0),
        }
    }
}

impl TokenKind {
    /// Check if this token kind is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Const
                | TokenKind::Local
                | TokenKind::Function
                | TokenKind::Return
                | TokenKind::If
                | TokenKind::Elseif
                | TokenKind::Else
                | TokenKind::Then
                | TokenKind::End
                | TokenKind::While
                | TokenKind::Do
                | TokenKind::For
                | TokenKind::In
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Repeat
                | TokenKind::Until
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::Not
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Nil
                | TokenKind::Interface
                | TokenKind::Type
                | TokenKind::Enum
                | TokenKind::Export
                | TokenKind::Import
                | TokenKind::From
                | TokenKind::As
                | TokenKind::Match
                | TokenKind::When
                | TokenKind::Class
                | TokenKind::Extends
                | TokenKind::Implements
                | TokenKind::Public
                | TokenKind::Private
                | TokenKind::Protected
                | TokenKind::Static
                | TokenKind::Abstract
                | TokenKind::Readonly
        )
    }

    /// Get keyword from string
    pub fn from_keyword(s: &str) -> Option<Self> {
        match s {
            "const" => Some(TokenKind::Const),
            "local" => Some(TokenKind::Local),
            "function" => Some(TokenKind::Function),
            "return" => Some(TokenKind::Return),
            "if" => Some(TokenKind::If),
            "elseif" => Some(TokenKind::Elseif),
            "else" => Some(TokenKind::Else),
            "then" => Some(TokenKind::Then),
            "end" => Some(TokenKind::End),
            "while" => Some(TokenKind::While),
            "do" => Some(TokenKind::Do),
            "for" => Some(TokenKind::For),
            "in" => Some(TokenKind::In),
            "break" => Some(TokenKind::Break),
            "continue" => Some(TokenKind::Continue),
            "repeat" => Some(TokenKind::Repeat),
            "until" => Some(TokenKind::Until),
            "and" => Some(TokenKind::And),
            "or" => Some(TokenKind::Or),
            "not" => Some(TokenKind::Not),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            "nil" => Some(TokenKind::Nil),
            "interface" => Some(TokenKind::Interface),
            "type" => Some(TokenKind::Type),
            "enum" => Some(TokenKind::Enum),
            "export" => Some(TokenKind::Export),
            "import" => Some(TokenKind::Import),
            "from" => Some(TokenKind::From),
            "as" => Some(TokenKind::As),
            "match" => Some(TokenKind::Match),
            "when" => Some(TokenKind::When),
            "class" => Some(TokenKind::Class),
            "extends" => Some(TokenKind::Extends),
            "implements" => Some(TokenKind::Implements),
            "public" => Some(TokenKind::Public),
            "private" => Some(TokenKind::Private),
            "protected" => Some(TokenKind::Protected),
            "static" => Some(TokenKind::Static),
            "abstract" => Some(TokenKind::Abstract),
            "readonly" => Some(TokenKind::Readonly),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_detection() {
        assert!(TokenKind::Const.is_keyword());
        assert!(TokenKind::Function.is_keyword());
        assert!(!TokenKind::Plus.is_keyword());
        assert!(!TokenKind::Identifier("test".to_string()).is_keyword());
    }

    #[test]
    fn test_from_keyword() {
        assert_eq!(TokenKind::from_keyword("const"), Some(TokenKind::Const));
        assert_eq!(
            TokenKind::from_keyword("function"),
            Some(TokenKind::Function)
        );
        assert_eq!(
            TokenKind::from_keyword("interface"),
            Some(TokenKind::Interface)
        );
        assert_eq!(TokenKind::from_keyword("notakeyword"), None);
    }
}
