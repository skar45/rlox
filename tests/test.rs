use std::fs;

mod common;

const VAR: &'static str = "./tests/var/var.rlox";
const VAR_RES: &'static str = "./tests/var/result";
const VAR_SCOPE: &'static str = "./tests/scope/scope.rlox";
const VAR_SCOPE_RES: &'static str = "./tests/scope/result";
const FUN_DECL: &'static str = "./tests/fun/fun.rlox";
const FUN_DECL_RES: &'static str = "./tests/fun/result";
const EXPR: &'static str = "./tests/expressions/expressions.rlox";
const EXPR_RES: &'static str = "./tests/expressions/result";
const LOOPS: &'static str = "./tests/loops/loops.rlox";
const LOOPS_RES: &'static str = "./tests/loops/result";

#[test]
fn test_var_declarations() {
    let res = fs::read_to_string(VAR_RES).unwrap();
    let mut cmd = common::setup();
    cmd.arg(VAR).assert().success().stdout(res);
}

#[test]
fn test_var_scope() {
    let res = fs::read_to_string(VAR_SCOPE_RES).unwrap();
    let mut cmd = common::setup();
    cmd.arg(VAR_SCOPE).assert().success().stdout(res);
}

#[test]
fn test_function() {
    let res = fs::read_to_string(FUN_DECL_RES).unwrap();
    let mut cmd = common::setup();
    cmd.arg(FUN_DECL).assert().success().stdout(res);
}

#[test]
fn test_expressions() {
    let res = fs::read_to_string(EXPR_RES).unwrap();
    let mut cmd = common::setup();
    cmd.arg(EXPR).assert().success().stdout(res);
}

#[test]
fn test_loops() {
    let res = fs::read_to_string(LOOPS_RES).unwrap();
    let mut cmd = common::setup();
    cmd.arg(LOOPS).assert().success().stdout(res);
}
