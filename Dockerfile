FROM rust:1.28-slim-stretch

ENV DEBIAN_FRONTEND noninteractive
ENV HOME /home/rusty
ENV USER rusty

RUN apt-get update -y \
      && apt-get --no-install-recommends install -y pkg-config apt-utils \
      build-essential sudo libffi-dev libssl-dev git-core \
      && rm -rf /var/lib/apt/lists/*

# Create a local user that will be able to run commands
RUN useradd -m -s /bin/bash rusty

# Create the root directory in the home of the user
RUN mkdir -p $HOME/src/

# Creates a dummy project used to grab dependencies
WORKDIR $HOME/src/
RUN cargo new git-leaf --bin

# Switch to the newly created project directory
WORKDIR $HOME/src/git-leaf/

# Copies over *only* your manifests
COPY Cargo* $HOME/src/git-leaf/

# Builds your dependencies and removes the
# fake source code from the dummy project
RUN cargo build --release
RUN rm src/*.rs

# Install Rust fmt
RUN rustup component add rustfmt-preview

# Copies only your actual source code to
# avoid invalidating the cache at all
COPY src $HOME/src/git-leaf/src

# Give the home directory the rights to the user
RUN chown -R rusty:rusty $HOME

USER rusty

# Builds again, this time it'll just be
# your actual source files being built
RUN cargo build --release

CMD ["cargo run"]
