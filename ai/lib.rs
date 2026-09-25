pub mod permissions {
	pub mod capability {
		include!("permissions/capability.rs");
	}
	pub mod dangerous_actions {
		include!("permissions/dangerous_actions.rs");
	}
	pub mod approval {
		include!("permissions/approval.rs");
	}
	pub mod policy {
		include!("permissions/policy.rs");
	}
}

pub mod agent {
	pub mod context {
		include!("agent/context.rs");
	}
	pub mod executor {
		include!("agent/executor.rs");
	}
	pub mod memory {
		include!("agent/memory.rs");
	}
	pub mod reasoning {
		include!("agent/reasoning.rs");
	}
	pub mod planner {
		include!("agent/planner.rs");
	}
	pub mod agent {
		include!("agent/agent.rs");
	}
	pub mod main {
		include!("agent/main.rs");
	}
}

pub mod tools {
	pub mod filesystem {
		include!("tools/filesystem.rs");
	}
	pub mod system_info {
		include!("tools/system_info.rs");
	}
	pub mod processes {
		include!("tools/processes.rs");
	}
	pub mod diagnostics {
		include!("tools/diagnostics.rs");
	}
	pub mod host {
		include!("tools/host.rs");
	}
}

pub mod automation {
	pub mod action {
		include!("automation/action.rs");
	}
	pub mod workflow {
		include!("automation/workflow.rs");
	}
	pub mod scheduler {
		include!("automation/scheduler.rs");
	}
	pub mod macro_recorder {
		include!("automation/macro.rs");
	}
}

pub mod model {
	pub mod provider {
		include!("model/provider.rs");
	}
	pub mod local {
		include!("model/local.rs");
	}
	pub mod openrouter {
		include!("model/openrouter.rs");
	}
	pub mod streaming {
		include!("model/streaming.rs");
	}
	pub mod model_registry {
		include!("model/model_registry.rs");
	}
}

pub mod ui {
	pub mod activity { include!("ui/activity.rs"); }
	pub mod chat { include!("ui/chat.rs"); }
	pub mod command_bar { include!("ui/command_bar.rs"); }
	pub mod voice_indicator { include!("ui/voice_indicator.rs"); }
	pub mod ai_panel { include!("ui/ai-panel.rs"); }
}

pub mod vision {
	pub mod screen_context { include!("vision/screen_context.rs"); }
	pub mod screenshot { include!("vision/screenshot.rs"); }
	pub mod screen_capture { include!("vision/screen_capture.rs"); }
	pub mod ui_detection { include!("vision/ui_detection.rs"); }
}

pub mod voice {
	pub mod audio_pipeline { include!("voice/audio_pipeline.rs"); }
	pub mod microphone { include!("voice/microphone.rs"); }
	pub mod wake_word { include!("voice/wake_word.rs"); }
	pub mod speech_to_text { include!("voice/speech_to_text.rs"); }
	pub mod text_to_speech { include!("voice/text_to_speech.rs"); }
	pub mod voice_session { include!("voice/voice_session.rs"); }
}