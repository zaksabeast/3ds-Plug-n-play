CRATE_NAME	= pnp_sys
TARGET_NAME	= pnp_sys
TITLE_ID = 0004013000CB9702
UNIQUE_ID = 0xCB97
PRODUCT_CODE = pnp

CARGO_BUILD_FLAGS = -Z build-std=core,alloc --target armv6k-nintendo-3ds

ifdef MODE3
	CARGO_BUILD_FLAGS	+= --no-default-features
	TITLE_ID	= 0004013000CB9802
	UNIQUE_ID	= 0xCB98
	PRODUCT_CODE	= pnp_m3
	TARGET_NAME	= pnp_sys_mode3
endif

RUST_OUT_DIR	=	../target/armv6k-nintendo-3ds
RUST_RELEASE_DIR	=	$(RUST_OUT_DIR)/release
RUST_DEBUG_DIR	=	$(RUST_OUT_DIR)/debug

OUT_DIR	= out
RELEASE_DIR = $(OUT_DIR)/release
DEBUG_DIR = $(OUT_DIR)/debug
RELEASE_TITLE_DIR = $(RELEASE_DIR)/$(TITLE_ID)
DEBUG_TITLE_DIR = $(DEBUG_DIR)/$(TITLE_ID)

RELEASE_ELF	= $(RUST_RELEASE_DIR)/$(CRATE_NAME).elf
DEBUG_ELF	= $(RUST_DEBUG_DIR)/$(CRATE_NAME).elf

RELEASE_CXI	= $(RUST_RELEASE_DIR)/$(TARGET_NAME).cxi
DEBUG_CXI	= $(RUST_DEBUG_DIR)/$(TARGET_NAME).cxi

RELEASE_CIA	= $(RELEASE_DIR)/$(TARGET_NAME).cia
DEBUG_CIA	= $(DEBUG_DIR)/$(TARGET_NAME).cia

RELEASE_EXHEADER = $(RELEASE_TITLE_DIR)/exheader.bin
DEBUG_EXHEADER = $(DEBUG_TITLE_DIR)/exheader.bin

SOURCES = $(wildcard src/*.rs) $(wildcard src/**/*.rs) $(wildcard src/**/**/*.rs) $(wildcard src/**/**/**/*.rs) $(wildcard src/**/**/**/**/*.rs)

.PHONY: all clean test docs lint

all: release debug test

docs:
	@cargo +nightly doc --open $(CARGO_BUILD_FLAGS)

# Nightly and unstable options prevent clippy from linting dependencies - https://github.com/rust-lang/rust-clippy/issues/1066
lint:
	@cargo +nightly clippy -Z unstable-options $(CARGO_BUILD_FLAGS)

test:
	@cargo +nightly test

release: $(RELEASE_EXHEADER) $(RELEASE_CIA)

debug: $(DEBUG_EXHEADER) $(DEBUG_CIA)

$(RELEASE_ELF) : $(SOURCES)
	@cargo +nightly build --release $(CARGO_BUILD_FLAGS)

$(DEBUG_ELF) : $(SOURCES)
	@cargo +nightly build $(CARGO_BUILD_FLAGS)

$(RELEASE_CIA) : $(RELEASE_ELF)
	@mkdir -p $(RELEASE_DIR)
	@makerom -f cia -o $(RELEASE_CIA) -rsf pnp.rsf -DPRODUCT_CODE=$(PRODUCT_CODE) -DUNIQUE_ID=$(UNIQUE_ID) -elf $(RELEASE_ELF)

$(DEBUG_CIA) : $(DEBUG_ELF)
	@mkdir -p $(RELEASE_DIR)
	@makerom -f cia -o $(DEBUG_CIA) -rsf pnp.rsf -DPRODUCT_CODE=$(PRODUCT_CODE) -DUNIQUE_ID=$(UNIQUE_ID) -elf $(DEBUG_ELF)

$(RELEASE_CXI) : $(RELEASE_ELF)
	@mkdir -p $(RELEASE_DIR)
	@makerom -f ncch -rsf pnp.rsf -DPRODUCT_CODE=$(PRODUCT_CODE) -DUNIQUE_ID=$(UNIQUE_ID) -o $@ -elf $<

$(DEBUG_CXI) : $(DEBUG_ELF)
	@mkdir -p $(DEBUG_DIR)
	@makerom -f ncch -rsf pnp.rsf -DPRODUCT_CODE=$(PRODUCT_CODE) -DUNIQUE_ID=$(UNIQUE_ID) -o $@ -elf $<

$(RELEASE_EXHEADER) : $(RELEASE_CXI)
	@mkdir -p $(RELEASE_TITLE_DIR)
	@ctrtool --exefsdir=$(RELEASE_TITLE_DIR) --exheader=$@ $< > /dev/null
	@echo Built code and exheader

$(DEBUG_EXHEADER) : $(DEBUG_CXI)
	@mkdir -p $(DEBUG_TITLE_DIR)
	@ctrtool --exefsdir=$(DEBUG_TITLE_DIR) --exheader=$@ $< > /dev/null
	@echo Built code and exheader

clean:
	@rm -rf $(OUT_DIR)
	@cargo clean
