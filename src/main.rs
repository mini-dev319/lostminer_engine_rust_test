// ==========================================
// CAPÍTULO 3 - ESTRUTURAS E ENUMS DE DADOS
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub enum TipoBloco {
    Terra,
    Pedra,
    Ouro,
}

struct Bloco {
    x: i32,
    y: i32,
    tamanho: i32,
    tipo: TipoBloco,
}

struct Picareta {
    durabilidade: u32,
    poder: u32,
}

// ==========================================
// CAPÍTULO 5 - LIFETIMES (TEMPOS DE VIDA)
// ==========================================
struct Jogador<'a> {
    nome: String,
    x: i32,
    y: i32,
    largura: i32,
    altura: i32,
    ferramenta: &'a Picareta, // Referência com lifetime que você acertou!
}

// ==========================================
// LÓGICA DO JOGO (COLISÃO E MINERAÇÃO)
// ==========================================

// Sua função de colisão do Desafio 1!
fn checar_colisao(player: &Jogador, bloco: &Bloco) -> bool {
    if player.x < bloco.x + bloco.tamanho &&       
       player.x + player.largura > bloco.x &&       
       player.y + player.altura > bloco.y &&        
       player.y < bloco.y + bloco.tamanho           
    {
        true 
    } else {
        false 
    }
}

// CAPÍTULO 4 - OPTION: O bloco existe nessa posição?
fn obter_bloco_no_mapa(x: i32, bloco_do_mundo: &Bloco) -> Option<TipoBloco> {
    // Se o jogador estiver na mesma posição X do bloco, nós encontramos ele!
    if x == bloco_do_mundo.x {
        Some(bloco_do_mundo.tipo.clone())
    } else {
        None // O "return None" que você fez no desafio!
    }
}

// CAPÍTULO 4 - RESULT: Sucesso ou Erro na mineração
fn minerar_bloco(bloco: TipoBloco, poder_picareta: u32) -> Result<String, String> {
    match bloco {
        TipoBloco::Ouro => {
            if poder_picareta >= 50 {
                Ok(String::from("✨ Sucesso! Você minerou uma Pepita de Ouro!"))
            } else {
                Err(String::from("❌ Sua picareta é fraca demais para minerar Ouro!"))
            }
        }
        TipoBloco::Pedra => Ok(String::from("🪨 Sucesso! Você conseguiu um bloco de Pedra.")),
        TipoBloco::Terra => Ok(String::from("🌱 Sucesso! Você conseguiu um bloco de Terra.")),
    }
}

// ==========================================
// FUNÇÃO PRINCIPAL (MAIN)
// ==========================================
fn main() {
    println!("--- INICIANDO SUA INDIE ENGINE EM RUST ---\n");

    // 1. Inicializando os objetos na memória
    let minha_picareta = Picareta { durabilidade: 100, poder: 10 }; // Picareta fraca de poder 10
    
    let player = Jogador {
        nome: String::from("Oliveira Moreira Gomes"),
        x: 10,
        y: 12,
        largura: 2,
        altura: 2,
        ferramenta: &minha_picareta,
    };

    let bloco_alvo = Bloco {
        x: 10,
        y: 10,
        tamanho: 2,
        tipo: TipoBloco::Ouro, // Um bloco de ouro valioso!
    };

    println!("Jogador: {}", player.nome);
    println!("Ferramenta: Picareta (Durabilidade: {})\n", player.ferramenta.durabilidade);

    // 2. Executando a física de colisão (Desafio 1)
    if checar_colisao(&player, &bloco_alvo) {
        println!("💥 COLISÃO DETECTADA! Você está encostando no bloco.");

        // 3. Buscando o bloco no mapa com Option (Desafio 3 - Parte 1)
        match obter_bloco_no_mapa(player.x, &bloco_alvo) {
            Some(bloco_encontrado) => {
                println!("Você está tentando minerar um bloco de: {:?}", bloco_encontrado);

                // 4. Tentando minerar com Result (Desafio 3 - Parte 2)
                // O match correto que analisa o sucesso ou o erro:
                match minerar_bloco(bloco_encontrado, player.ferramenta.poder) {
                    Ok(mensagem_sucesso) => println!("{}", mensagem_sucesso),
                    Err(mensagem_erro) => println!("{}", mensagem_erro),
                }
            }
            None => println!("💨 Nenhum bloco encontrado nessa coordenada."),
        }
    } else {
        println!("🍃 O jogador está voando livremente pelo mapa.");
    }
}
