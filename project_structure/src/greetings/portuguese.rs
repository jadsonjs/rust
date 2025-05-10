

pub struct Verbos {
    pub verbo1: &'static str,
    pub verbo2: &'static str,
    pub verbo3: &'static str,
}

impl Verbos {
    pub fn new() -> Self {
        Verbos {
            verbo1: "correr",
            verbo2: "comer",
            verbo3: "dormir",
        }
    }
}

// O pub fn nos arquivos english.rs e portuguese.rs permite que as funções sejam chamadas fora do módulo.
pub fn say_hello() {
    println!("Olá!");
}