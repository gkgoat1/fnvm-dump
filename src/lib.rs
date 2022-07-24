use rand::{distributions::Alphanumeric, Rng}; 
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod emit;
mod target;
pub fn rid() -> String{
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    return s;
}
#[cxx::bridge]
pub mod ffi{
    use ::lambda_calculus::Term;

    extern "Rust"{
        type Term;
        fn unvar_mut(&mut self: Term) -> Result<usize>;
        fn unabs_mut(&mut self: Term) -> Result<&mut Term>;
        fn unapp_mut(&mut self: Term) -> Result<(&mut Term,&mut Term)>;
        fn var_(x: usize) -> UniquePtr<Term>{
            return lambda_calculus::Var(x).into()
        }
        fn abs(t: UniquePtr<Term>) -> UniquePtr<Term>{
            return lambda_calculus::Abs(t.into()).into();
        }
        fn app(t: UniquePtr<Term>,u: UniquePtr<Term>) -> UniquePtr<Term>{
            return lambda_calculus::App(t.into(),u.into()).into();
        }
    }
    unsafe extern "C++"{

    }
}