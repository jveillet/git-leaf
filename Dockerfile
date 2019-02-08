FROM rust:1.32-stretch

ENV DEBIAN_FRONTEND noninteractive
ENV HOME /home/rusty
ENV SRC_DIR /home/rusty/src
ENV APP_DIR /home/rusty/src/git-leaf
ENV USER rusty

RUN apt-get update -y \
      && apt-get --no-install-recommends install -y pkg-config apt-utils \
      build-essential sudo libffi-dev libssl-dev git-core \
      && rm -rf /var/lib/apt/lists/*

# Create a local user that will be able to run commands
RUN useradd -m -s /bin/bash $USER

# Create the root directory in the home of the user
RUN mkdir -p $SRC_DIR

# Creates a dummy project used to grab dependencies
WORKDIR $SRC_DIR
RUN cargo new git-leaf --bin

# Switch to the newly created project directory
WORKDIR $APP_DIR

# Copies over *only* your manifests
COPY Cargo* $APP_DIR/

# Builds your dependencies and removes the
# fake source code from the dummy project
RUN cargo build --release
RUN rm src/*.rs

# Install Rust fmt and Clippy
RUN rustup component add rustfmt-preview
RUN rustup component add clippy-preview

# Copies only your actual source code to
# avoid invalidating the cache at all
COPY src $APP_DIR/src

# Give the home directory the rights to the user
RUN chown -R rusty:rusty $HOME

# For some reason, the cargo cache and indexes do not seems to have the user rights
RUN chown -R rusty:rusty /usr/local/cargo

USER rusty

# Builds again, this time it'll just be
# your actual source files being built
RUN cargo build --release

CMD ["cargo", "run"]
