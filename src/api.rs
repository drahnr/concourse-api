use crate::Method;

/// SaveConfig Councourse CI API call
pub fn save_config (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/config", team_name=team_name,pipeline_name=pipeline_name), Method::Put)
}



/// GetConfig Councourse CI API call
pub fn get_config (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/config", team_name=team_name,pipeline_name=pipeline_name), Method::Get)
}



/// CreateBuild Councourse CI API call
pub fn create_build (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/builds", team_name=team_name), Method::Post)
}



/// ListBuilds Councourse CI API call
pub fn list_builds () -> (String, Method) {
    (format!("/api/v1/builds", ), Method::Get)
}



/// GetBuild Councourse CI API call
pub fn get_build (build_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/builds/{build_id}", build_id=build_id), Method::Get)
}



/// GetBuildPlan Councourse CI API call
pub fn get_build_plan (build_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/builds/{build_id}/plan", build_id=build_id), Method::Get)
}



/// BuildEvents Councourse CI API call
pub fn build_events (build_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/builds/{build_id}/events", build_id=build_id), Method::Get)
}



/// BuildResources Councourse CI API call
pub fn build_resources (build_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/builds/{build_id}/resources", build_id=build_id), Method::Get)
}



/// AbortBuild Councourse CI API call
pub fn abort_build (build_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/builds/{build_id}/abort", build_id=build_id), Method::Put)
}



/// GetBuildPreparation Councourse CI API call
pub fn get_build_preparation (build_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/builds/{build_id}/preparation", build_id=build_id), Method::Get)
}



/// ListBuildArtifacts Councourse CI API call
pub fn list_build_artifacts (build_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/builds/{build_id}/artifacts", build_id=build_id), Method::Get)
}



/// SetBuildComment Councourse CI API call
pub fn set_build_comment (build_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/builds/{build_id}/comment", build_id=build_id), Method::Put)
}



/// ListAllJobs Councourse CI API call
pub fn list_all_jobs () -> (String, Method) {
    (format!("/api/v1/jobs", ), Method::Get)
}



/// ListJobs Councourse CI API call
pub fn list_jobs (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs", team_name=team_name,pipeline_name=pipeline_name), Method::Get)
}



/// GetJob Councourse CI API call
pub fn get_job (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name), Method::Get)
}



/// ListJobBuilds Councourse CI API call
pub fn list_job_builds (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/builds", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name), Method::Get)
}



/// CreateJobBuild Councourse CI API call
pub fn create_job_build (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/builds", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name), Method::Post)
}



/// RerunJobBuild Councourse CI API call
pub fn rerun_job_build (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display,build_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/builds/{build_name}", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name,build_name=build_name), Method::Post)
}



/// ListJobInputs Councourse CI API call
pub fn list_job_inputs (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/inputs", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name), Method::Get)
}



/// GetJobBuild Councourse CI API call
pub fn get_job_build (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display,build_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/builds/{build_name}", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name,build_name=build_name), Method::Get)
}



/// PauseJob Councourse CI API call
pub fn pause_job (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/pause", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name), Method::Put)
}



/// UnpauseJob Councourse CI API call
pub fn unpause_job (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/unpause", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name), Method::Put)
}



/// ScheduleJob Councourse CI API call
pub fn schedule_job (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/schedule", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name), Method::Put)
}



/// JobBadge Councourse CI API call
pub fn job_badge (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/badge", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name), Method::Get)
}



/// MainJobBadge Councourse CI API call
pub fn main_job_badge (pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/pipelines/{pipeline_name}/jobs/{job_name}/badge", pipeline_name=pipeline_name,job_name=job_name), Method::Get)
}



/// ClearTaskCache Councourse CI API call
pub fn clear_task_cache (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,job_name: impl ::std::fmt::Display,step_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/jobs/{job_name}/tasks/{step_name}/cache", team_name=team_name,pipeline_name=pipeline_name,job_name=job_name,step_name=step_name), Method::Delete)
}



/// ListAllPipelines Councourse CI API call
pub fn list_all_pipelines () -> (String, Method) {
    (format!("/api/v1/pipelines", ), Method::Get)
}



