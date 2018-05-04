Swagger interface file for the Vault API, as documented
[here](https://www.vaultproject.io/docs/auth/token.html) and
[here](https://www.vaultproject.io/api/index.html).

This does not spec out the whole API - just those that we are interested in.

# Building client/server libraries

Use [swagger-codegen](https://github.com/swagger-api/swagger-codegen).

Or alternatively, run `make` to build Rust crates.

# Known issues

The [Swagger 2.0 specification](http://swagger.io/specification) is not a perfect fit for this API:

1. The swagger specification doesn't allow arbitrary paths. So, there is no way to spec out the
following paths without a lot of tedium:
  - /secret/secret-1
  - /secret/foo/secret-2
  - /secret/foo/bar/secret-3
2. The swagger specification doesn't let you use a query parameter to switch to a logically
different endpoint, for example the following each have different response types:
  - /secret/foo
  - /secret/foo?list=1
  - /secret/foo?help=1

Also, the existing Rust implementation of swagger-codegen doesn't support all the features required
here. See:
- https://gitlab.datcon.co.uk/orchestration/vault-schema/issues/1
