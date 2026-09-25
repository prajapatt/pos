extern crate pos_kernel;

use pos_kernel::ai::agent::context::SystemContext;
use pos_kernel::ai::agent::executor::{ExecutionError, Executor, ToolRequest, ToolResult, ToolRuntime};
use pos_kernel::permissions::approval::ApprovalToken;
use pos_kernel::permissions::capability::Capability;
use pos_kernel::permissions::policy::Policy;

struct EchoRuntime;

impl ToolRuntime for EchoRuntime {
    fn execute(&mut self, request: &ToolRequest, _context: &SystemContext) -> ToolResult {
        ToolResult {
            success: true,
            verified: true,
            detail: format!("{}:{}", request.operation, request.argument),
        }
    }
}

#[test]
fn executor_allows_verified_tool_run() {
    let mut executor = Executor::new(EchoRuntime, Policy::default());
    let context = SystemContext::new("tester");

    let request = ToolRequest {
        capability: Capability::SystemInfo,
        operation: "system.info".to_string(),
        argument: "cpu".to_string(),
    };

    let result = executor.execute(&request, &context, None).unwrap();
    assert!(result.success);
    assert!(result.verified);
    assert!(result.detail.contains("system.info:cpu"));
}

#[test]
fn executor_rejects_unverified_runtime_output() {
    struct UnverifiedRuntime;

    impl ToolRuntime for UnverifiedRuntime {
        fn execute(&mut self, _request: &ToolRequest, _context: &SystemContext) -> ToolResult {
            ToolResult {
                success: true,
                verified: false,
                detail: "not verified".to_string(),
            }
        }
    }

    let mut executor = Executor::new(UnverifiedRuntime, Policy::default());
    let context = SystemContext::new("tester");
    let request = ToolRequest {
        capability: Capability::DiagnosticsRun,
        operation: "diagnostics.run".to_string(),
        argument: "health".to_string(),
    };

    let err = executor.execute(&request, &context, None).unwrap_err();
    assert_eq!(err, ExecutionError::Unverified);
}

#[test]
fn approval_token_is_accepted_when_policy_allows() {
    let mut executor = Executor::new(EchoRuntime, Policy::default());
    let context = SystemContext::new("tester");
    let request = ToolRequest {
        capability: Capability::FilesystemWrite,
        operation: "filesystem.write".to_string(),
        argument: "/tmp/test".to_string(),
    };

    let approval = ApprovalToken::new("demo");
    let result = executor.execute(&request, &context, Some(approval)).unwrap();
    assert!(result.success);
}
