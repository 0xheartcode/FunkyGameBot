# Use a Rust base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the local Cargo.toml and Cargo.lock files into the container
COPY Cargo.toml Cargo.lock ./
COPY ./.env ./
ENV $(cat .env | xargs)
# Extract the package name from Cargo.toml using sed
RUN PACKAGE_NAME=$(sed -n 's/^name = "\([^"]*\)"$/\1/p' Cargo.toml | tr '[:upper:]' '[:lower:]')

# Build the application (cache dependencies separately)
RUN mkdir src
COPY src ./src
RUN cargo build --release

# Create a minimal runtime image
FROM rust:latest 

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the built executable from the builder image using the extracted package name
COPY --from=builder /usr/src/app/target/release/third_bot .

# Set the command to run on container start using the extracted package name
CMD ["./third_bot"]

