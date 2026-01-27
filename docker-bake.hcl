group "default" {
  targets = [
    "run",
    "out",
  ]
}

target "base" {
  platforms = [
    "linux/arm64",
    "linux/amd64",
  ]
}

target "run" {
  inherits = ["base"]
  target = "run"
  tags = [
    "forgejo.app.jumpdrive.dev/rster2002/ocicp:latest"
  ]
  output = [
    {
      type = "registry"
    }
  ]
}

target "out" {
  inherits = ["base"]
  target = "out"
  tags = [
    "ocicp-build:latest"
  ]
  output = [
    {
      type = "local"
      dest = "./dist"
    }
  ]
}