/// ListPipelines Councourse CI API call
pub fn list_pipelines (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines", team_name=team_name), Method::Get)
}



/// GetPipeline Councourse CI API call
pub fn get_pipeline (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}", team_name=team_name,pipeline_name=pipeline_name), Method::Get)
}



/// DeletePipeline Councourse CI API call
pub fn delete_pipeline (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}", team_name=team_name,pipeline_name=pipeline_name), Method::Delete)
}



/// OrderPipelines Councourse CI API call
pub fn order_pipelines (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/ordering", team_name=team_name), Method::Put)
}



/// OrderPipelinesWithinGroup Councourse CI API call
pub fn order_pipelines_within_group (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/ordering", team_name=team_name,pipeline_name=pipeline_name), Method::Put)
}



/// PausePipeline Councourse CI API call
pub fn pause_pipeline (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/pause", team_name=team_name,pipeline_name=pipeline_name), Method::Put)
}



/// ArchivePipeline Councourse CI API call
pub fn archive_pipeline (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/archive", team_name=team_name,pipeline_name=pipeline_name), Method::Put)
}



/// UnpausePipeline Councourse CI API call
pub fn unpause_pipeline (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/unpause", team_name=team_name,pipeline_name=pipeline_name), Method::Put)
}



/// ExposePipeline Councourse CI API call
pub fn expose_pipeline (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/expose", team_name=team_name,pipeline_name=pipeline_name), Method::Put)
}



/// HidePipeline Councourse CI API call
pub fn hide_pipeline (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/hide", team_name=team_name,pipeline_name=pipeline_name), Method::Put)
}



/// GetVersionsDB Councourse CI API call
pub fn get_versions_db (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/versions-db", team_name=team_name,pipeline_name=pipeline_name), Method::Get)
}



/// RenamePipeline Councourse CI API call
pub fn rename_pipeline (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/rename", team_name=team_name,pipeline_name=pipeline_name), Method::Put)
}



/// ListPipelineBuilds Councourse CI API call
pub fn list_pipeline_builds (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/builds", team_name=team_name,pipeline_name=pipeline_name), Method::Get)
}



/// CreatePipelineBuild Councourse CI API call
pub fn create_pipeline_build (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/builds", team_name=team_name,pipeline_name=pipeline_name), Method::Post)
}



/// PipelineBadge Councourse CI API call
pub fn pipeline_badge (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/badge", team_name=team_name,pipeline_name=pipeline_name), Method::Get)
}



/// ListAllResources Councourse CI API call
pub fn list_all_resources () -> (String, Method) {
    (format!("/api/v1/resources", ), Method::Get)
}



/// ListResources Councourse CI API call
pub fn list_resources (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources", team_name=team_name,pipeline_name=pipeline_name), Method::Get)
}



/// ListSharedForResource Councourse CI API call
pub fn list_shared_for_resource (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/shared", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Get)
}



/// ListSharedForResourceType Councourse CI API call
pub fn list_shared_for_resource_type (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_type_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resource-types/{resource_type_name}/shared", team_name=team_name,pipeline_name=pipeline_name,resource_type_name=resource_type_name), Method::Get)
}



/// ListResourceTypes Councourse CI API call
pub fn list_resource_types (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resource-types", team_name=team_name,pipeline_name=pipeline_name), Method::Get)
}



/// GetResource Councourse CI API call
pub fn get_resource (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Get)
}



/// CheckResource Councourse CI API call
pub fn check_resource (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/check", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Post)
}



/// CheckResourceWebHook Councourse CI API call
pub fn check_resource_web_hook (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/check/webhook", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Post)
}



/// CheckResourceType Councourse CI API call
pub fn check_resource_type (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_type_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resource-types/{resource_type_name}/check", team_name=team_name,pipeline_name=pipeline_name,resource_type_name=resource_type_name), Method::Post)
}



/// CheckPrototype Councourse CI API call
pub fn check_prototype (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,prototype_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/prototypes/{prototype_name}/check", team_name=team_name,pipeline_name=pipeline_name,prototype_name=prototype_name), Method::Post)
}



/// ClearResourceCache Councourse CI API call
pub fn clear_resource_cache (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/cache", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Delete)
}



