pub mod gpu {
    pub mod abstraction { include!("gpu/abstraction.rs"); }
    pub mod buffer { include!("gpu/buffer.rs"); }
    pub mod command { include!("gpu/command.rs"); }
    pub mod synchronization { include!("gpu/synchronization.rs"); }
}
pub mod renderer {
    pub mod framebuffer { include!("renderer/framebuffer.rs"); }
    pub mod pipeline { include!("renderer/pipeline.rs"); }
    pub mod shader { include!("renderer/shader.rs"); }
    pub mod texture { include!("renderer/texture.rs"); }
    pub mod renderer { include!("renderer/renderer.rs"); }
}
pub mod vulkan {
    pub mod instance { include!("vulkan/instance.rs"); }
    pub mod device { include!("vulkan/device.rs"); }
    pub mod commands { include!("vulkan/commands.rs"); }
}
pub mod window {
    pub mod surface { include!("window/surface.rs"); }
    pub mod swapchain { include!("window/swapchain.rs"); }
}