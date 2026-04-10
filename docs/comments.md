## implementação

1. Collector básico — fazer collector.rs retornar uma struct com CPU, RAM, disco, uptime usando sysinfo
2. Rota JSON — criar rota GET /stats no server que chama o collector e retorna JSON
3. Adicionar temperatura — incluir dados de temperatura na struct/coleta
4. WebSocket — criar rota /ws que envia as métricas a cada 1 segundo via WebSocket
5. HTML dashboard — servir uma página HTML estática pelo Axum com layout básico
6. Frontend real-time — JS no HTML que conecta no WebSocket e atualiza os números
7. Gráficos — adicionar charts (CPU/RAM ao longo do tempo) com uma lib JS leve
8. CORS + polish — configurar CORS, tratamento de erros, build de release
9. Deploy — instruções para rodar como binário standalone em qualquer máquina

## Compilação

Compilação é transformar código legível (alto nível) por humanos em código que a máquina entende (baixo nível).

Compilada = Traduz tudo antes de rodar, gera um executável
Interpretada = Lê e executa linha por linha, em tempo real
Híbrida = Compila pra um intermediário, roda numa VM

<hr>

## Libs info

cargo build para baixar as deps

- axum — servidor HTTP + WebSocket
- tokio — runtime async (como goroutines)
- sysinfo — coleta info de CPU, RAM, disco
- serde — serialização JSON
- tower-http — middleware (CORS)

  String::new() // :: porque tá criando, não tem instância ainda
  my_string.len() // . porque my_string já existe

:: acessa módulos, tipos, funções associadas (static)
// . acessa métodos da instância (como Go)

## returns no rust

// 1. Retorna o valor, transfere o ownership
fn get_string() -> String {
let s = String::from("hello");
s // ownership passa pra quem chamou
}

// 2. Retorna &'static porque é literal hardcoded
fn get_string() -> &'static str {
"hello"
}

// 3. Recebe a string de fora e devolve referência dela
fn get_string(s: &str) -> &str {
s // lifetime atrelado ao parâmetro
}

// borrow
let s2;
{
let s = String::from("hello");
s2 = &s; // s2 emprestou de s
} // s foi desalocado aqui
println!("{}", s2); // ❌ não compila, s2 aponta pra memória inválida

// move
let s2 = s; // s deixa de existir, s2 é o novo dono

## protocolo

Protocolo é um conjunto de regras que define como dois lados se comunicam.

- HTTP — regras pra "peço algo, você responde" (browser pede página, servidor responde)
- WebSocket — regras pra "abrimos um canal e ambos mandam mensagens quando quiserem"
- TCP — regras pra garantir que os dados chegam na ordem certa
