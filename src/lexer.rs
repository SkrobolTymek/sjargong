use std::io;

#[derive(Error, Debug)]
pub enum LexerError{
    #[error("")]
    FileIO(#[from] io::Error),

    MissingExpectedSymbol {
        
    }
}