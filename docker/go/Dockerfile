FROM golang:1.20

# Work directory
WORKDIR /payment-frame

# Installing dependencies
COPY go.mod go.sum ./
RUN go mod download && go mod verify

RUN go install github.com/cosmtrek/air@latest

# Copy the source from the current directory to the workspace
COPY . .

# Command to run the executable
CMD ["air", "-c", ".air.toml"]