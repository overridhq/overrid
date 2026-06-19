# Per-Service SDS Index

## Purpose

This index lists every detailed SDS document for the Overrid tools, services, adapters, native apps, and support modules. The SDS files mirror the category layout in `docs/service_catalog` and link back to the original service implementation plans.

| Category | SDS files |
| --- | --- |
| Foundation and developer tooling | [Admin and Developer UI](foundation/admin_developer_ui.md), [CLI](foundation/cli.md), [Integration Test Harness](foundation/integration_test_harness.md), [Local Development Stack](foundation/local_development_stack.md), [Repository Layout](foundation/repository_layout.md), [SDK](foundation/sdk.md), [Shared Schema Package](foundation/shared_schema_package.md) |
| Control plane | [Overgate](control_plane/overgate.md), [Overkey](control_plane/overkey.md), [Overpass](control_plane/overpass.md), [Overqueue](control_plane/overqueue.md), [Overregistry](control_plane/overregistry.md), [Overrid Protocol Core](control_plane/overrid_protocol_core.md), [Overtenant](control_plane/overtenant.md), [Overwatch](control_plane/overwatch.md) |
| Execution and scheduling | [Benchmark Runner](execution_scheduling/benchmark_runner.md), [Hardware Discovery](execution_scheduling/hardware_discovery.md), [Node Installer](execution_scheduling/node_installer.md), [Overcache](execution_scheduling/overcache.md), [Overcell](execution_scheduling/overcell.md), [Overlease](execution_scheduling/overlease.md), [Overmesh](execution_scheduling/overmesh.md), [Overmeter](execution_scheduling/overmeter.md), [Overpack](execution_scheduling/overpack.md), [Overrun](execution_scheduling/overrun.md), [Oversched](execution_scheduling/oversched.md) |
| Data, storage, and namespace | [Overbase](data_storage_namespace/overbase.md), [Overstore](data_storage_namespace/overstore.md), [Overvault](data_storage_namespace/overvault.md), [Universal Namespace Service](data_storage_namespace/universal_namespace_service.md) |
| Trust, policy, verification, and disputes | [Challenge Task Service](trust_policy_verification/challenge_task_service.md), [Overclaim](trust_policy_verification/overclaim.md), [Overguard](trust_policy_verification/overguard.md), [Oververify](trust_policy_verification/oververify.md), [Policy Dry-Run API](trust_policy_verification/policy_dry_run_api.md), [Reputation and Anti-Sybil Service](trust_policy_verification/reputation_anti_sybil_service.md), [Workload Classifier](trust_policy_verification/workload_classifier.md) |
| Accounting, credits, billing, and rights | [ORU Account Service](accounting/oru_account_service.md), [Overasset](accounting/overasset.md), [Overbill](accounting/overbill.md), [Overgrant](accounting/overgrant.md), [Overmark](accounting/overmark.md), [Provider Payout Service](accounting/provider_payout_service.md), [Seal Ledger](accounting/seal_ledger.md) |
| Deployment and grid-resident backbone | [Backup and Restore Service](deployment_grid/backup_restore_service.md), [Deployment Planner](deployment_grid/deployment_planner.md), [Failover and Recovery Coordinator](deployment_grid/failover_recovery_coordinator.md), [Grid-Resident Service Packager](deployment_grid/grid_resident_service_packager.md), [Package Validator](deployment_grid/package_validator.md), [Release Strategy Service](deployment_grid/release_strategy_service.md), [System-Service Workload Class](deployment_grid/system_service_workload_class.md) |
| Federation and public capacity | [Federation Template Service](federation_public/federation_template_service.md), [Fraud Control Service](federation_public/fraud_control_service.md), [Public-Interest Pool Service](federation_public/public_interest_pool_service.md), [Public Provider Onboarding](federation_public/public_provider_onboarding.md), [Public Sandbox Profile](federation_public/public_sandbox_profile.md), [Purpose Tag Registry](federation_public/purpose_tag_registry.md) |
| AI, RAG, and model routing | [ADES Enrichment Adapter](ai_rag_model_routing/ades_enrichment_adapter.md), [AI Gateway Router](ai_rag_model_routing/ai_gateway_router.md), [Central AI Service](ai_rag_model_routing/central_ai_service.md), [Encrypted Docdex RAG Adapter](ai_rag_model_routing/encrypted_docdex_rag_adapter.md), [Lightweight Classifier](ai_rag_model_routing/lightweight_classifier.md), [Personal AI Assistant](ai_rag_model_routing/personal_ai_assistant.md) |
| Ecosystem adapters | [Codali Adapter](adapters/codali_adapter.md), [Docdex Adapter](adapters/docdex_adapter.md), [Mcoda Adapter](adapters/mcoda_adapter.md), [mSwarm Runtime Bridge](adapters/mswarm_runtime_bridge.md) |
| Native applications | [Central AI Stewardship Interface](native_apps/central_ai_stewardship_interface.md), [Directory Listings](native_apps/directory_listings.md), [Maps and Navigation](native_apps/maps_navigation.md), [Messaging Center](native_apps/messaging_center.md), [Search Engine](native_apps/search_engine.md), [Social Photo/Video App](native_apps/social_photo_video_app.md), [Wallet and Usage Center](native_apps/wallet_usage_center.md), [Workspace and Office Suite](native_apps/workspace_office_suite.md) |
| Governance, compliance, and operations | [Compliance Boundary Service](governance_ops/compliance_boundary_service.md), [Incident Response Service](governance_ops/incident_response_service.md), [Migration Tooling](governance_ops/migration_tooling.md), [Protocol Improvement Proposal Registry](governance_ops/pip_registry.md), [Stewardship Reporting Service](governance_ops/stewardship_reporting_service.md), [Threat Modeling and Security Review Tracker](governance_ops/threat_modeling_security_review_tracker.md) |
| Mobile service layer | [Mobile Backend Gateway](mobile/mobile_backend_gateway.md), [Mobile SDK](mobile/mobile_sdk.md) |

