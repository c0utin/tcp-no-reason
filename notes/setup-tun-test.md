# Setup TUN Interface with `tcp-no-reason`

This guide will walk you through the steps to build and run the `tcp-no-reason` binary, configure a TUN interface, and send a packet.

## Prerequisites

Ensure you have the following installed on your system:

- Rust and Cargo (for building the binary)
- `setcap` utility
- Root access for network-related commands

## Steps

### 1. Build the `tcp-no-reason` Project

First, ensure that you have the project built in release mode:

```bash
cargo b --release
```

This will compile the binary in the `target/release/` directory.

### 2. Set Capabilities for the Binary

Next, give the binary the necessary capabilities to manage network interfaces without needing full root privileges:

```bash
sudo setcap cap_net_admin=eip target/release/tcp-no-reason
```

The `cap_net_admin` capability allows the binary to create and manage network interfaces like `tun`.

### 3. Run the `tcp-no-reason` Binary

Run the binary to create the `tun` device:

```bash
./target/release/tcp-no-reason
```

This should create a new TUN interface (e.g., `tun0`).

### 4. Verify the TUN Interface

In a **separate terminal**, check if the TUN interface has been created using:

```bash
ip addr
```

You should see the `tun` interface in the list of devices, something like:

```
tun0: <POINTOPOINT,MULTICAST,NOARP,UP,LOWER_UP> mtu 1500 qdisc noqueue state UNKNOWN group default qlen 500
    link/none 
```

### 5. Assign an IP Address to the TUN Interface

Now, assign an IP address to the `tun` interface using the following command:

```bash
sudo ip addr add 192.168.0.1/24 dev tun0
```

This will assign the IP `192.168.0.1` to `tun0`.

### 6. Bring the Interface Up

After assigning the IP, bring the interface up so that it's active and ready to use:

```bash
sudo ip link set up dev tun0
```

Once this is done, the `tun` interface should be fully operational.

## Summary of Commands

```bash
cargo b --release
sudo setcap cap_net_admin=eip target/release/tcp-no-reason
./target/release/tcp-no-reason
ip addr   # In another terminal to verify the tun interface
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
```

### Additional Notes

- Ensure your user has the necessary permissions to use `sudo` for setting capabilities and network configuration.
- The IP address `192.168.0.1/24` is used as an example, you can change it based on your specific network setup.

This documentation should cover the basic steps to create and configure a TUN interface using your Rust project!
