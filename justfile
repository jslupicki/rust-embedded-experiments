target := "thumbv7m-none-eabi"
gdb := "arm-none-eabi-gdb"

setup-env:
    rustup default nightly
    rustup target add {{target}}

build:
    cargo build

build-with-semihost:
    cargo build --features semihosting

flush: build
    openocd -f openocd.cfg -c "program target/{{target}}/debug/app verify reset exit"

erase: 
    pyocd erase --chip

gdb: build-with-semihost ocd
    {{gdb}} -x openocd.gdb -se target/{{target}}/debug/app

ocd: 
    openocd -f openocd.cfg &
