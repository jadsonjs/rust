

pub struct Verbs {
    pub verb1: &'static str,
    pub verb2: &'static str,
    pub verb3: &'static str,
}

impl Verbs {
    pub fn new() -> Self {
        Verbs {
            verb1: "run",
            verb2: "eat",
            verb3: "sleep",
        }
    }
}


// O pub fn nos arquivos english.rs e portuguese.rs permite que as funções sejam chamadas fora do módulo.
pub fn say_hello() {
    println!("Hello!");
}