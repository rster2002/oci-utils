# OCICP

Small utility for directly copying contents out of images.

## Usage

```shell
# Copy from local image
ocicp 'docker:some-image:/out/{bin,lib}/**/*' ./out

# ... with tag
ocicp 'docker:some-image:alpine:/out/{bin,lib}/**/*' ./out

# Copy from remote registry using stored credentials
ocicp 'docker+https://some-registry.com/some-image:latest:/out/{bin,lib}/**/*' ./out

# ... or provide a username and password directly
ocicp 'docker+https://username:password@some-registry.com/some-image:latest:/out/{bin,lib}/**/*' ./out
```