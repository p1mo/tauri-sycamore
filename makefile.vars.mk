ifeq ($(OS),Windows_NT)
    CURR_OS += WIN32
    ifeq ($(PROCESSOR_ARCHITEW6432),AMD64)
        CURR_ARCH += AMD64
    else
        ifeq ($(PROCESSOR_ARCHITECTURE),AMD64)
            CURR_ARCH += AMD64
        endif
        ifeq ($(PROCESSOR_ARCHITECTURE),x86)
            CURR_ARCH += IA32
        endif
    endif
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        CURR_OS = LINUX
    endif
    ifeq ($(UNAME_S),Darwin)
        CURR_OS = OSX
    endif
    UNAME_P := $(shell uname -p)
    ifeq ($(UNAME_P),x86_64)
        CURR_ARCH = AMD64
    endif
    ifneq ($(filter %86,$(UNAME_P)),)
        CURR_ARCH = IA32
    endif
    ifneq ($(filter arm%,$(UNAME_P)),)
        CURR_ARCH = ARM
    endif
endif
