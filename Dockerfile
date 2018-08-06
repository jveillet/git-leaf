FROM rust:1.28-stretch

ENV DEBIAN_FRONTEND noninteractive
ENV HOME /home/rusty
ENV USER rusty

RUN apt-get update -y \
      && apt-get --no-install-recommends install -y pkg-config apt-utils \
      build-essential sudo libffi-dev libssl-dev \
      && rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash rusty
RUN mkdir -p $HOME/src/git-leaf

RUN rustup component add rustfmt-preview

WORKDIR $HOME/src/git-leaf

COPY . .

RUN chown -R rusty:rusty $HOME

RUN rm -rf target/

USER rusty

RUN cargo build --release

CMD ["cargo run"]
