include makefile.vars.mk

look:
	@echo CURR_OS:   $(CURR_OS)
	@echo CURR_ARCH: $(CURR_ARCH)

dev:
	cargo tauri dev

devr:
	cargo tauri dev --release

clean:
	ifeq ($(CURR_OS),WIN32)
    	wsl -e bash -c "rm Cargo.lock && rm -r target/"
	endif
	ifeq ($(CURR_OS),LINUX)
    	rm Cargo.lock && rm -r target/
	endif
	ifeq ($(CURR_OS),OSX)
    	rm Cargo.lock && rm -r target/
	endif