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

## processos x threads

O SO tem muito mais threads do que núcleos. Tipo: sua máquina tem 16 slots lógicos, mas no Task Manager deve ter
milhares de threads rodando (soma tudo dos processos).

Como isso funciona? O SO dá a cada thread um pequeno intervalo de tempo (tipo 10-20 milissegundos, chamado time
slice). Quando acaba:

1. SO pausa a thread (salva o estado dela: registradores, program counter, etc)
2. SO escolhe outra thread da fila
3. SO restaura o estado dela
4. Thread roda por mais um time slice

Isso acontece milhares de vezes por segundo, tão rápido que parece que tudo roda ao mesmo tempo.

---

Analogia: o processo é uma casa, as threads são as pessoas dentro dela. Todas compartilham os móveis (memória), mas
cada uma faz suas tarefas.

## passagem por valor x referencia

- Passagem por valor → valor (sem &) → copia/move o dado
- Passagem por referência → &valor → empresta (passa o endereço)

Em Rust, passar valor sem & pode fazer duas coisas dependendo do tipo:

- Tipos Copy (inteiros, floats, bool): copia o valor
- Tipos não-Copy (String, Vec, structs): move o ownership (a variável original deixa de existir)

  let s = String::from("hello");
  fn take(s: String) { ... }
  take(s); // s é MOVIDO pra dentro da função
  // println!("{}", s); // ❌ erro, s não existe mais

  fn borrow(s: &String) { ... }
  let s2 = String::from("world");
  borrow(&s2); // s2 é emprestado
  println!("{}", s2); // ✅ ok, s2 ainda existe
