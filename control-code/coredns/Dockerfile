# Use the official Golang base image for the first build stage
FROM golang:alpine AS builder

# Install necessary packages for compiling Go with libc
RUN apk --no-cache add git gcc musl-dev

# Clone the CoreDNS repository
RUN git clone --depth 1 https://github.com/coredns/coredns /coredns

# Navigate into the coredns directory
WORKDIR /coredns

# Add redis to the plugin.cfg
RUN echo "redis:github.com/awesomepandapig/coredns-redis" >> plugin.cfg

# Install the redis plugin
RUN go get github.com/awesomepandapig/coredns-redis@c05b6fd

# Generate and build the code
RUN go generate && go build -o /coredns/coredns

# Use a minimal base image for the second build stage
FROM alpine:latest

# Set the working directory
WORKDIR /coredns

# Copy the CoreDNS binary from the first build stage
COPY --from=builder /coredns/coredns /coredns

# Set the entry point as the command to run CoreDNS
ENTRYPOINT ["./coredns"]