## Maintenance Rule

When a service implementation plan changes, update the matching SDS file in this folder and then rerun link, stale-note, removed-financial-projection, and Docdex indexing checks.

## Refinement Status

The SDS set has been reviewed in grouped refinement passes. The pass list below is kept as a compact public index; internal pass plans and progress logs are not part of the public repository.

Refined in pass 1:

- [Admin and Developer UI](foundation/admin_developer_ui.md)
- [CLI](foundation/cli.md)
- [Integration Test Harness](foundation/integration_test_harness.md)
- [Local Development Stack](foundation/local_development_stack.md)
- [Repository Layout](foundation/repository_layout.md)

Refined in pass 2:

- [SDK](foundation/sdk.md)
- [Shared Schema Package](foundation/shared_schema_package.md)
- [Overgate](control_plane/overgate.md)
- [Overkey](control_plane/overkey.md)
- [Overpass](control_plane/overpass.md)

Refined in pass 3:

- [Overqueue](control_plane/overqueue.md)
- [Overregistry](control_plane/overregistry.md)
- [Overrid Protocol Core](control_plane/overrid_protocol_core.md)
- [Overtenant](control_plane/overtenant.md)
- [Overwatch](control_plane/overwatch.md)

Refined in pass 4:

- [Benchmark Runner](execution_scheduling/benchmark_runner.md)
- [Hardware Discovery](execution_scheduling/hardware_discovery.md)
- [Node Installer](execution_scheduling/node_installer.md)
- [Overcache](execution_scheduling/overcache.md)
- [Overcell](execution_scheduling/overcell.md)

Refined in pass 5:

- [Overlease](execution_scheduling/overlease.md)
- [Overmesh](execution_scheduling/overmesh.md)
- [Overmeter](execution_scheduling/overmeter.md)
- [Overpack](execution_scheduling/overpack.md)
- [Overrun](execution_scheduling/overrun.md)

Refined or audited in pass 6, covering the user-requested inclusive range 25 through 30:

- [Overrun](execution_scheduling/overrun.md)
- [Oversched](execution_scheduling/oversched.md)
- [Overbase](data_storage_namespace/overbase.md)
- [Overstore](data_storage_namespace/overstore.md)
- [Overvault](data_storage_namespace/overvault.md)
- [Universal Namespace Service](data_storage_namespace/universal_namespace_service.md)

Refined in pass 7:

- [Challenge Task Service](trust_policy_verification/challenge_task_service.md)
- [Overclaim](trust_policy_verification/overclaim.md)
- [Overguard](trust_policy_verification/overguard.md)
- [Oververify](trust_policy_verification/oververify.md)
- [Policy Dry-Run API](trust_policy_verification/policy_dry_run_api.md)

