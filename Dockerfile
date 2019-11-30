FROM rust:1.39
ADD . /app
WORKDIR /app/iron-rest-stephane-homepage
RUN cargo build --release
EXPOSE 3000
CMD ["cargo", "run", "--release"]
