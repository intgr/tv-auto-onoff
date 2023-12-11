# Dumb down your smart TV!

Automatically power on/off TV like a computer display.

* **On/off:** Observes computer's intended display power status and automatically turns on/off your smart TV
  (instead of seeing "no signal" when computer suspends the display).
* **Keep-alive:** When computer display is active, sends periodic pings to inhibit TV from sleeping.

### Support

Note: This project is still very basic and only supports two interfaces:

* **Display status source:** GNOME Shell (via Mutter D-Bus API)
* **Smart TVs:** Sony Bravia via
  the [Simple IP control protocol](https://pro-bravia.sony.net/develop/integrate/ssip/overview/index.html)

### Try it out

`tv-auto-onoff` is written in Rust and requires Cargo to be installed.

1. Check out this repository.
2. Run `cargo run <tv-ip-address>`
