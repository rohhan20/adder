use std::env;
use std::fs::File;
use std::io::prelude::*;
use sexp::*;
use sexp::Atom::*;

// fn compile(program: String) -> String {
//     let num = program.trim().parse::<i32>().unwrap();
//     return format!("mov rax, {}", num);
// }

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let expr = parse_expr(&parse(&in_contents).unwrap());
    let result = compile_expr(&expr);
    let asm_program = format!("
section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
", result);

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}


#[derive(Debug)]
enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Negate(Box<Expr>),
}

// fn eval(e: &Expr) -> i32 {
//     match e {
//         Expr::Num(n) => *n,
//         Expr::Add1(e1) => eval(e1) + 1,
//         Expr::Sub1(e1) => eval(e1) - 1,
//         Expr::Negate(e1) => -eval(e1),
//     }
// }

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), e] if op == "add1" => Expr::Add1(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::Sub1(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "negate" => Expr::Negate(Box::new(parse_expr(e))),
                _ => panic!("parse error: Unrecognized list {:?}", vec),
            }
        },
        _ => panic!("parse error"),
    }
}

fn compile_expr(e: &Expr) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", *n),
        Expr::Add1(subexpr) => compile_expr(subexpr) + "\nadd rax, 1",
        Expr::Sub1(subexpr) => compile_expr(subexpr) + "\nsub rax, 1",
        Expr::Negate(subexpr) => compile_expr(subexpr) + "\nneg rax",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
      let expr1 = Expr::Num(10);
      let result = eval(&expr1);
      assert_eq!(result, 10);
    }

    #[test]
    fn test_num() {
        let expr = Expr::Num(10);
        assert_eq!(eval(&expr), 10);
    }

    #[test]
    fn test_add1() {
        let expr = Expr::Add1(Box::new(Expr::Num(5)));
        assert_eq!(eval(&expr), 6);
    }

    #[test]
    fn test_sub1() {
        let expr = Expr::Sub1(Box::new(Expr::Num(5)));
        assert_eq!(eval(&expr), 4);
    }

    #[test]
    fn test_negate() {
        let expr = Expr::Negate(Box::new(Expr::Num(5)));
        assert_eq!(eval(&expr), -5);
    }

    #[test]
    fn test_nested_expression() {
        let expr = Expr::Negate(Box::new(
            Expr::Add1(Box::new(
                Expr::Sub1(Box::new(Expr::Num(73)))
            ))
        ));
        assert_eq!(eval(&expr), -73);
    }
}
