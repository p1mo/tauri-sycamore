include makefile.vars.mk

.PHONY: look
look:
	@echo CURR_OS:   $(CURR_OS)
	@echo CURR_ARCH: $(CURR_ARCH)

.PHONY: dev
dev:
	cargo tauri dev

.PHONY: devr
devr:
	cargo tauri dev --release

.PHONY: clean
clean:
	wsl -e bash -c "rm Cargo.lock && rm -r target/"
