/// Boot protocol integration
/// Handles Limine bootloader requests and responses

use limine::request::{FramebufferRequest, StackSizeRequest};

// Set the base revision to 3 (latest)
#[used]
#[link_section = ".limine_requests"]
pub static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new();

// Request a framebuffer
#[used]
#[link_section = ".limine_requests"]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

// Request a larger stack (recommended for Rust)
#[used]
#[link_section = ".limine_requests"]
pub static STACK_SIZE_REQUEST: StackSizeRequest = StackSizeRequest::new().with_size(0x100000);

// Start and end markers for Limine requests
#[used]
#[link_section = ".limine_requests_start"]
static _START_MARKER: [u8; 0] = [];

#[used]
#[link_section = ".limine_requests_end"]
static _END_MARKER: [u8; 0] = [];
