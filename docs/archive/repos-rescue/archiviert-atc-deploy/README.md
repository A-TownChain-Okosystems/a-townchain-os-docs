# atc-deploy

Deployment-Tools für das A-TownChain-Ökosystem.

## Features (geplant)
- Smart Contract Deployment (ATCLang → ShivaVM Bytecode)
- Node-Setup-Scripts (Validator, Full-Node, Light-Node)
- Docker-Compose-Templates
- Kubernetes-Manifeste
- Terraform-Module (Cloud-Deployment)
- Blue-Green-Deployment
- Rollback-Management

## Architektur
```
atc-deploy/
├── src/
│   ├── deploy.py         # Contract-Deployment
│   ├── node_setup.py     # Node-Setup
│   └── rollback.py       # Rollback-Logic
├── docker/
│   ├── docker-compose.yml
│   └── Dockerfile
├── k8s/
│   ├── validator.yaml
│   └── fullnode.yaml
├── terraform/
│   ├── main.tf
│   └── variables.tf
└── scripts/
    ├── setup.sh
    └── deploy.sh
```


## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-blockchain`](https://github.com/A-TownChain-Okosystems/atc-blockchain)

## Copyright
Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.
