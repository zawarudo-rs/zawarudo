# Zawarudo
**live demo** [here](https://blockchain-danu.herokuapp.com/graphql)   
blockchain service implementation written in rust

## Usage

### Requirement

1. rust, install [here](https://rustup.rs/)
2. diesel, using `cargo install diesel` command
3. postgreSQL or alternatively using docker (and docker-compose)

### Installation

1. Clone this project
2. copy `.env.example` file as `.env`
3. (optional) if you don't have postgreSQL in your local machine you could use docker-compose command
   `docker-compose up` in the project directory, it will host a virtual postgreSQL instance at port `:5432` (user: postgres, pass: secret)
   and a sql admin at port `:8080`
4. edit your `.env` file with your preffered/setting setting
5. run `diesel setup`
6. run `cargo run --release`

### Interface

when the application succesfuly running it will open a graphql api than can be accessed by 2 methods:

using a `POST /graphql` fetch the api with a body as the example below:
`{ "query": "your query" }`

or just open `localhost:PORT/graphql` in your browser it will open a graphql playground

### Example Query

```graphql
// hello world query
{
   helloWorld
}
```

```
// get last 10 block
{
   getAllBlock(limit: 10, offset: 0) {
      index
      hash
      prevHash
      prevHash
      createdAt
      updatedAt
   }
}
```

```
// recursive query
{
   getAllBlock(limit: 10, offset: 0) {
      index
      hash
      prev {
         index
         hash
      } 
   }
}
```

```
// block creation
mutation {
   createNewBlock(data: "some message here") {
      data
      index
      prevHash
      hash
   }
}
```

```
// verify blockchain
{
  verifyAllBlock
}
```