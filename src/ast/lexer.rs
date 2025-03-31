pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
}

pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan{
    pub fn new(start: usize, literal: String ) -> Self{
        Self{ start, end, literal }
    }
    pub fn length(&self) -> usize{
        self.end - self.start
    }
}

pub struct Token{
    kind: TokenKind,
    span: TextSpan
}


impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) ->self{
        Self { kind, span }
    }
}

pub struct Lexer<'a>{
    input: Peekable<Chars<'a>>
}

impl <'a> Lexer<'a>{
    pub fn new(input: &'a str) -> self{
        Self { input:input.chars().peekable() }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        
    }
}