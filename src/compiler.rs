use std::io;

use anyhow::Result;

use crate::parser::{Exp, ExpKind};

pub fn write_js(w: &mut impl io::Write, exp: &Exp) -> Result<()> {
    write!(w, "{}", include_str!("core.js"))?;
    write_exp(w, exp)?;
    Ok(())
}

fn write_exp(w: &mut impl io::Write, exp: &Exp) -> Result<()> {
    match &exp.kind {
        ExpKind::Call(exp, args) => write_call(w, exp, args, false),
        ExpKind::Fn(name, params, body) => write_fn(w, *name, params, body),
        ExpKind::Let(name, body) => write_let(w, name, body),
        ExpKind::Num(n) => write_num(w, n),
        ExpKind::String(s) => write_string(w, s),
        ExpKind::Var(name) => write_var(w, name),
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
    body: &[Exp],
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
    if let Some(last) = body.last() {
        for exp in body.iter().take(body.len() - 1) {
            write_exp(w, exp)?;
            write!(w, ";")?;
        }
        write!(w, "return ")?;
        write_res(w, last)?;
        write!(w, ";")?;
    }
    write!(w, "}}")?;
    Ok(())
}

fn write_res(w: &mut impl io::Write, exp: &Exp) -> Result<()> {
    if let ExpKind::Call(exp, args) = &exp.kind {
        write_call(w, exp, args, true)?;
    } else {
        write!(w, "res(")?;
        write_exp(w, exp)?;
        write!(w, ")")?;
    }
    Ok(())
}

// TODO this is not an expression, change the grammar to ensure it can only appear in a block.
fn write_let(w: &mut impl io::Write, name: &str, body: &Exp) -> Result<()> {
    write!(w, "const ${} = ", name)?;
    write_exp(w, body)?;
    Ok(())
}

fn write_num(w: &mut impl io::Write, n: &str) -> Result<()> {
    write!(w, "{}", n)?;
    Ok(())
}

fn write_string(w: &mut impl io::Write, s: &str) -> Result<()> {
    write!(w, "\"{}\"", s)?;
    Ok(())
}

fn write_var(w: &mut impl io::Write, name: &str) -> Result<()> {
    write!(w, "${}", name)?;
    Ok(())
}
