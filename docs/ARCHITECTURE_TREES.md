# 🌳 Architektur-Bäume — Dual-Repo-Struktur

> **Regeneriert:** 2026-09-06 (nach 128→2-Konsolidierung) | **Agent:** Aurora (Base44)
> **Methode:** Verzeichnis-Scan, volle Tiefe, mit Zeilenzählung (ohne Build-Artefakte: target/, node_modules/, __pycache__)
> **Hinweis:** Die 70-Repo-Version vom 06.08.2026 ist im Git-Verlauf erhalten.

---

## Zusammenfassung

| Repository | Dateien | Zeilen |
|---|---|---|
| a-townchain-os (Code-Monorepo) | 2,230 | 307,809 |
| a-townchain-os-docs (Docs-Hub) | 2,653 | 433,010 |
| **Total** | **4,883** | **740,819** |

---

## a-townchain-os (Code-Monorepo)

| Metrik | Wert |
|---|---|
| Dateien | 2,230 |
| Zeilen | 307,809 |
| .atc | 605 |
| .md | 551 |
| .py | 287 |
| .rs | 227 |
| .tsx | 173 |
| .ts | 69 |

```
├── .github/
│   ├── workflows/
│   │   ├── ci.yml (42 lines)
│   │   └── codeql.yml (83 lines)
│   └── changelog-config.json (31 lines)
├── config/
│   ├── ai_models.json (26 lines)
│   ├── kai_config.toml (52 lines)
│   ├── mainnet_genesis.json (95 lines)
│   └── settings.json (50 lines)
├── docker/
│   ├── nginx/
│   │   └── nginx.conf (36 lines)
│   ├── Dockerfile.backend (17 lines)
│   ├── Dockerfile.bootstrap (8 lines)
│   ├── Dockerfile.core (57 lines)
│   ├── Dockerfile.frontend (58 lines)
│   ├── Dockerfile.gateway (61 lines)
│   ├── Dockerfile.node (24 lines)
│   ├── Makefile (43 lines)
│   ├── docker-compose.testnet.yml (137 lines)
│   ├── docker-compose.yml (175 lines)
│   └── prometheus.yml (22 lines)
├── scripts/
│   ├── build.sh (90 lines)
│   ├── ci-fix.sh (37 lines)
│   ├── fix-workflows.sh (68 lines)
│   ├── health.sh (41 lines)
│   ├── start.sh (95 lines)
│   ├── start_testnet.sh (49 lines)
│   ├── stop.sh (50 lines)
│   ├── sync-docs.sh (155 lines)
│   ├── test-report.sh (63 lines)
│   └── test.sh (106 lines)
├── src/
│   ├── atclang/
│   │   ├── ATCLANG_SPEC.md (295 lines)
│   │   ├── CHANGELOG.md (8 lines)
│   │   ├── CONTRIBUTING.md (19 lines)
│   │   └── README.md (46 lines)
│   ├── blockchain/
│   │   ├── core/
│   │   │   ├── __init__.py (2 lines)
│   │   │   ├── consensus.atc (144 lines)
│   │   │   ├── fork_atc85.atc (74 lines)
│   │   │   ├── fork_resolution.atc (7 lines)
│   │   │   ├── gas_fee.atc (7 lines)
│   │   │   ├── gas_fee_atc86.atc (71 lines)
│   │   │   ├── hybrid_atc84.atc (98 lines)
│   │   │   ├── hybrid_consensus.atc (7 lines)
│   │   │   ├── poh.atc (7 lines)
│   │   │   ├── poh.py (67 lines)
│   │   │   ├── poh_atc83.atc (79 lines)
│   │   │   ├── poh_integration.atc (78 lines)
│   │   │   ├── poh_integration.py (29 lines)
│   │   │   ├── pos.atc (7 lines)
│   │   │   ├── pos_atc82.atc (92 lines)
│   │   │   ├── pow.atc (7 lines)
│   │   │   ├── pow_atc81.atc (89 lines)
│   │   │   └── shiva_consensus.py (641 lines)
│   │   ├── __init__.py (1 lines)
│   │   ├── contract_registry.atc (6 lines)
│   │   ├── smart_contract_registry.atc (6 lines)
│   │   ├── smart_contract_registry.py (53 lines)
│   │   └── smart_contracts.atc (6 lines)
│   ├── bridge/
│   │   ├── Cargo.toml (12 lines)
│   │   ├── bridge_contract.atc (42 lines)
│   │   ├── config.rs (14 lines)
│   │   ├── error.rs (15 lines)
│   │   ├── fee.rs (19 lines)
│   │   ├── fee_manager.atc (18 lines)
│   │   ├── lib.rs (14 lines)
│   │   ├── lockbox.rs (39 lines)
│   │   ├── merkle_verifier.atc (18 lines)
│   │   ├── message_bus.atc (28 lines)
│   │   ├── relay.rs (39 lines)
│   │   ├── relayer_manager.atc (40 lines)
│   │   ├── state_proof.atc (18 lines)
│   │   ├── token_registry.atc (29 lines)
│   │   ├── validator.rs (33 lines)
│   │   ├── validator_set.atc (30 lines)
│   │   └── wrapped_token.atc (28 lines)
│   ├── contracts/
│   │   ├── ATCBridge.test.js (274 lines)
│   │   ├── __init__.py (5 lines)
│   │   ├── atc8300.atc (96 lines)
│   │   ├── atc8300_token.atc (178 lines)
│   │   ├── atc8300_token.py (126 lines)
│   │   ├── atcoin.atc (176 lines)
│   │   ├── atcoin.py (139 lines)
│   │   ├── base_contract.atc (69 lines)
│   │   ├── base_contract.py (87 lines)
│   │   ├── bridge_contract.atc (172 lines)
│   │   ├── bridge_contract.py (133 lines)
│   │   ├── genesis_token.atc (6 lines)
│   │   ├── genesis_token.py (74 lines)
│   │   ├── governance_contract.atc (237 lines)
│   │   ├── governance_contract.py (299 lines)
│   │   ├── keygen.py (140 lines)
│   │   ├── marketplace_contract.py (301 lines)
│   │   ├── requirements.txt (5 lines)
│   │   ├── revenue.atc (93 lines)
│   │   ├── setup.py (23 lines)
│   │   ├── smart_contract_registry.atc (88 lines)
│   │   ├── smart_contract_registry.py (53 lines)
│   │   ├── token.atc (72 lines)
│   │   ├── wallet_ecdsa.py (72 lines)
│   │   └── wallet_keygen.py (140 lines)
│   ├── core/
│   │   ├── crypto/
│   │   │   ├── __init__.py (19 lines)
│   │   │   ├── ecdsa.py (15 lines)
│   │   │   ├── hash_utils.py (22 lines)
│   │   │   └── key_generator.py (20 lines)
│   │   ├── kernel/
│   │   │   ├── api.py (883 lines)
│   │   │   ├── capabilities.py (160 lines)
│   │   │   ├── did.py (75 lines)
│   │   │   ├── kernel.py (424 lines)
│   │   │   ├── remote_capability.py (208 lines)
│   │   │   └── syscalls.atc (118 lines)
│   │   ├── __init__.py (1 lines)
│   │   ├── atcfs.py (122 lines)
│   │   ├── event_bus.py (16 lines)
│   │   ├── kai_cli.atc (6 lines)
│   │   └── module_loader.py (17 lines)
│   ├── franchise/
│   │   └── __init__.py (5 lines)
│   ├── game/
│   │   ├── __init__.py (5 lines)
│   │   ├── game_routes.py (59 lines)
│   │   └── marketplace_routes.py (93 lines)
│   ├── gateway/
│   │   ├── __init__.py (2 lines)
│   │   ├── auth.atc (82 lines)
│   │   ├── auth.py (19 lines)
│   │   ├── gateway.atc (138 lines)
│   │   ├── logger.atc (70 lines)
│   │   ├── logger.py (9 lines)
│   │   ├── main.atc (127 lines)
│   │   ├── main.py (47 lines)
│   │   ├── rate_limit.atc (50 lines)
│   │   ├── rate_limit.py (26 lines)
│   │   ├── requirements.txt (9 lines)
│   │   ├── router.atc (96 lines)
│   │   ├── signature_verify.atc (43 lines)
│   │   └── signature_verify.py (57 lines)
│   ├── governance/
│   │   ├── Cargo.toml (12 lines)
│   │   ├── delegation.atc (16 lines)
│   │   ├── delegation.rs (59 lines)
│   │   ├── lib.rs (13 lines)
│   │   ├── multisig.atc (16 lines)
│   │   ├── proposal.rs (73 lines)
│   │   ├── proposal_manager.atc (17 lines)
│   │   ├── timelock.atc (17 lines)
│   │   ├── timelock.rs (39 lines)
│   │   ├── treasury.atc (17 lines)
│   │   ├── treasury.rs (49 lines)
│   │   ├── voting.rs (67 lines)
│   │   └── voting_engine.atc (17 lines)
│   ├── legacy/
│   │   ├── atclang/
│   │   │   └── README.md (46 lines)
│   │   ├── atcpkg/
│   │   │   ├── FILE_REGISTER.md (20 lines)
│   │   │   └── README.md (39 lines)
│   │   ├── backend/
│   │   │   └── README.md (14 lines)
│   │   ├── blockchain/
│   │   │   └── README.md (14 lines)
│   │   ├── bootloader/
│   │   │   ├── FILE_REGISTER.md (13 lines)
│   │   │   └── README.md (107 lines)
│   │   ├── ci/
│   │   │   └── FILE_REGISTER.md (13 lines)
│   │   ├── ci-cd-fix/
│   │   │   ├── README.md (34 lines)
│   │   │   ├── apply-fix.sh (27 lines)
│   │   │   ├── ci-cd.yml (102 lines)
│   │   │   └── codeql.yml (32 lines)
│   │   ├── cli/
│   │   │   ├── FILE_REGISTER.md (13 lines)
│   │   │   └── README.md (120 lines)
│   │   ├── contracts/
│   │   │   └── README.md (70 lines)
│   │   ├── devnet/
│   │   │   └── README.md (554 lines)
│   │   ├── dns/
│   │   │   ├── FILE_REGISTER.md (13 lines)
│   │   │   └── README.md (107 lines)
│   │   ├── drivers/
│   │   │   └── README.md (108 lines)
│   │   ├── explorer/
│   │   │   └── FILE_REGISTER.md (18 lines)
│   │   ├── frontend/
│   │   │   ├── mobile/
│   │   │   │   └── README.md (2 lines)
│   │   │   ├── ui/
… (2474 weitere Einträge — vollständige Inventare: FILE_REGISTER.md im jeweiligen Repo)
```

