pub fn run(code: &str) -> Result<String, String> {
    let lines = Lang::lex(code);
    Ok(String::new())
}

struct Lang;

impl Lang {
    fn lex(code: &str) -> Vec<Line> {
        code.lines()
            .map(|s| Line::from(s))
            .collect()
    }
}

enum Line {
    Code(Vec<Token>),
    Comment,
    Empty,
}

impl Line {
    fn from(s: &str) -> Line {
        if s.len() == 0 {
            return Line::Empty;
        }

        let mut tokens = Vec::new();

        for character in s.chars() {
            if let Some(token) = Token::from(character) {
                tokens.push(token);
            } else {
                return Line::Comment;
            }
        }

        Line::Code(tokens)
    }
}

enum Token {
    Wall,
    Space,
    A,
    B,
}

impl Token {
    fn from(character: char) -> Option<Token> {
        match character {
            '#' => Some(Token::Wall),
            ' ' => Some(Token::Space),
            'A' => Some(Token::A),
            'B' => Some(Token::B),
            _ => None,
        }
    }
}
