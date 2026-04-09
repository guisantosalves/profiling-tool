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
