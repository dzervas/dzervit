# DZerv.IT

A joke built with love, affection, Rust, React and kubernetes.

For usage and API reference, see [here](https://we.dzerv.it/docs)

## Development Setup

`npm start` to start the webpack build server - just spits a massive javascript
file under `/bundle` and a couple of static files.

`cargo run` to start the rocket dev web server. Handles the API and the
static file serving. Contrary to current (2021) hipster specification documents,
it does not feature a hot reload functionality, so you have to re-run the
command each time you change the source. I'm sure a haskell utility will solve
this problem.

## Deployment

There's a pre-built docker container in GitHub Packages. To pull it:

`docker pull ghcr.io/dzervas/dzervit`

There's also a helm chart to deploy the whole app:

```bash
helm repo add https://dzervas.github.io/dzervit/
helm repo update
helm install dzervit dzervit/dzervit -n dzervit --create-namespace
```

For the values check [here](chart/dzervit/values.yaml).
