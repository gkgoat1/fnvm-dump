use std::{collections::HashMap, fmt::format};

use lambda_calculus::Term;

fn emitLambda(arg: String,body: String) -> String{
    return format!("λ{}.{}",arg,body);
}
fn emitCond(val: String,tru: String,flase: String) -> String{
    return format!("{} {} {}",val,tru,flase)
}
fn emitBool(b: bool) -> String{
    return if b{
        emitLambda("a".to_owned(),emitLambda("b".to_owned(),"a".to_owned()))
    }else{
        emitLambda("a".to_owned(),emitLambda("b".to_owned(),"b".to_owned()))
    }
}
fn closeOver(arg: String,body: String,traget: String) -> String{
    return "(".to_owned() + emitLambda(arg,body) + " " + traget + ")"
}
fn emitNot(val: String) -> String{
    return emitCond(val,emitBool(false),emitBool(true))
}
fn emitOr(a: String,b: String) -> String{
    return emitCond(a,b,emitBool(true))
}
fn emitAnd(a: String,b: String) -> String{
    return emitNot(emitOr(emitNot(a),emitNot(b)));
}
fn emitY() -> String{
    return "λf. (λx. x x) (λx. f (λy. (x x) y))".to_owned()
}
fn emitRecurse(f: String) -> String{
    return format!("(({}) {})",emitY(),f)
}
fn emitWhile(cond: String,then: String) -> String{
    let r = crate::rid();
    return emitRecurse(emitLambda(r,emitCond(cond,r,then)))
}
fn emitUntil(cond: String,eles: String) -> String{
    return emitWhile(emitNot(cond), eles)
}
fn emitNum(x: usize) -> String{
    return if x == 0{
        emitBool(false)
    } else{
        emitSucc(emitNum(x))
    }
}
fn emitSucc(x: String) -> String{
    return format!("({} {})",emitLambda("n".to_string(), emitLambda("f".to_owned(), emitLambda("x".to_owned(),"f (n f x)".to_owned()))),x)
}
fn emitPred(x: String) -> String{
    return format!("(λnfx.n (λgh.h (g f)) (λu.x) (λu.u) {})",x)
}
fn emitAdd(a: String,b: String) -> String{
    return format!("{} {}",b,emitSucc(a))
}
fn emitSub(a: String,b: String) -> String{
    return format!("{} {}",b,emitPred(a))
}
fn emitMul(a: String,b: String) -> String{
    return format!("((λmnf.m (n f)) {} {})",a,b);
}
fn emitIsZero(x: String) -> String{
    return format!("({} {} {})",x,emitLambda("_a".to_owned(), emitBool(false)),emitBool(true))
}
fn emitLeq(a: String,b: String) -> String{
    return emitIsZero(emitSub(b,a));
}
fn emitEq(a: String,b: String) -> String{
    return emitAnd(emitLeq(a,b), emitLeq(b,a))
}
fn emitDiv(a: String,b: String) -> String{
    return format!("(({} (λzab.LT a b (λx.{}) (λx.{}) I)) {} {})",emitY(),emitNum(0),emitSucc(format!("(z {} b)",emitSub("a".to_owned(), "b".to_owned()))),a,b);
}
fn emitFNumsRec(fns: Vec<String>,idx: usize) -> String{
    if fns.len() == 0{
        return emitNum(0);
    }
    let (a,b) = fns.split_at(1);
    let (a,b) = (a.to_owned(),b.to_owned());
    return emitCond(emitEq("targetFn".to_string(), emitNum(idx)),closeOver("tail".to_owned(),emitLambda("state".to_owned(),a[0]), format!("call {}",emitNum(idx))),emitFNumsRec(b, idx + 1))
}
fn emitFNums(fns: Vec<String>) -> String{
    return emitRecurse(emitLambda("call".to_owned(),emitFNumsRec(fns,0)));
}
fn emitFNames(fns: HashMap<String,String>) -> String{
    let mut fv= Vec::new();
    let mut fm: HashMap<String,usize> = HashMap::new();
    for (a,b) in fns.into_iter(){
        fv.push(b);
        fm[a] = fv.len() - 1;
    }
    for (a,b) in fm.into_iter(){
        let s = format!("(call {} state)",b);
        let ap = format!("fp_{}",&a);
        let sp = format!("{}",b)
        let a = format!("fn_{}",a);
        fv = fv.into_iter().map(|x|x.replace(&a, &s).replace(&ap, &sp)).collect()
    }
    return emitFNums(fv);
}
pub static NORMAL: u8 = 0;
pub static FORKABLE: u8 = 1;
pub static FOREVER: u8 = 2;