/// ListResourceVersions Councourse CI API call
pub fn list_resource_versions (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Get)
}



/// ClearResourceVersions Councourse CI API call
pub fn clear_resource_versions (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Delete)
}



/// ClearResourceTypeVersions Councourse CI API call
pub fn clear_resource_type_versions (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_type_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resource-types/{resource_type_name}/versions", team_name=team_name,pipeline_name=pipeline_name,resource_type_name=resource_type_name), Method::Delete)
}



/// GetResourceVersion Councourse CI API call
pub fn get_resource_version (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display,resource_config_version_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions/{resource_config_version_id}", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name,resource_config_version_id=resource_config_version_id), Method::Get)
}



/// EnableResourceVersion Councourse CI API call
pub fn enable_resource_version (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display,resource_config_version_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions/{resource_config_version_id}/enable", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name,resource_config_version_id=resource_config_version_id), Method::Put)
}



/// DisableResourceVersion Councourse CI API call
pub fn disable_resource_version (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display,resource_config_version_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions/{resource_config_version_id}/disable", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name,resource_config_version_id=resource_config_version_id), Method::Put)
}



/// PinResourceVersion Councourse CI API call
pub fn pin_resource_version (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display,resource_config_version_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions/{resource_config_version_id}/pin", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name,resource_config_version_id=resource_config_version_id), Method::Put)
}



/// UnpinResource Councourse CI API call
pub fn unpin_resource (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/unpin", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Put)
}



/// SetPinCommentOnResource Councourse CI API call
pub fn set_pin_comment_on_resource (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/pin_comment", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Put)
}



/// ListBuildsWithVersionAsInput Councourse CI API call
pub fn list_builds_with_version_as_input (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display,resource_config_version_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions/{resource_config_version_id}/input_to", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name,resource_config_version_id=resource_config_version_id), Method::Get)
}



/// ListBuildsWithVersionAsOutput Councourse CI API call
pub fn list_builds_with_version_as_output (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display,resource_config_version_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions/{resource_config_version_id}/output_of", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name,resource_config_version_id=resource_config_version_id), Method::Get)
}



/// GetDownstreamResourceCausality Councourse CI API call
pub fn get_downstream_resource_causality (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display,resource_config_version_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions/{resource_config_version_id}/downstream", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name,resource_config_version_id=resource_config_version_id), Method::Get)
}



/// GetUpstreamResourceCausality Councourse CI API call
pub fn get_upstream_resource_causality (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display,resource_config_version_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/versions/{resource_config_version_id}/upstream", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name,resource_config_version_id=resource_config_version_id), Method::Get)
}



/// GetResourceCausality Councourse CI API call
pub fn get_resource_causality (team_name: impl ::std::fmt::Display,pipeline_name: impl ::std::fmt::Display,resource_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/pipelines/{pipeline_name}/resources/{resource_name}/causality", team_name=team_name,pipeline_name=pipeline_name,resource_name=resource_name), Method::Get)
}



/// GetCC Councourse CI API call
pub fn get_cc (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/cc.xml", team_name=team_name), Method::Get)
}



/// ListWorkers Councourse CI API call
pub fn list_workers () -> (String, Method) {
    (format!("/api/v1/workers", ), Method::Get)
}



/// RegisterWorker Councourse CI API call
pub fn register_worker () -> (String, Method) {
    (format!("/api/v1/workers", ), Method::Post)
}



/// LandWorker Councourse CI API call
pub fn land_worker (worker_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/workers/{worker_name}/land", worker_name=worker_name), Method::Put)
}



/// RetireWorker Councourse CI API call
pub fn retire_worker (worker_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/workers/{worker_name}/retire", worker_name=worker_name), Method::Put)
}



/// PruneWorker Councourse CI API call
pub fn prune_worker (worker_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/workers/{worker_name}/prune", worker_name=worker_name), Method::Put)
}



/// HeartbeatWorker Councourse CI API call
pub fn heartbeat_worker (worker_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/workers/{worker_name}/heartbeat", worker_name=worker_name), Method::Put)
}



/// DeleteWorker Councourse CI API call
pub fn delete_worker (worker_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/workers/{worker_name}", worker_name=worker_name), Method::Delete)
}



