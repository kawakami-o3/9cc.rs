// Intermediate representation
#![allow(dead_code, non_camel_case_types)]

use crate::parse::*;
use std::collections::HashMap;
use std::sync::Mutex;

fn to_ir_type(node_type: &NodeType) -> IRType {
    match node_type {
        NodeType::ADD => IRType::ADD,
        NodeType::SUB => IRType::SUB,
        NodeType::MUL => IRType::MUL,
        NodeType::DIV => IRType::DIV,
        _ => {
            panic!(format!("unknown NodeType {:?}", node_type));
        }
    }
}

lazy_static! {
    static ref CODE: Mutex<Vec<IR>> = Mutex::new(Vec::new());
    static ref REGNO: Mutex<i32> = Mutex::new(1);
    static ref BASEREG: Mutex<i32> = Mutex::new(0);
    static ref VARS: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
    static ref BPOFF: Mutex<i32> = Mutex::new(0);
    static ref LABEL: Mutex<i32> = Mutex::new(0);
    static ref IRINFO: Mutex<Vec<IRInfo>> = Mutex::new(vec![
        IRInfo {
            op: IRType::ADD,
            name: String::from("+"),
            ty: IRInfoType::REG_REG
        },
        IRInfo {
            op: IRType::SUB,
            name: String::from("-"),
            ty: IRInfoType::REG_REG
        },
        IRInfo {
            op: IRType::MUL,
            name: String::from("*"),
            ty: IRInfoType::REG_REG
        },
        IRInfo {
            op: IRType::DIV,
            name: String::from("/"),
            ty: IRInfoType::REG_REG
        },
        IRInfo {
            op: IRType::IMM,
            name: String::from("MOV"),
            ty: IRInfoType::REG_IMM
        },
        IRInfo {
            op: IRType::ADD_IMM,
            name: String::from("ADD"),
            ty: IRInfoType::REG_IMM
        },
        IRInfo {
            op: IRType::MOV,
            name: String::from("MOV"),
            ty: IRInfoType::REG_REG
        },
        IRInfo {
            op: IRType::LABEL,
            name: String::new(),
            ty: IRInfoType::LABEL
        },
        IRInfo {
            op: IRType::JMP,
            name: String::from("JMP"),
            ty: IRInfoType::LABEL
        },
        IRInfo {
            op: IRType::UNLESS,
            name: String::from("UNLESS"),
            ty: IRInfoType::REG_LABEL
        },
        IRInfo {
            op: IRType::CALL,
            name: String::from("CALL"),
            ty: IRInfoType::CALL
        },
        IRInfo {
            op: IRType::RETURN,
            name: String::from("RET"),
            ty: IRInfoType::REG
        },
        IRInfo {
            op: IRType::ALLOCA,
            name: String::from("ALLOCA"),
            ty: IRInfoType::REG_IMM
        },
        IRInfo {
            op: IRType::LOAD,
            name: String::from("LOAD"),
            ty: IRInfoType::REG_REG
        },
        IRInfo {
            op: IRType::STORE,
            name: String::from("STORE"),
            ty: IRInfoType::REG_REG
        },
        IRInfo {
            op: IRType::KILL,
            name: String::from("KILL"),
            ty: IRInfoType::REG
        },
        IRInfo {
            op: IRType::NOP,
            name: String::from("NOP"),
            ty: IRInfoType::NOARG
        },
        IRInfo {
            op: IRType::NULL,
            name: String::new(),
            ty: IRInfoType::NULL },
    ]);
}

fn basereg() -> i32 {
    *BASEREG.lock().unwrap()
}

fn regno() -> i32 {
    *REGNO.lock().unwrap()
}

fn inc_regno() {
    let mut regno = REGNO.lock().unwrap();
    *regno += 1;
}

fn bpoff() -> i32 {
    *BPOFF.lock().unwrap()
}

