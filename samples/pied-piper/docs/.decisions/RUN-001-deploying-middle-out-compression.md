---
type: runbook
id: RUN-001
title: "Deploying Middle-Out Compression"
status: accepted
created: 2014-06-15
updated: 2014-07-01
authors: [gilfoyle, dinesh]
tags:
- api
- compression
- deployment
runbook_meta:
  last_verified: 2015-01-15
  estimated_duration: 30 minutes
links:
  depends_on: [ADR-001]
  relates_to: [ADR-002]
  implements: [ADR-001]
---

# Deploying Middle-Out Compression

## Purpose

This runbook describes how to deploy and configure the Pied Piper middle-out compression API service. Use this when setting up new compression workers, scaling the cluster, or recovering from failures.

## Prerequisites

- [ ] AWS credentials configured with EC2, S3, and SQS permissions
- [ ] Docker installed (version 18.09+)
- [ ] Access to Pied Piper container registry
- [ ] VPN connection to production network
- [ ] Compression license key from licensing server

## Steps

### 1. Pull Latest Container Image

```bash
# Authenticate to container registry
aws ecr get-login-password --region us-west-2 | docker login --username AWS --password-stdin 123456789.dkr.ecr.us-west-2.amazonaws.com

# Pull the compression worker image
docker pull 123456789.dkr.ecr.us-west-2.amazonaws.com/piper-compress:latest
```

**Expected outcome:** Image downloads successfully, approximately 2.3GB.

### 2. Configure Environment Variables

```bash
# Create environment file
cat > /etc/piper/compress.env << EOF
PIPER_LICENSE_KEY=${LICENSE_KEY}
PIPER_SQS_QUEUE=piper-jobs-prod
PIPER_S3_BUCKET=piper-data-prod
PIPER_WEISSMAN_TARGET=5.2
PIPER_MAX_FILE_SIZE=10GB
PIPER_WORKER_THREADS=8
EOF

# Verify configuration
cat /etc/piper/compress.env
```

**Expected outcome:** Environment file created with all required variables.

### 3. Start Compression Worker

```bash
# Start the compression container
docker run -d \
  --name piper-compress \
  --env-file /etc/piper/compress.env \
  --memory 16g \
  --cpus 8 \
  -v /data/piper:/data \
  123456789.dkr.ecr.us-west-2.amazonaws.com/piper-compress:latest

# Verify container is running
docker ps | grep piper-compress
docker logs piper-compress --tail 50
```

**Expected outcome:** Container starts and begins polling SQS queue for jobs.

## Verification

- [ ] Container shows "Ready to accept jobs" in logs
- [ ] SQS queue shows worker registered
- [ ] Test compression job completes successfully
- [ ] Weissman score meets target (>5.0)

```bash
# Run verification test
curl -X POST http://localhost:8080/health
curl -X POST http://localhost:8080/test -d '{"data":"test payload"}'
```

## Rollback

If the deployment fails or causes issues:

```bash
# Stop the new container
docker stop piper-compress
docker rm piper-compress

# Start previous version
docker run -d \
  --name piper-compress \
  --env-file /etc/piper/compress.env \
  123456789.dkr.ecr.us-west-2.amazonaws.com/piper-compress:previous
```

## Troubleshooting

| Problem | Solution |
|---------|----------|
| License key rejected | Verify key in licensing portal, check expiration |
| Low Weissman score | Increase PIPER_WORKER_THREADS, check CPU throttling |
| SQS connection timeout | Verify VPN connection, check security groups |
| Out of memory errors | Reduce PIPER_MAX_FILE_SIZE or increase container memory |
| Container won't start | Check Docker logs, verify image pulled successfully |

## Related

- ADR-001: Middle-Out Compression Algorithm
- ADR-002: Enterprise Platform Architecture
- RUN-003: New Employee Onboarding Guide
