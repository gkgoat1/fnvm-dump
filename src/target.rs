use lambda_calculus::{app, Term};
use Term::{Abs, App, Var};

pub fn target_blc(t: lambda_calculus::Term) -> Vec<u8> {
    return blc::to_bits(&t);
}
pub fn target_haskell(t: Term) -> String {
    let s = t.to_string();
    return s.replace("λ", "\\").replace(".", "->");
}
pub fn target_opt(t: Term) -> Term {
    return lambda_calculus::beta(t, lambda_calculus::NOR, 0);
}
pub fn target_obf(t: Term, noopt: Term) -> Term {
    return match t {
        Var(x) => app!(
            lambda_calculus::data::boolean::if_else(noopt),
            tobf_dead(noopt),
            Var(x)
        ),
        Abs(x) => {
            app!(lambda_calculus::data::boolean::if_else(noopt), tobf_dead(noopt), Abs(target_obf(x,noopt)),
    App(a,b) => app!(lambda_calculus::data::boolean::if_else(),noopt, toobf_dead(noopt),a, b))
        }
    };
}
fn tobf_dead(noopt: Term) -> Term {
    return if rand::random() {
        if rand::random() {
            Abs(Box::new(tobf_dead(noopt)))
        } else {
            Var(rand::random())
        }
    } else if rand::random() {
        if rand::random() {
            if rand::random() {
                crate::emit::emitStrAsTerm(rand::random())
            } else {
                crate::emit::emitStrAsTerm(target_blc(tobf_dead(noopt)).as_hex())
            }
        } else {
            app!(
                lambda_calculus::data::boolean::if_else(),
                noopt,
                tobf_dead(noopt),
                tobf_dead(noopt)
            )
        }
    } else {
        App(Box::new(tobf_dead(noopt)), Box::new(tobf_dead(noopt)))
    };
}