#[derive(Clone, Debug, PartialEq)]
pub enum IRType {
    IMM,
    ADD_IMM,
    MOV,
    RETURN,
    CALL,
    LABEL,
    JMP,
    UNLESS,
    ALLOCA,
    LOAD,
    STORE,
    KILL,
    ADD,
    SUB,
    MUL,
    DIV,
    NOP,
    NULL,
}

#[derive(Clone, Debug)]
pub struct IR {
    pub op: IRType,
    pub lhs: i32,
    pub rhs: i32,

    pub name: String,
    pub nargs: usize,
    pub args: [i32; 6],

    // Function
    pub ir: Vec<IR>,
}

#[derive(Clone, Debug)]
pub enum IRInfoType {
    NOARG,
    REG,
    LABEL,
    REG_REG,
    REG_IMM,
    REG_LABEL,
    CALL,
    NULL,
}

#[derive(Clone, Debug)]
pub struct IRInfo {
    pub op: IRType,
    pub name: String,
    pub ty: IRInfoType,
}

pub fn get_irinfo(ir: &IR) -> IRInfo {
    let irinfo = IRINFO.lock().unwrap();
    for i in 0..irinfo.len() {
        if irinfo[i].op == ir.op {
            return irinfo[i].clone();
        }
    }
    panic!("invalid instruction");
}

fn tostr(ir: IR) -> String {
    let info = get_irinfo(&ir);
    return match info.ty {
        IRInfoType::LABEL => format!(".L{}:", ir.lhs),
        IRInfoType::REG => format!("{} r{}", info.name, ir.lhs),
        IRInfoType::REG_REG => format!("{} r{}, r{}", info.name, ir.lhs, ir.rhs),
        IRInfoType::REG_IMM => format!("{} r{}, {}", info.name, ir.lhs, ir.rhs),
        IRInfoType::REG_LABEL => format!("{} r{}, .L{}", info.name, ir.lhs, ir.rhs),
        IRInfoType::CALL => {
            let mut s = String::new();
            s.push_str(&format!("r{} = {}(", ir.lhs, ir.name));
            for i in ir.args.iter() {
                s.push_str(&format!(", r{}", i));
            }
            s.push_str(")");
            s
        }
        IRInfoType::NOARG => format!("{}", info.name),
        _ => {
            panic!("unknown ir");
        }
    };
}

fn dump_ir(irv: Vec<IR>) {
    for i in 0..irv.len() {
        let fun = &irv[i];
        eprintln!("{}():", fun.name);
        for j in 0..fun.ir.len() {
            eprintln!("{}", tostr(fun.ir[j].clone()));
        }
    }
}

fn alloc_ir() -> IR {
    return IR {
        op: IRType::NOP,
        lhs: 0,
        rhs: 0,
        
        name: String::new(),
        nargs: 0,
        args: [0; 6],

        ir: Vec::new(),
    };
}
fn add(op: IRType, lhs: i32, rhs: i32) -> usize {
    let mut ir = alloc_ir();
    ir.op = op;
    ir.lhs = lhs;
    ir.rhs = rhs;
        
    match CODE.lock() {
        Ok(mut code) => {
            (*code).push(ir);
            return code.len() - 1;
        }
        Err(_) => {
            panic!();
        }
    }
}

fn gen_lval(node: Node) -> i32 {
    if node.ty != NodeType::IDENT {
        panic!("not an lvaue");
    }

    let mut vars = VARS.lock().unwrap();
    if None == vars.get(&node.name) {
        let mut bpoff = BPOFF.lock().unwrap();
        (*vars).insert(node.name.clone(), *bpoff);
        *bpoff += 8;
    }

    let mut regno = REGNO.lock().unwrap();
    let r = *regno;
    *regno += 1;

    let off = *vars.get(&node.name).unwrap();
    let basereg = BASEREG.lock().unwrap();

    add(IRType::MOV, r, *basereg);
    add(IRType::ADD_IMM, r, off);
    return r;
}