---

## a-townchain-os-docs (Docs-Hub)

| Metrik | Wert |
|---|---|
| Dateien | 2,653 |
| Zeilen | 433,010 |
| .md | 1,609 |
| .atc | 365 |
| .py | 314 |
| .tsx | 159 |
| .ts | 44 |
| .gitignore | 38 |

```
├── TODO/
│   └── MASTER_TODO.md (61 lines)
├── aistudio/
│   ├── assets/
│   │   └── .aistudio/
│   │       └── .gitignore (1 lines)
│   ├── src/
│   │   ├── backend/
│   │   │   ├── blockchain/
│   │   │   │   └── engine.ts (129 lines)
│   │   │   └── p2p/
│   │   │       └── network.ts (77 lines)
│   │   ├── components/
│   │   │   ├── ATCAssetView.tsx (191 lines)
│   │   │   ├── ATCDjStudioView.tsx (445 lines)
│   │   │   ├── ATCLangEditor.tsx (625 lines)
│   │   │   ├── ATCWalletView.tsx (498 lines)
│   │   │   ├── ATownDashboardView.tsx (302 lines)
│   │   │   ├── ATownOSNode.tsx (1439 lines)
│   │   │   ├── ATownTestView.tsx (111 lines)
│   │   │   ├── AgentCivilizationView.tsx (152 lines)
│   │   │   ├── Ai3DRenderEngineTab.tsx (199 lines)
│   │   │   ├── AiAnimationEngineTab.tsx (198 lines)
│   │   │   ├── AiAudioEngineTab.tsx (198 lines)
│   │   │   ├── AiCharacterBioTab.tsx (199 lines)
│   │   │   ├── AiGameEngineTab.tsx (200 lines)
│   │   │   ├── AiKernelView.tsx (128 lines)
│   │   │   ├── AiOsEngineView.tsx (490 lines)
│   │   │   ├── AiSoftwareWorkflowView.tsx (229 lines)
│   │   │   ├── AiTimelineEngineTab.tsx (199 lines)
│   │   │   ├── AntiCheatView.tsx (261 lines)
│   │   │   ├── ApiHealthWidget.tsx (85 lines)
│   │   │   ├── ApiInterfacesView.tsx (189 lines)
│   │   │   ├── ApiOrchestratorView.tsx (354 lines)
│   │   │   ├── AppGlobeView.tsx (233 lines)
│   │   │   ├── ArchitectureDependencyGraph.tsx (248 lines)
│   │   │   ├── ArchitectureView.tsx (888 lines)
│   │   │   ├── AssetVaultView.tsx (187 lines)
│   │   │   ├── AtcAssetsDbView.tsx (250 lines)
│   │   │   ├── AtcCoreKernelView.tsx (144 lines)
│   │   │   ├── AtcLangArchitectureView.tsx (585 lines)
│   │   │   ├── AtcLangPlaygroundView.tsx (256 lines)
│   │   │   ├── AtcLangPresetsView.tsx (64 lines)
│   │   │   ├── AtcWhitepaperView.tsx (187 lines)
│   │   │   ├── AtsSuite.tsx (51 lines)
│   │   │   ├── AtvmSandboxView.test.tsx (85 lines)
│   │   │   ├── AtvmSandboxView.tsx (499 lines)
│   │   │   ├── BatteryStatus.tsx (269 lines)
│   │   │   ├── BattleArenaView.tsx (143 lines)
│   │   │   ├── BenchmarkCenterView.tsx (288 lines)
│   │   │   ├── BlockchainEcosystemView.tsx (224 lines)
│   │   │   ├── BlockchainLedgerView.tsx (247 lines)
│   │   │   ├── CalculatorView.tsx (74 lines)
│   │   │   ├── CalendarView.tsx (78 lines)
│   │   │   ├── CiCdPipelineView.tsx (159 lines)
│   │   │   ├── ClockView.tsx (72 lines)
│   │   │   ├── CodeAnalyzerView.tsx (90 lines)
│   │   │   ├── CommitHeatmap.tsx (110 lines)
│   │   │   ├── ComplianceEngineView.tsx (84 lines)
│   │   │   ├── ComplianceView.tsx (191 lines)
│   │   │   ├── ConflictResolutionModal.tsx (257 lines)
│   │   │   ├── ConsensusIntegrationGuide.tsx (1528 lines)
│   │   │   ├── CryptoVisualizationView.tsx (473 lines)
│   │   │   ├── DataProcessingView.tsx (78 lines)
│   │   │   ├── DbOrchestratorView.tsx (112 lines)
│   │   │   ├── DeFiLiquidityPoolView.tsx (255 lines)
│   │   │   ├── DependencyMapView.tsx (123 lines)
│   │   │   ├── DeploymentPipelineWidget.tsx (160 lines)
│   │   │   ├── DevToolsView.tsx (133 lines)
│   │   │   ├── DeveloperKnowledgeBaseView.tsx (359 lines)
│   │   │   ├── DistributedDatalakeView.tsx (73 lines)
│   │   │   ├── EcosystemInstaller.tsx (297 lines)
│   │   │   ├── EcosystemTreeOverlay.tsx (357 lines)
│   │   │   ├── EcosystemUmlView.tsx (143 lines)
│   │   │   ├── EcosystemVisualizerView.tsx (325 lines)
│   │   │   ├── FileManagerView.tsx (170 lines)
│   │   │   ├── FolderView.tsx (111 lines)
│   │   │   ├── FranchiseFactoryView.tsx (1733 lines)
│   │   │   ├── GateToHellBrowser.tsx (106 lines)
│   │   │   ├── GenesisBlockGeneratorView.tsx (150 lines)
│   │   │   ├── GitGraphVisualization.tsx (137 lines)
│   │   │   ├── GitHubRepoSyncView.tsx (1385 lines)
│   │   │   ├── GitHubStatusDashboard.tsx (643 lines)
│   │   │   ├── GitOpsView.tsx (126 lines)
│   │   │   ├── GovernanceView.tsx (601 lines)
│   │   │   ├── GpuPerformanceWidget.tsx (120 lines)
│   │   │   ├── HardwareDriversView.tsx (376 lines)
│   │   │   ├── IdeaToAppFlowchartView.tsx (153 lines)
│   │   │   ├── ImageGeneratorTab.tsx (117 lines)
│   │   │   ├── IntegrationsWindow.tsx (426 lines)
│   │   │   ├── InterfacesView.tsx (56 lines)
│   │   │   ├── JsExampleRunner.tsx (86 lines)
│   │   │   ├── LazyMetricsCharts.tsx (808 lines)
│   │   │   ├── LegalView.tsx (87 lines)
│   │   │   ├── LoginOverlay.tsx (690 lines)
│   │   │   ├── MainnetLaunchView.tsx (251 lines)
│   │   │   ├── MarketplaceView.tsx (450 lines)
│   │   │   ├── MediaApps.tsx (254 lines)
│   │   │   ├── MetricsDashboard.tsx (105 lines)
│   │   │   ├── MetricsView.tsx (1476 lines)
│   │   │   ├── ModulesPluginView.tsx (309 lines)
│   │   │   ├── NetworkExplorerView.test.tsx (121 lines)
│   │   │   ├── NetworkExplorerView.tsx (370 lines)
│   │   │   ├── NetworkTopologyView.tsx (38 lines)
│   │   │   ├── NodeHealthMonitor.tsx (113 lines)
│   │   │   ├── NotepadView.tsx (67 lines)
│   │   │   ├── OfficeApps.tsx (14 lines)
│   │   │   ├── OfficeSuiteView.tsx (271 lines)
│   │   │   ├── P2PChatView.tsx (277 lines)
│   │   │   ├── Paint3DView.tsx (140 lines)
│   │   │   ├── PaymentSystemView.tsx (93 lines)
│   │   │   ├── PipelineGeneratorTab.tsx (433 lines)
│   │   │   ├── PoAITrainingEngineView.tsx (173 lines)
│   │   │   ├── ProjectAuditDashboard.tsx (135 lines)
│   │   │   ├── ProjectHubView.tsx (501 lines)
│   │   │   ├── ProtocolsView.tsx (207 lines)
│   │   │   ├── ReportsView.tsx (202 lines)
│   │   │   ├── RepositoryActivityChart.tsx (145 lines)
│   │   │   ├── RepositoryLineChart.tsx (198 lines)
│   │   │   ├── RescueSystemView.tsx (307 lines)
│   │   │   ├── RoadmapView.tsx (196 lines)
│   │   │   ├── SemanticGraphView.tsx (86 lines)
│   │   │   ├── SessionExportView.tsx (221 lines)
│   │   │   ├── SettingsView.tsx (2312 lines)
│   │   │   ├── SocialMediaView.tsx (287 lines)
│   │   │   ├── SoftwareAuditView.tsx (885 lines)
│   │   │   ├── SoftwareKnowledgeDbView.tsx (380 lines)
│   │   │   ├── SourceCodeViewer.tsx (547 lines)
│   │   │   ├── SpecificSettingsViews.tsx (306 lines)
│   │   │   ├── StorageManagerView.tsx (258 lines)
│   │   │   ├── StrategicArchitectureMap.tsx (243 lines)
│   │   │   ├── StructureView.tsx (505 lines)
│   │   │   ├── SyncDashboardModal.tsx (88 lines)
│   │   │   ├── SyncHistoryModal.tsx (249 lines)
│   │   │   ├── SyncMetricsView.tsx (170 lines)
│   │   │   ├── SyncStatusDonutChart.tsx (99 lines)
│   │   │   ├── SyncStatusOverview.tsx (168 lines)
│   │   │   ├── SystemDiagnosticsView.tsx (337 lines)
│   │   │   ├── SystemFinderView.tsx (56 lines)
│   │   │   ├── SystemHealthDashboard.tsx (246 lines)
│   │   │   ├── SystemHealthDashboardWidget.tsx (63 lines)
│   │   │   ├── SystemLogsView.tsx (89 lines)
│   │   │   ├── TaskManagerView.tsx (82 lines)
│   │   │   ├── TechDocsView.tsx (335 lines)
│   │   │   ├── TechTreeView.tsx (420 lines)
│   │   │   ├── TerminalView.tsx (189 lines)
│   │   │   ├── TestnetOrchestrationView.tsx (178 lines)
│   │   │   ├── TestnetSimulationView.tsx (298 lines)
│   │   │   ├── TextGeneratorTab.tsx (177 lines)
│   │   │   ├── ThemeSwitcher.tsx (143 lines)
│   │   │   ├── TodoView.tsx (383 lines)
│   │   │   ├── TooltipIcon.tsx (29 lines)
│   │   │   ├── TxOrchestratorView.tsx (105 lines)
│   │   │   ├── UserProfileView.tsx (255 lines)
│   │   │   ├── VideoGeneratorTab.tsx (176 lines)
│   │   │   ├── WebhookMonitor.tsx (145 lines)
│   │   │   ├── Window.tsx (158 lines)
│   │   │   ├── WindowExtras.tsx (87 lines)
│   │   │   ├── ZeroKnowledgeProofView.tsx (129 lines)
│   │   │   ├── ZkCircuitEditorView.tsx (108 lines)
│   │   │   └── ZkVisualizationView.tsx (99 lines)
│   │   ├── contexts/
│   │   │   ├── FirebaseContext.tsx (94 lines)
│   │   │   ├── GoogleWorkspaceContext.tsx (83 lines)
│   │   │   ├── SyncMetricsContext.tsx (47 lines)
│   │   │   └── WalletContext.tsx (45 lines)
│   │   ├── db/
│   │   │   ├── drizzle.config.ts (29 lines)
│   │   │   ├── index.ts (24 lines)
│   │   │   └── schema.ts (11 lines)
│   │   ├── hooks/
│   │   │   ├── useGoogleSheetsSync.ts (220 lines)
│   │   │   └── useKeyboardShortcut.ts (30 lines)
│   │   ├── lib/
│   │   │   ├── CryptoEngine.ts (42 lines)
│   │   │   ├── firebase-admin.ts (15 lines)
│   │   │   ├── firebase.ts (64 lines)
│   │   │   ├── indexedDb.ts (88 lines)
│   │   │   ├── syncLogic.test.ts (82 lines)
│   │   │   └── syncLogic.ts (68 lines)
│   │   ├── middleware/
│   │   │   └── auth.ts (30 lines)
│   │   ├── routes/
│   │   │   └── notion.ts (146 lines)
│   │   ├── services/
│   │   │   ├── SyncService.ts (106 lines)
│   │   │   └── githubSync.ts (37 lines)
│   │   ├── utils/
│   │   │   ├── appSync.tsx (84 lines)
│   │   │   ├── auditUtils.test.ts (56 lines)
│   │   │   ├── auditUtils.ts (27 lines)
│   │   │   └── crypto.ts (73 lines)
│   │   ├── App.tsx (5440 lines)
│   │   ├── DesktopApp.tsx (2740 lines)
│   │   ├── atcLangRoadmapData.ts (201 lines)
│   │   ├── atcLangWikiData.ts (227 lines)
│   │   ├── auditData.ts (76 lines)
│   │   ├── data.ts (411 lines)
│   │   ├── ecosystemData.ts (291 lines)
│   │   ├── fix_translation.cjs (9 lines)
… (3046 weitere Einträge — vollständige Inventare: FILE_REGISTER.md im jeweiligen Repo)
```
