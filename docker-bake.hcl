group "default" {
  targets = [
    "ocicp",
  ]
}

target "ocicp" {
  target = "out"
  platforms = [
    "linux/arm64",
    "linux/amd64",
  ]
}