fn gen_expr(node: Node) -> i32 {
    if node.ty == NodeType::NUM {
        let mut regno = REGNO.lock().unwrap();

        let r = *regno;
        *regno += 1;
        add(IRType::IMM, r, node.val);
        return r;
    }

    if node.ty == NodeType::IDENT {
        let r = gen_lval(node);
        add(IRType::LOAD, r, r);
        return r;
    }

    if node.ty == NodeType::CALL {
        let mut args = Vec::new();
        for a in node.args.iter() {
            args.push(gen_expr(a.clone()));
        }

        let r = regno();
        inc_regno();

        let ir_idx = add(IRType::CALL, r, -1);

        let (nargs, args) = match CODE.lock() {
            Ok(mut code) => {
                code[ir_idx].name = node.name.clone();
                code[ir_idx].nargs = node.args.len();

                for i in 0..code[ir_idx].nargs {
                    code[ir_idx].args[i] = args[i];
                }

                (code[ir_idx].nargs, code[ir_idx].args)
            }
            Err(_) => {
                panic!();
            }
        };

        for i in 0..nargs {
            add(IRType::KILL, args[i], -1);
        }
        return r;
    }

    if node.ty == NodeType::EQ {
        let rhs = gen_expr(*node.rhs.unwrap());
        let lhs = gen_lval(*node.lhs.unwrap());
        add(IRType::STORE, lhs, rhs);
        add(IRType::KILL, rhs, -1);
        return lhs;
    }

    assert!(
        node.ty == NodeType::ADD
            || node.ty == NodeType::SUB
            || node.ty == NodeType::MUL
            || node.ty == NodeType::DIV,
        format!("not op: {:?}", node)
    );

    let lhs = gen_expr(*node.lhs.unwrap());
    let rhs = gen_expr(*node.rhs.unwrap());

    add(to_ir_type(&node.ty), lhs, rhs);
    add(IRType::KILL, rhs, -1);
    return lhs;
}

fn gen_stmt(node: Node) {
    match node.ty {
        NodeType::IF => {
            let r = gen_expr(*node.cond.unwrap());
            let mut label = LABEL.lock().unwrap();
            let x = *label;
            *label += 1;

            add(IRType::UNLESS, r, x);
            add(IRType::KILL, r, -1);

            gen_stmt(*node.then.unwrap());

            if node.els.is_none() {
                add(IRType::LABEL, x, -1);
                return;
            }

            let y = *label;
            *label += 1;
            add(IRType::JMP, y, -1);
            add(IRType::LABEL, x, -1);
            gen_stmt(*node.els.unwrap());
            add(IRType::LABEL, y, -1);
        }
        NodeType::RETURN => {
            let r = gen_expr(*node.expr.unwrap());
            add(IRType::RETURN, r, -1);
            add(IRType::KILL, r, -1);
        }
        NodeType::EXPR_STMT => {
            let r = gen_expr(*node.expr.unwrap());
            add(IRType::KILL, r, -1);
        }
        NodeType::COMP_STMT => {
            for n in node.stmts {
                gen_stmt(n);
            }
        }
        _ => {
            panic!(format!("unknown node: {:?}", node.ty));
        }
    }
}

pub fn gen_ir(nodes: Vec<Node>) -> Vec<IR> {
    let mut v = Vec::new();

    for i in 0..nodes.len() {
        let node = nodes[i].clone();
        assert!(node.ty == NodeType::FUNC, "");

        *CODE.lock().unwrap() = Vec::new();

        let alloca_idx = add(IRType::ALLOCA, basereg(), -1);
        gen_stmt(*node.body.unwrap());
        match CODE.lock() {
            Ok(mut code) => {
                code[alloca_idx].rhs = bpoff();
            }
            Err(_) => {
                panic!();
            }
        }
        add(IRType::KILL, basereg(), -1);

        let mut fun = alloc_ir();
        fun.name = node.name.clone();
        fun.ir = CODE.lock().unwrap().clone();

        v.push(fun);
    }

    return v;
}