Refined or audited in pass 8, covering the user-requested inclusive range 35 through 40:

- [Policy Dry-Run API](trust_policy_verification/policy_dry_run_api.md)
- [Reputation and Anti-Sybil Service](trust_policy_verification/reputation_anti_sybil_service.md)
- [Workload Classifier](trust_policy_verification/workload_classifier.md)
- [ORU Account Service](accounting/oru_account_service.md)
- [Overasset](accounting/overasset.md)
- [Overbill](accounting/overbill.md)

Refined in pass 9, covering the user-requested entries 41 through 45:

- [Overgrant](accounting/overgrant.md)
- [Overmark](accounting/overmark.md)
- [Provider Payout Service](accounting/provider_payout_service.md)
- [Seal Ledger](accounting/seal_ledger.md)
- [Backup and Restore Service](deployment_grid/backup_restore_service.md)

Refined in pass 10, covering the user-requested entries 46 through 50:

- [Deployment Planner](deployment_grid/deployment_planner.md)
- [Failover and Recovery Coordinator](deployment_grid/failover_recovery_coordinator.md)
- [Grid-Resident Service Packager](deployment_grid/grid_resident_service_packager.md)
- [Package Validator](deployment_grid/package_validator.md)
- [Release Strategy Service](deployment_grid/release_strategy_service.md)

Refined in pass 11, covering the user-requested entries 51 through 55:

- [System-Service Workload Class](deployment_grid/system_service_workload_class.md)
- [Federation Template Service](federation_public/federation_template_service.md)
- [Fraud Control Service](federation_public/fraud_control_service.md)
- [Public-Interest Pool Service](federation_public/public_interest_pool_service.md)
- [Public Provider Onboarding](federation_public/public_provider_onboarding.md)

Refined in pass 12, covering the user-requested entries 56 through 60:

- [Public Sandbox Profile](federation_public/public_sandbox_profile.md)
- [Purpose Tag Registry](federation_public/purpose_tag_registry.md)
- [ADES Enrichment Adapter](ai_rag_model_routing/ades_enrichment_adapter.md)
- [AI Gateway Router](ai_rag_model_routing/ai_gateway_router.md)
- [Central AI Service](ai_rag_model_routing/central_ai_service.md)

Refined in pass 13, covering the user-requested entries 61 through 65:

- [Encrypted Docdex RAG Adapter](ai_rag_model_routing/encrypted_docdex_rag_adapter.md)
- [Lightweight Classifier](ai_rag_model_routing/lightweight_classifier.md)
- [Personal AI Assistant](ai_rag_model_routing/personal_ai_assistant.md)
- [Codali Adapter](adapters/codali_adapter.md)
- [Docdex Adapter](adapters/docdex_adapter.md)

Refined in pass 14, covering the user-requested entries 66 through 70:

- [Mcoda Adapter](adapters/mcoda_adapter.md)
- [mSwarm Runtime Bridge](adapters/mswarm_runtime_bridge.md)
- [Central AI Stewardship Interface](native_apps/central_ai_stewardship_interface.md)
- [Directory Listings](native_apps/directory_listings.md)
- [Maps and Navigation](native_apps/maps_navigation.md)

Refined in pass 15, covering the user-requested entries 71 through 75:

- [Messaging Center](native_apps/messaging_center.md)
- [Search Engine](native_apps/search_engine.md)
- [Social Photo/Video App](native_apps/social_photo_video_app.md)
- [Wallet and Usage Center](native_apps/wallet_usage_center.md)
- [Workspace and Office Suite](native_apps/workspace_office_suite.md)

Refined in pass 16, covering the user-requested entries 76 through 80:

- [Compliance Boundary Service](governance_ops/compliance_boundary_service.md)
- [Incident Response Service](governance_ops/incident_response_service.md)
- [Migration Tooling](governance_ops/migration_tooling.md)
- [Protocol Improvement Proposal Registry](governance_ops/pip_registry.md)
- [Stewardship Reporting Service](governance_ops/stewardship_reporting_service.md)

Refined in pass 17, covering the user-requested entries 81 through 83 and completing the service SDS set:

- [Threat Modeling and Security Review Tracker](governance_ops/threat_modeling_security_review_tracker.md)
- [Mobile Backend Gateway](mobile/mobile_backend_gateway.md)
- [Mobile SDK](mobile/mobile_sdk.md)

All 83 service SDS files have now been reviewed and refined for generated-template risk.