pub static NO: u8 = 3;
fn emitRuntime(m: &mut HashMap<String,String>,mode: u8){
    m["__rt_call_fp"] = emitLambda("x".to_owned(), "call x state".to_owned());
    m["__rt_alloc_byte"] = emitLambda("target".to_owned(),format!("call target {}",emitCons(emitNum(0),"state".to_owned())));
    m["__rt_alloc_bytes"] = emitLambda("val".to_owned(), emitLambda("target".to_owned(),emitCond(emitEq("val".to_string(),emitNum(1)),"fn___rt_alloc_byte target".to_owned() , emitRawFCall("fn___rt_alloc_byte fp___rt_alloc_bytes".to_owned(), emitSub("val".to_owned(),emitNum(1))))));
    m["__rt_loop_fp"] = emitLambda("x".to_owned(), emitWhile("call x state".to_owned(),emitBool(false)));

    if mode >= NO{panic!("no")}
    m["__rt_fork"] = if mode >= FORKABLE{emitLambda("a".to_owned(), emitLambda("b".to_owned(), emitCond(emitFork(),"a".to_owned(),"b".to_owned())))}else{emitBool(false)}
}
fn emitEmptyList() -> String{
    emitBool(true)
}
fn emitIsEmptyList(x: String) -> String{
    return format!("{} {} {}",x,emitBool(true),emitLambda("_".to_owned(), emitBool(false)))
}
fn emitCons(a: String,b: String) -> String{
    return format!("(λaxnc.c a ((λl.l) x n c)) {} {}",a,b);
}
fn emitFirst(a: String) -> String{
    return format!("(λaxnc.c a ((λl.l) x n c)) {}",a);
}
fn emitBasePair(a: String,b: String) -> String{
    return format!("((λxyz.z x y) {} {})",a,b);
}
fn emitFirstOfPair(p: String) -> String{
    return format!("({} {})",p,emitBool(true));
}
fn emitSecondOfPair(p: String) -> String{
    return format!("({} {})",p,emitBool(false));
}
fn emitRawFCall(a: String,b: String) -> String{
    return format!("({} {})",a,b)
}
fn emitAllButFirst(l: String) -> String{
    return emitFirstOfPair(emitRawFCall(emitRawFCall(l,emitBasePair(emitBool(false),
    emitBool(false))), emitLambda("ap".to_owned(), 
    emitBasePair(emitSecondOfPair("p".to_owned()),emitCons("a".to_owned(),emitSecondOfPair("p".to_owned()))))));
}
fn emitPatch(x: String,target: String) -> String{
    return emitCons(emitFirst(target),emitRawFCall(x, emitAllButFirst(target)))
}
fn emitPatchFn(x: String) -> String{
    let id = crate::rid();
    return emitLambda(id, emitPatch(x,id));
}
fn emitOuter(num: usize) -> String{
    return format!("#-{}#",num);
}
fn emitIO(val: String,i: usize) -> String{
    return emitRawFCall(emitRawFCall(emitOuter(1),val), emitNum(i));
}
fn emitList(l: Vec<String>) -> String{
    if l.len() == 0{
        return emitEmptyList();
    }
    return emitCons(l[0],emitList(l[1..].to_owned()));
}
fn emitNumList(l: Vec<usize>) -> String{
    return emitList(l.into_iter().map(|x|format!("{}",x)).collect());
}
fn emitBytes(l: Vec<u8>) -> String{
    return emitNumList(l.into_iter().map(Into::into).collect());
}
fn emitString(s: String) -> String{
    return emitBytes(s.into_bytes())
}
fn emitFork() -> String{
    return emitIO(emitBool(false), 0);
}
fn emitPrint(x: String) -> String{
    return emitIO(emitBasePair(x,emitBool(false)), 1);
}
fn emitInput() -> String{
    return emitIO(emitBasePair(emitBool(false),emitBool(true)), 1);
}
pub fn emitStrAsTerm(x: String) -> Term{
    return emitString(x).to_string();
}