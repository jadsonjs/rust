
// carrega o módulo greetings (cujo conteúdo está no mod.rs).
mod greetings;

use greetings::english::{say_hello as say_hello_en, Verbs};
use greetings::portuguese::{say_hello as say_hello_pt, Verbos};

fn main() {

    // Caminho absoluto: Começa no topo do crate (como greetings::english::Verbs).

    greetings::english::say_hello();
    greetings::portuguese::say_hello();


    let eng_verbs = greetings::english::Verbs::new();
    println!("English verbs: {}, {}, {}", eng_verbs.verb1, eng_verbs.verb2, eng_verbs.verb3);

    let pt_verbos = greetings::portuguese::Verbos::new();
    println!("Verbos em português: {}, {}, {}", pt_verbos.verbo1, pt_verbos.verbo2, pt_verbos.verbo3);

    // use importa um item e pode renomear com as (útil para evitar conflitos, como say_hello).
    // Com use, você pode escrever menos e deixar o código mais legível.

    say_hello_en();
    say_hello_pt();

    let eng_verbs = Verbs::new();
    println!("English verbs: {}, {}, {}", eng_verbs.verb1, eng_verbs.verb2, eng_verbs.verb3);

    let pt_verbos = Verbos::new();
    println!("Verbos em português: {}, {}, {}", pt_verbos.verbo1, pt_verbos.verbo2, pt_verbos.verbo3);
}
