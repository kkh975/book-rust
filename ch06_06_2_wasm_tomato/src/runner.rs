use std::collections::HashMap;
use crate::parser::tomato;
use crate::node::Node;

// 프로그램 전체에서 이용할 컨텍스트 정의 --- (*1)
struct Context {
    vars: HashMap<String, i64>, // 변수와 값 저장
    output: String, // 출력 결과(추가)
}

// 구문 트리를 하나씩 실행 --- (*2)
fn run_node(ctx: &mut Context, node: Node) -> i64 {
    // 어떤 타입의 노드인지 판단
    match node {
        Node::Number(v) => v, // 숫자 값을 반환
        Node::Calc(op, l, r) => { // 계산식 --- (*3)
            calc_op(op, run_node(ctx, *l), run_node(ctx, *r))
        },
        Node::GetVar(name) => { // 숫자 값 얻기 --- (*4)
            match ctx.vars.get(&name) {
                Some(v) => *v,
                None => 0,
            }
        },
        Node::SetVar(name, node) => { // 변수 대입
            let val = run_node(ctx, *node);
            ctx.vars.insert(name, val);
            val
        },
        Node::If(cond, true_n, false_n) => { // if 문 --- (*5)
            let cond_v = run_node(ctx, *cond);
            if cond_v > 0 {
                run_nodes(ctx, &*true_n)
            } else {
                run_nodes(ctx, &*false_n)
            }
        },
        Node::For(name, start, end, body) => { // for 문 --- (*6)
            let mut r = 0;
            let nodes = *body;
            for i in start..=end {
                ctx.vars.insert(name.clone(), i);
                r = run_nodes(ctx, &nodes);
            }
            r
        },
        Node::PrintStr(v) => { 
            ctx.output += &format!("{}\n", v);
            0
        }, // --- (*7)
        Node::Print(node) => { // print文 --- (*8)
            let v = run_node(ctx, *node);
            ctx.output += &format!("{}\n", v);
            v
        },
        _ => 0,
    }
}

// 연산자를 바탕으로 계산 --- (*9)
fn calc_op(op: char, val_l:i64, val_r:i64) -> i64 {
    match op {
        '+' => val_l + val_r,
        '-' => val_l - val_r,
        '*' => val_l * val_r,
        '/' => val_l / val_r,
        '%' => val_l % val_r,
        '=' => if val_l == val_r {1} else {0},
        '!' => if val_l != val_r {1} else {0},
        '>' => if val_l > val_r {1} else {0},
        'g' => if val_l >= val_r {1} else {0},
        '<' => if val_l < val_r {1} else {0},
        'l' => if val_l <= val_r {1} else {0},
        _ => 0,
    }
}

// 반복해서 Node를 실행 --- (*10)
fn run_nodes(ctx: &mut Context, nodes: &Vec<Node>) -> i64 {
    let mut result = 0;
    nodes.iter().for_each(|node| {
        result = run_node(ctx, node.clone())});
    result
}

// 프로그램을 실행할 함수 --- (*11)
pub fn run(src: &str) -> String {
    // 구문분석
    let nodes = match tomato::parse(src) {
        Ok(res) => res,
        Err(e) => return e.to_string(),
    };

    // 컨텍스트 생성
    let mut ctx = Context{
        vars:HashMap::new(),
        output:String::new()
    };

    // 실행
    let r = run_nodes(&mut ctx, &nodes);

    // 결과 반환
    // print로 결과를 출력
    if ctx.output == "" {
        return format!("{}", r);
    } else {
        return ctx.output.clone();
    }
}