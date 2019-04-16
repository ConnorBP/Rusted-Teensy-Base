BIN=teensy
OUTDIR=target/thumbv7em-none-eabihf/release
HEX=$(OUTDIR)/$(BIN).hex
ELF=$(OUTDIR)/$(BIN)

all:: $(ELF)

.PHONY: $(ELF)
$(ELF):
	cargo build --release --verbose

$(HEX): $(ELF)
	arm-none-eabihf-objcopy -O ihex $(ELF) $(HEX)

.PHONY: flash
flash: $(HEX)
	teensy_loader_cli -w -mmcu=mk20dx256 $(HEX) -v
