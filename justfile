target := "thumbv7m-none-eabi"

build:
    cargo build

flush: build
    openocd -f openocd.cfg -c "program target/{{target}}/debug/app verify reset exit"

erase: 
    pyocd erase --chip
