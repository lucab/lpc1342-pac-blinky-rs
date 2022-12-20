@default: cargo-build binary

default_profile := 'release'

cargo-build profile=default_profile:
  cargo build --profile "{{profile}}"

binary profile=default_profile:
  #!/usr/bin/env bash
  set -euo pipefail
  rm -f "target/thumbv7m-none-eabi/{{profile}}/lpc1342-pac-blinky.bin"
  objcopy "target/thumbv7m-none-eabi/{{profile}}/lpc1342-pac-blinky" -O binary "target/thumbv7m-none-eabi/{{profile}}/lpc1342-pac-blinky.bin"
  echo "Firmware binary at ./target/thumbv7m-none-eabi/{{profile}}/lpc1342-pac-blinky.bin"

convert profile=default_profile:
  #!/usr/bin/env bash
  set -euo pipefail
  rm -f "target/thumbv7m-none-eabi/{{profile}}/firmware.bin"
  lpc-flash "target/thumbv7m-none-eabi/{{profile}}/lpc1342-pac-blinky.bin" "target/thumbv7m-none-eabi/{{profile}}/firmware.bin"
  echo "Firmware blob for flashing at ./target/thumbv7m-none-eabi/{{profile}}/firmware.bin"

flash profile=default_profile:
  sudo mcopy -o -i '/dev/disk/by-label/CRP\\x20DISABLD' "target/thumbv7m-none-eabi/{{profile}}/firmware.bin" ::/firmware.bin
