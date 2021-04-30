PACKAGE_NAME=vault-api
OPENAPI_GENERATOR_IMAGE := openapitools/openapi-generator-cli

.FORCE:
.PHONY: codegen
codegen: Cargo.toml
clean:
	-rm -r output

Cargo.toml: .FORCE
	@echo "Running openapi-generator for $*..."
	@docker pull ${OPENAPI_GENERATOR_IMAGE}
	@docker run --rm -u $$(id -u $$USER) -v=${CURDIR}:/src ${OPENAPI_GENERATOR_IMAGE} generate \
                 -i /src/api/openapi.yaml \
                 -g rust-server \
                 -o /src \
                 --additional-properties packageName=${PACKAGE_NAME} \
				 --skip-validate-spec
