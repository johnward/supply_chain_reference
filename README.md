# Supply Chain Reference

This repository for a reference project for a Supply Chain solution in Rust, the aim of this repo is to provide a basic implementation of a supply chain solution in Microservices.

# Overview
This section provides a detailed overview of the folder structure

## rust_examples folder
This is a set of examples in Rust on topics such as buffer overflow

## monolith_service folder
This is an example of how to implement a set of supply chain services as a monolith application

## microservices folder
This is a set of services that show how to implement a supply chain solution as microservices

## json_client
This is a hyper client to test the supplyc chain services

## spring-comerce folder
This is a java spring monolith server

## bazel_rust
This is a rust project that has been altered to use Bazel, as an example

# Getting Started
To run this projects, you will need to install the latest version of Rust, Docker and Docker Compose.

## Build and Run Monolith
TODO

## Build and Run Microservices
In the root directory, type:

$ cd microservices/suschain_services

## Set up the Database
In each of the services you will need to set up your Diesel run database:

In this suschain_services folder, to get the database up:

$ docker-compose up database

Then in each sub folder type:

$ diesel setup
$ diesel migration run

That should create the releavat database table in your running postgres container.

## Run the Service
Now move back to the root suschain_services folder (microservices/suschain_services)

And type:

$ cargo build
$ docker-compose up


That is it, you should be up and running with microservices :-)
