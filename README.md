# dumb-ina219
A dumb INA219 driver for the Raspberry Pi.

## Notes to dev
Specify cross comp target to check:
```bash
cross check --target aarch64-unknown-linux-gnu
```

Send to Pi:
```
scp -i ~/gipop_plc /Users/ander/Documents/proj/dumb-ina219/target/aarch64-unknown-linux-gnu/release/examples/ina219_xmpl pi@172.30.40.32:/home/pi/palanuk/anc/
```
