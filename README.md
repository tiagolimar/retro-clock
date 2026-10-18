# Relógio Retro

Um relógio digital retro em Rust: fundo preto, letras verdes, hora local em formato de sete segmentos, data `dd/mm/yy` e contagem regressiva do ciclo diário das 08:00 às 17:00.

## Comportamento

- A hora é atualizada usando o horário local do sistema.
- Antes das 08:00, a contagem mostra o ciclo completo de 9 horas e informa que está aguardando o início.
- Entre 08:00 e 17:00, a contagem regressiva vai até 17:00.
- A partir das 17:00, a contagem fica em `00:00:00` até o próximo ciclo das 08:00.
- A janela pode ser movida mantendo o botão esquerdo pressionado e arrastando pelo conteúdo.
- A janela é redimensionável pelas quinas e pode ser maximizada para ocupar a tela. O conteúdo é esticado para preencher a área disponível.
- O título da janela fica oculto; use o menu de contexto para fechar o aplicativo.
- O menu de contexto oferece as opções `Tela inteira`, `Restaurar` e `FECHAR`.
- Para fechar pelo app, clique com o botão direito dentro da janela e depois clique em `FECHAR`.

## Ubuntu

Instale o Rust pelo método recomendado para sua distribuição. Para o backend gráfico X11 do `minifb`, o Ubuntu/Debian normalmente precisa destas dependências:

```bash
sudo apt update
sudo apt install -y build-essential cmake pkg-config libx11-dev libxkbcommon-dev libgl1-mesa-dev libxrandr-dev
```

Se o Ubuntu estiver usando Wayland e o backend padrão não abrir, instale também:

```bash
sudo apt install -y libwayland-dev libxkbcommon-dev wayland-protocols
```

Execute:

```bash
cargo run --release
```

Para gerar o executável:

```bash
cargo build --release
```

O binário ficará em `target/release/relogio-retro`.

## Windows

Com Rust instalado, abra PowerShell nesta pasta e execute:

```powershell
cargo run --release
```

O executável ficará em `target\release\relogio-retro.exe`.

Use o botão de maximizar da janela para ocupar a tela inteira. O relógio mantém o layout centralizado e aumenta junto com a janela.

O programa usa `minifb` 0.29 para a janela/pixels e `chrono` 0.4 para ler o relógio local. O código não depende de uma fonte instalada nem de componentes específicos do Windows.

## Organização do código

O projeto separa domínio, interação e apresentação:

```text
src/
├── main.rs                 # ciclo da janela e integração com minifb
├── app.rs                  # coordenação de um ciclo da aplicação
├── config.rs               # dimensões e configuração compartilhada
├── input.rs                # modelo de entrada do mouse
├── ui.rs                   # estado e eventos do menu de contexto
├── model/
│   ├── clock.rs            # provedor de horário local e snapshot do relógio
│   └── countdown.rs        # regra 08:00–17:00 e testes unitários
└── render/
    ├── canvas.rs           # operações básicas de desenho
    ├── colors.rs           # paleta visual
    ├── frame.rs            # moldura do painel
    ├── context_menu.rs     # desenho do menu de fechar
    ├── font/               # fonte bitmap para textos
    └── segment/            # dígitos, separadores e displays
```

A regra da contagem regressiva pode ser testada sem abrir uma janela:

```bash
cargo test
```

