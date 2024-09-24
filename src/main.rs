use ebnf::Expression;


const GRAMMAR: &str  = r"message       ::= ['@' tags SPACE] [':' prefix SPACE ] command [params] crlf;
tags          ::= tag [';' tag]*;
tag           ::= key ['=' escaped_value];
key           ::= [ client_prefix ] [ vendor '/' ] key_name;
client_prefix ::= '+';
";

fn thing(rule: &Expression) -> String {
    let name = &rule.lhs;
    

    format!("fn parse_{name}(input: &mut &str) -> winnow::prelude::PResult<char>
    {{
    }}")
}


fn main() {
    let g = ebnf::get_grammar(GRAMMAR).unwrap().expressions;
    
    println!("{}", thing(g.first().unwrap()));
}

