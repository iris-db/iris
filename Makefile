.PHONY: clean

# Compilation flags
C_DIR       = dist
C_FLAGS     = GOARCH=amd64
BINARY_NAME = sigma

# Go
GOCMD  = go
GOTEST = $(GOCMD) test -v

# Remove compilation directory
clean:
	rm -rf $(C_DIR)

test:
	$(GOTEST) ./api
	$(GOTEST) ./database/postgres

# Build binary for current OS
sigma-local:
	$(C_FLAGS) go build -o $(C_DIR)/local/$(BINARY_NAME)

# Build cross-platform binaries
sigma:
	$(call build_os,linux)
	$(call build_os,darwin)
	$(call build_os,windows,.exe)

# Builds binary for target GOOS
define build_os
	GOOS=$(1) $(C_FLAGS) go build -o $(C_DIR)/$(1)/$(BINARY_NAME)$(2)
endef
