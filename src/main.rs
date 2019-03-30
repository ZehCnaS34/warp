#![allow(dead_code)]
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fs;

#[derive(Debug)]
struct Token {
    value: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
enum Atom {
    Symbol(String),
    Keyword(String),
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Reference(usize),
}

impl Atom {
    fn infer(src: &Token) -> Option<Atom> {
        let src = src.value.clone();
        // handling the boolean case
        match src.as_str() {
            "true" => {
                return Some(Atom::Boolean(true));
            }
            "false" => {
                return Some(Atom::Boolean(false));
            }
            _ => (),
        }

        // int
        if let Ok(value) = src.parse::<i64>() {
            return Some(Atom::Int(value));
        }

        // float
        if let Ok(value) = src.parse::<f64>() {
            return Some(Atom::Float(value));
        }

        Some(Atom::Symbol(src))
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Boolean(v) => write!(f, "{}", v),
            Atom::Float(v) => write!(f, "{}", v),
            Atom::Int(v) => write!(f, "{}", v),
            Atom::Keyword(v) => write!(f, "{}", v),
            Atom::Reference(v) => write!(f, "%{}", v),
            Atom::String(v) => write!(f, "{}", v),
            Atom::Symbol(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug)]
struct SExp {
    _type: &'static str,
    children: Vec<Atom>,
}

impl SExp {
    fn new(t: &Token) -> SExp {
        let _type = match t.value.as_str() {
            "(" => "exec",
            "[" => "vec",
            "{" => "map",
            "\"" => "string",
            "'" => "list",
            _ => panic!("Unsupported type"),
        };

        SExp {
            _type,
            children: vec![],
        }
    }

    fn push(&mut self, atom: Atom) {
        self.children.push(atom);
    }
}

impl fmt::Display for SExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = format!("({}", self._type);
        let mut body: Vec<String> = vec![];
        for child in &self.children {
            body.push(child.to_string());
        }
        let body = body.join(" ");
        let output = format!("{} {})", output, body);

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
struct AST {
    items: HashMap<usize, SExp>,
}

impl AST {
    fn tokenize(src: String) -> Vec<Token> {
        let mut strings: Vec<String> = vec![];

        for c in src.chars() {
            if let Some(token) = strings.last() {
                match token.as_str() {
                    "{" | "}" | "[" | "]" | "(" | ")" | " " | "\n" | "\"" | "'" | "@" | "~"
                    | "`" => strings.push(c.to_string()),
                    _ => match c {
                        '}' | '{' | ']' | '[' | ' ' | ')' | '(' | '\n' | '"' | '\'' | '@' | '~'
                        | '`' => strings.push(c.to_string()),
                        _ => {
                            let mut token = strings.pop().unwrap();
                            token += c.to_string().as_str();
                            strings.push(token);
                        }
                    },
                }
            } else {
                strings.push(c.to_string());
            }
        }

        let mut tokens: Vec<Token> = vec![];

        for s in strings {
            match s.as_str() {
                " " | "\n" | "\t" => (),
                _ => {
                    tokens.push(Token { value: s });
                }
            }
        }

        tokens
    }

    fn read(tokens: &Vec<Token>) -> AST {
        let mut items: HashMap<usize, SExp> = HashMap::new();
        let mut sexps: Vec<SExp> = vec![];
        let mut id: usize = 0;
        let mut ids: VecDeque<usize> = VecDeque::new();

        for token in tokens {
            match token.value.as_str() {
                "\"" => {}
                "'" => {}
                "(" | "[" | "{" => {
                    if let Some(sexp) = sexps.last_mut() {
                        sexp.push(Atom::Reference(id));
                    }
                    ids.push_back(id);
                    id += 1;
                    sexps.push(SExp::new(token));
                }
                ")" | "]" | "}" => {
                    items.insert(
                        ids.pop_back().expect("No more items left"),
                        sexps.pop().unwrap(),
                    );
                }
                _ => {
                    let atom = Atom::infer(token).unwrap();
                    if let Some(sexp) = sexps.last_mut() {
                        sexp.push(atom);
                    }
                }
            }
        }

        AST { items }
    }
}

struct ENV {}

fn eval(ast: &AST, pc: usize) {
    let reference = pc;
    let mut pc: usize = pc;
    while ast.items.len() > pc {
        if let Some(sexp) = ast.items.get(&pc) {
            println!("{} {}", pc, sexp);
            for atom in &sexp.children {
                match atom {
                    Atom::Reference(n) => {
                        if n > &pc {
                            pc = n + 1;
                        }
                        eval(&ast, n.clone());
                    }
                    _atom => {}
                }
            }
            if pc == reference {
                return;
            }
            pc += 1;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("./src.clj").expect("Could not read file.");

    // NOTE: These two lines should probably be one. (I think).
    let tokens = AST::tokenize(contents);
    let ast = AST::read(&tokens);

    // println!("{:#?}", ast);
    // println!("{:#?}", tokens);

    // for (id, sexp) in &ast.items {
    //     println!("{} {}", id, sexp);
    // }

    eval(&ast, 0);
    return;

    let max = ast.items.len();
    for i in 0..max {
        println!("{}\t{}", i, ast.items[&i]);
    }
}