/// GetLogLevel Councourse CI API call
pub fn get_log_level () -> (String, Method) {
    (format!("/api/v1/log-level", ), Method::Get)
}



/// SetLogLevel Councourse CI API call
pub fn set_log_level () -> (String, Method) {
    (format!("/api/v1/log-level", ), Method::Put)
}



/// DownloadCLI Councourse CI API call
pub fn download_cli () -> (String, Method) {
    (format!("/api/v1/cli", ), Method::Get)
}



/// GetInfo Councourse CI API call
pub fn get_info () -> (String, Method) {
    (format!("/api/v1/info", ), Method::Get)
}



/// GetInfoCreds Councourse CI API call
pub fn get_info_creds () -> (String, Method) {
    (format!("/api/v1/info/creds", ), Method::Get)
}



/// GetUser Councourse CI API call
pub fn get_user () -> (String, Method) {
    (format!("/api/v1/user", ), Method::Get)
}



/// ListActiveUsersSince Councourse CI API call
pub fn list_active_users_since () -> (String, Method) {
    (format!("/api/v1/users", ), Method::Get)
}



/// ListDestroyingContainers Councourse CI API call
pub fn list_destroying_containers () -> (String, Method) {
    (format!("/api/v1/containers/destroying", ), Method::Get)
}



/// ReportWorkerContainers Councourse CI API call
pub fn report_worker_containers () -> (String, Method) {
    (format!("/api/v1/containers/report", ), Method::Put)
}



/// ListContainers Councourse CI API call
pub fn list_containers (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/containers", team_name=team_name), Method::Get)
}



/// GetContainer Councourse CI API call
pub fn get_container (team_name: impl ::std::fmt::Display,id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/containers/{id}", team_name=team_name,id=id), Method::Get)
}



/// HijackContainer Councourse CI API call
pub fn hijack_container (team_name: impl ::std::fmt::Display,id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/containers/{id}/hijack", team_name=team_name,id=id), Method::Get)
}



/// ListVolumes Councourse CI API call
pub fn list_volumes (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/volumes", team_name=team_name), Method::Get)
}



/// ListDestroyingVolumes Councourse CI API call
pub fn list_destroying_volumes () -> (String, Method) {
    (format!("/api/v1/volumes/destroying", ), Method::Get)
}



/// ReportWorkerVolumes Councourse CI API call
pub fn report_worker_volumes () -> (String, Method) {
    (format!("/api/v1/volumes/report", ), Method::Put)
}



/// ListTeams Councourse CI API call
pub fn list_teams () -> (String, Method) {
    (format!("/api/v1/teams", ), Method::Get)
}



/// GetTeam Councourse CI API call
pub fn get_team (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}", team_name=team_name), Method::Get)
}



/// SetTeam Councourse CI API call
pub fn set_team (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}", team_name=team_name), Method::Put)
}



/// RenameTeam Councourse CI API call
pub fn rename_team (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/rename", team_name=team_name), Method::Put)
}



/// DestroyTeam Councourse CI API call
pub fn destroy_team (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}", team_name=team_name), Method::Delete)
}



/// ListTeamBuilds Councourse CI API call
pub fn list_team_builds (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/builds", team_name=team_name), Method::Get)
}



/// CreateArtifact Councourse CI API call
pub fn create_artifact (team_name: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/artifacts", team_name=team_name), Method::Post)
}



/// GetArtifact Councourse CI API call
pub fn get_artifact (team_name: impl ::std::fmt::Display,artifact_id: impl ::std::fmt::Display) -> (String, Method) {
    (format!("/api/v1/teams/{team_name}/artifacts/{artifact_id}", team_name=team_name,artifact_id=artifact_id), Method::Get)
}



/// GetWall Councourse CI API call
pub fn get_wall () -> (String, Method) {
    (format!("/api/v1/wall", ), Method::Get)
}



/// SetWall Councourse CI API call
pub fn set_wall () -> (String, Method) {
    (format!("/api/v1/wall", ), Method::Put)
}



/// ClearWall Councourse CI API call
pub fn clear_wall () -> (String, Method) {
    (format!("/api/v1/wall", ), Method::Delete)
}


