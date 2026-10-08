#[cfg(windows)]
pub mod windows {

    use std::ffi::OsString;
    use std::time::Duration;

    use anyhow::{Context, Result};
    use windows_service::service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    };
    use windows_service::service_control_handler::ServiceControlHandlerResult;
    use windows_service::service_dispatcher;
    use windows_service::{define_windows_service, service_control_handler};
    use windows_service_detector::is_running_as_windows_service;

    pub const SERVICE_NAME: &str = "PipeProxy";

    define_windows_service!(ffi_service_main, service_main);

    pub fn run_as_service() -> Result<()> {
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
            .context("windows service dispatch failed")
    }

    // Entry point for the service. Execution will start on a background thread after service_dispatcher::start
    fn service_main(arguments: Vec<OsString>) {
        if let Err(e) = service_worker(arguments) {
            eprintln!("service error: {e:?}");
        }
    }

    // actual service worker function
    fn service_worker(_arguments: Vec<OsString>) -> Result<()> {
        // Event handler
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
                ServiceControl::Stop | ServiceControl::Shutdown => {
                    service_shutdown();
                    ServiceControlHandlerResult::NoError
                }
                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };
        let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)
            .context("failed to register service control handler")?;

        status_handle
            .set_service_status(ServiceStatus {
                service_type: ServiceType::OWN_PROCESS,
                current_state: ServiceState::Running,
                controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
                exit_code: ServiceExitCode::Win32(0),
                checkpoint: 0,
                wait_hint: Duration::default(),
                process_id: None,
            })
            .context("service status error")?;

        Ok(())
    }

    fn service_shutdown() {
        eprintln!("service shutdown requested");
        std::process::exit(1);
    }

    pub fn is_service_run() -> bool {
        match is_running_as_windows_service() {
            Ok(ret) => ret,
            Err(err) => {
                eprintln!("windows service detection error: {}", err);
                false
            }
        }
    }
}

#[cfg(not(windows))]
pub mod non_windows {
    use anyhow::{Result, bail};

    pub fn run_as_service() -> Result<()> {
        bail!("Windows service mode is only supported on Windows.");
    }
}
