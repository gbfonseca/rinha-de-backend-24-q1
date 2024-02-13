# Use a imagem oficial do Rust como base
FROM rust:latest as builder

# Defina o diretório de trabalho dentro do contêiner
WORKDIR /usr/src/app

# Copie os arquivos do projeto para o diretório de trabalho
COPY . .

RUN rustup target add x86_64-unknown-linux-gnu

# Compile o projeto
RUN cargo build --target x86_64-unknown-linux-gnu --release

# Copie o binário de release do diretório de construção para a nova imagem
# COPY  ./target/x86_64-unknown-linux-gnu/release/rinha-de-backend-24-q1 /usr/local/bin/rinha-de-backend-24-q1

# Exponha a porta utilizada pela aplicação Actix-Web
EXPOSE 8080

# Comando para iniciar a aplicação quando o contêiner for iniciado
CMD ["./target/x86_64-unknown-linux-gnu/release/rinha-de-backend-24-q1"]
