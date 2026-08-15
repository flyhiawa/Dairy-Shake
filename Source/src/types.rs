pub struct SymName(String);

impl SymName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.contains(' ') || name.is_empty() {
            return Err("d".into());
        }

        Ok(SymName(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub enum Types {
	Sym(SymName),
	Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
	Create,
	Background,
	Fun,
	Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Identifier(String),
	Type(String),
	Text(String),
	Number(f64),
}

pub enum Token {
	Key(Keyword),
	Lit(Literal),
	Equal,
}

impl Keyword {
	pub fn keywords(s: &str) -> Option<Keyword> {
		match s {
			"cvar" => Some(Keyword::Create),
			"__bg" => Some(Keyword::Background),
			"fun" => Some(Keyword::Fun),
			"return" => Some(Keyword::Return),
			_ => None,
		}
	}
}