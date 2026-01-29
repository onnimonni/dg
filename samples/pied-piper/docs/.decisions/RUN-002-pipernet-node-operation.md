---
type: runbook
id: RUN-002
title: "PiperNet Node Operation"
status: deprecated
created: 2018-02-01
updated: 2018-03-15
authors: [gilfoyle]
tags:
- decentralized
- node
- operation
- pipernet
runbook_meta:
  last_verified: 2019-10-01
  estimated_duration: 45 minutes
links:
  depends_on: [ADR-004]
  relates_to: [ADR-005, INC-003]
  implements: [ADR-004]
---

# PiperNet Node Operation

> **DEPRECATED**: PiperNet was intentionally destroyed on December 21, 2019 (see DEC-005). This runbook is preserved for historical reference only.

## Purpose

This runbook describes how to set up and operate a PiperNet node. Node operators contribute storage and bandwidth to the decentralized network in exchange for PiedPiperCoin (PPC) rewards.

## Prerequisites

- [ ] Linux server (Ubuntu 18.04+ recommended)
- [ ] Minimum 1TB available storage
- [ ] 100Mbps symmetric internet connection
- [ ] Static IP address or dynamic DNS configured
- [ ] PiperNet wallet address for receiving rewards

## Steps

### 1. Install PiperNet Daemon

```bash
# Add PiperNet repository
curl -fsSL https://get.pipernet.io/gpg | sudo apt-key add -
echo "deb https://get.pipernet.io/apt stable main" | sudo tee /etc/apt/sources.list.d/pipernet.list

# Install pipernet-node
sudo apt update
sudo apt install pipernet-node

# Verify installation
pipernet --version
```

**Expected outcome:** PiperNet daemon version 2.3.x or higher installed.

### 2. Configure Node Settings

```bash
# Initialize node configuration
pipernet init --wallet YOUR_WALLET_ADDRESS

# Edit configuration
sudo nano /etc/pipernet/config.yaml
```

Configuration options:
```yaml
node:
  name: "my-node-name"
  region: "us-west"
  storage_path: /data/pipernet
  storage_limit: 1000GB
  bandwidth_limit: 100Mbps

wallet:
  address: "0x..."
  payout_threshold: 100 PPC

network:
  port: 8443
  enable_upnp: true
```

**Expected outcome:** Configuration file created and customized.

### 3. Start Node and Join Network

```bash
# Start PiperNet daemon
sudo systemctl enable pipernet
sudo systemctl start pipernet

# Check node status
pipernet status

# View logs
journalctl -u pipernet -f
```

**Expected outcome:** Node connects to network and begins syncing with peers.

## Verification

- [ ] Node shows "Connected" status with 10+ peers
- [ ] Storage allocation shows available capacity
- [ ] Wallet shows node registration transaction
- [ ] First PPC rewards received within 24 hours

```bash
# Check node health
pipernet health

# View earnings
pipernet wallet balance
pipernet wallet history
```

## Rollback

To safely disconnect from the network:

```bash
# Graceful shutdown (allows data migration)
pipernet leave --graceful --timeout 24h

# Emergency shutdown (may forfeit pending rewards)
sudo systemctl stop pipernet
```

## Troubleshooting

| Problem | Solution |
|---------|----------|
| No peers connecting | Check firewall, verify port 8443 is open |
| Storage not recognized | Verify disk permissions, check storage_path |
| Low reward rate | Improve uptime, increase storage allocation |
| Sync stuck | Restart daemon, check network connectivity |
| Slashing penalty | Review node logs for policy violations |

## Related

- ADR-004: PiperNet Decentralized Architecture
- ADR-005: PiedPiperCoin Tokenomics
- INC-003: 51 Percent Attack on PiperNet
