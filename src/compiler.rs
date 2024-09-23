use std::{fs, io, path::Path};

use anyhow::Result;

use crate::parser::{self, Exp, Stmt};

pub fn compile(w: &mut impl io::Write, path: &Path) -> Result<()> {
    writeln!(w, "{}", include_str!("core.js"))?;
    for stmt in parser::parse(Path::new("core.bao"), include_str!("core.bao"))? {
        write_stmt(w, &stmt)?;
        writeln!(w, ";\n")?;
    }
    write_exp(
        w,
        &Exp::Call(
            Box::new(Exp::Fn(
                None,
                Vec::new(),
                parser::parse(path, &fs::read_to_string(path)?)?,
                Box::new(Exp::Var("nil")),
            )),
            Vec::new(),
        ),
    )?;
    Ok(())
}

fn write_exp(w: &mut impl io::Write, exp: &Exp) -> Result<()> {
    match exp {
        Exp::Call(exp, args) => write_call(w, exp, args, false),
        Exp::Fn(name, params, body, res) => write_fn(w, *name, params, body, res),
        Exp::Literal(n) => write_literal(w, n),
        Exp::Var(name) => write_var(w, name),
    }?;
    Ok(())
}

fn write_call(w: &mut impl io::Write, exp: &Exp, args: &[Exp], tail: bool) -> Result<()> {
    if tail {
        write!(w, "tail(")?;
    } else {
        write!(w, "call(")?;
    }
    write_exp(w, exp)?;
    for arg in args {
        write!(w, ",")?;
        write_exp(w, arg)?;
    }
    write!(w, ")")?;
    Ok(())
}

fn write_fn(
    w: &mut impl io::Write,
    name: Option<&str>,
    params: &[&str],
    body: &[Stmt],
    res: &Exp,
) -> Result<()> {
    write!(w, "function")?;
    if let Some(name) = name {
        write!(w, " ${}", name)?;
    }
    write!(w, "(")?;
    for param in params {
        write!(w, "${},", param)?;
    }
    write!(w, "){{")?;
    for stmt in body {
        write_stmt(w, stmt)?;
        write!(w, ";")?;
    }
    write!(w, "return ")?;
    write_res(w, res)?;
    write!(w, ";")?;
    write!(w, "}}")?;
    Ok(())
}

fn write_stmt(w: &mut impl io::Write, stmt: &Stmt) -> Result<()> {
    match stmt {
        Stmt::Let(name, body) => write_let(w, name, body),
        Stmt::Exp(exp) => write_exp(w, exp),
    }
}

fn write_res(w: &mut impl io::Write, exp: &Exp) -> Result<()> {
    if let Exp::Call(exp, args) = exp {
        write_call(w, exp, args, true)?;
    } else {
        write!(w, "res(")?;
        write_exp(w, exp)?;
        write!(w, ")")?;
    }
    Ok(())
}

fn write_let(w: &mut impl io::Write, name: &str, body: &Exp) -> Result<()> {
    write!(w, "const ${} = ", name)?;
    write_exp(w, body)?;
    Ok(())
}

fn write_literal(w: &mut impl io::Write, literal: &str) -> Result<()> {
    write!(w, "{}", literal)?;
    Ok(())
}

fn write_var(w: &mut impl io::Write, name: &str) -> Result<()> {
    write!(
        w,
        "${}",
        name.replace('?', "$Q")
            .replace('!', "$E")
            .replace('*', "$S")
    )?;
    Ok(())
}