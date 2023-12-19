FROM elixir:latest

WORKDIR /app

COPY . .

RUN mix local.hex --force && \
    mix local.rebar --force

