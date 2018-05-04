PACKAGE_NAME=vault-api
SWAGGER_CODEGEN_IMAGE := jimschubert/swagger-codegen-cli

.FORCE:
.PHONY: codegen
codegen: Cargo.toml
clean:
	-rm -r output

Cargo.toml: .FORCE
	@echo "Running swagger-codegen for $*..."
	@docker pull ${SWAGGER_CODEGEN_IMAGE}
	@docker run --rm -u $$(id -u $$USER) -v=${CURDIR}:/src ${SWAGGER_CODEGEN_IMAGE} generate \
                 -i /src/api/swagger.yaml \
                 -l rust-server \
                 -o /src \
                 --additional-properties packageName=${PACKAGE_NAME}
