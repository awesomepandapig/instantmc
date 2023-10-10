# Use the official Golang base image
FROM golang AS build

# Clone the CoreDNS repository
RUN git clone https://github.com/coredns/coredns /coredns

# Navigate into the coredns directory
WORKDIR /coredns

# Add redis to the plugin.cfg
RUN echo "redis:github.com/awesomepandapig/coredns-redis" >> plugin.cfg

# Install the redis plugin
RUN go get github.com/awesomepandapig/coredns-redis@c05b6fd

# Generate and build the code
RUN go generate && go build

# Set the entry point as the command to run CoreDNS
ENTRYPOINT ["./coredns"]