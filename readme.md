# OCICP

Small utility for directly copying contents out of images.

## Usage

```shell
# Copy from local image
ocicp 'docker:some-image:latest:/out/{bin,lib}/**/*' ./out

# Copy from remote registry
ocicp 'docker+https://username:password@some-registry.com/some-image:latest:/out/{bin,lib}/**/*' ./out
```