# 🚀 Aprendizado Rust + DevSecOps

Este repositório contém meus primeiros passos na jornada com **Rust** dentro do roadmap de **6 meses focado em Rust + DevSecOps**.  
Aqui estão os dois projetos iniciais:

---

## 📂 Projetos

### 1. `hello-rust`

Meu primeiro projeto em Rust criado com `cargo`.  
Objetivos:

- Instalar e configurar o ambiente Rust.
- Aprender a estrutura básica de um projeto (`Cargo.toml`, `src/main.rs`).
- Rodar o clássico **Hello World**.
- Alterar a mensagem para personalizar a saída no console.

📌 Como rodar:

```bash
    cd hello-rust
    cargo run
```

### 2. `rustlings`

Conjunto de exercícios oficiais da comunidade Rust para treinar fundamentos da linguagem.
Objetivos:

Fixar conceitos básicos de Rust (variáveis, funções, controle de fluxo).

Praticar ownership, borrowing e lifetimes.

Avançar para tópicos mais avançados como traits, enums e coleções.

📌 Como rodar:

```bash
cd rustlings
cargo run --bin rustlings -- watch
```

🛠 Ferramentas Utilizadas

Rust - (via rustup)
Cargo – gerenciador de pacotes e build
Rustlings – exercícios interativos

📅 Roadmap

Este repositório faz parte do meu roadmap de 6 meses focado em Rust + DevSecOps,
com etapas semanais organizadas em Obsidian/Kanban.

Mês 1 – Semana 1: Instalação, Hello World e início do Rustlings
Mês 1 – Semana 2: Ownership, References, Lifetimes
Mês 2 em diante: APIs seguras com Axum + SQLx, Docker, CI/CD, etc.

📜 Licença

Este repositório é de uso pessoal para fins de estudo.
Inspirado pelo material da comunidade Rust-Lang.

---

# 📄 `.gitignore`

Para projetos Rust, o padrão é ignorar **arquivos de build** e **cache** do cargo:

```yaml
# Arquivos de build
/target/

# Logs e artefatos temporários
*.log

# Arquivos de backup do editor
*~

# Diretórios de dependências locais (se existirem)
**/.cargo/

# VSCode / JetBrains
.vscode/
.idea/

# Sistema
.DS_Store
Thumbs.db
```

📌 Sugestão de organização no GitHub:

```bash
/rust-learning/
 ├── hello-rust/
 ├── rustlings/
 ├── README.md
 └── .gitignore
```

## Contato

<a href="https://github.com/SilvaneiMartins">
    <img
        style="border-radius:50%"
        src="https://github.com/SilvaneiMartins.png"
        width="100px;"
        alt="Silvanei Martins"
    />
    <br />
    <sub>
        <b>Silvanei de Almeida Martins</b>
    </sub>
</a>
     <a href="https://github.com/SilvaneiMartins" title="Silvanei martins" >
 </a>
<br />
🚀 Feito com ❤️ por Silvanei Martins
