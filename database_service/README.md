# MongoDB service

## Building the Docker image

To build the image and ensure a docker network called `gaarkeuken-net` exists:

```
make build
```

## Running the Docker image

To deploy the build image on this network run:

```
make run		/* To run in production */

or

make dev		/* To run in dev mode (i.e. publish port 27017) */
```

To use the deployed service connect to it by deploying your own containerized application and as connection URL use: `gk-mongo:27017`

=======
# mongoService
