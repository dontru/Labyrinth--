pub fn run(code: &str) -> Result<String, String> {
    let lines = Lang::lex(code);

    let syntax = match Lang::parse(lines) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    Ok(String::new())
}

struct Lang;

impl Lang {
    fn lex(code: &str) -> Vec<Line> {
        code.lines()
            .map(|s| Line::from(s))
            .collect()
    }

    fn parse(lines: Vec<Line>) -> Result<Syntax, String> {
        let code_lines = match Line::ringfence_code(&lines) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        Ok(Syntax::new(code_lines))
    }
}

#[derive(PartialEq)]
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

    fn is_code(&self) -> bool {
        if let Line::Code(_) = self {
            true
        } else {
            false
        }
    }

    fn ringfence_code(lines: &[Line]) -> Result<&[Line], String> {
        let start_idx = match lines.iter().position(|x| x.is_code()) {
            Some(idx) => idx,
            None => return Err(String::from("Cannot find any code line")),
        };

        let end_idx = lines.iter().rposition(|x| x.is_code()).unwrap();

        if start_idx > 0 && lines[start_idx-1] != Line::Empty {
            return Err(String::from("No empty line between comments and code"));
        }

        if end_idx < lines.len()-1 && lines[end_idx+1] != Line::Empty {
            return Err(String::from("No empty line between comments and code"));
        }

        Ok(&lines[start_idx..end_idx+1])
    }
}

#[derive(PartialEq)]
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

struct Syntax;

impl Syntax {
    fn new(code_lines: &[Line]) -> Syntax {
        Syntax {}
    }
}

