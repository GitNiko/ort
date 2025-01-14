use super::ExecutionProvider;
use crate::{Error, ExecutionProviderDispatch, Result, SessionBuilder};

#[cfg(all(not(feature = "load-dynamic"), feature = "armnn"))]
extern "C" {
	fn OrtSessionOptionsAppendExecutionProvider_ArmNN(options: *mut ort_sys::OrtSessionOptions, use_arena: std::os::raw::c_int) -> ort_sys::OrtStatusPtr;
}

#[derive(Debug, Default, Clone)]
pub struct ArmNNExecutionProvider {
	use_arena: bool
}

impl ArmNNExecutionProvider {
	pub fn with_arena_allocator(mut self) -> Self {
		self.use_arena = true;
		self
	}

	pub fn build(self) -> ExecutionProviderDispatch {
		self.into()
	}
}

impl From<ArmNNExecutionProvider> for ExecutionProviderDispatch {
	fn from(value: ArmNNExecutionProvider) -> Self {
		ExecutionProviderDispatch::ArmNN(value)
	}
}

impl ExecutionProvider for ArmNNExecutionProvider {
	fn as_str(&self) -> &'static str {
		"ArmNNExecutionProvider"
	}

	#[allow(unused, unreachable_code)]
	fn register(&self, session_builder: &SessionBuilder) -> Result<()> {
		#[cfg(any(feature = "load-dynamic", feature = "armnn"))]
		{
			super::get_ep_register!(OrtSessionOptionsAppendExecutionProvider_ArmNN(options: *mut ort_sys::OrtSessionOptions, use_arena: std::os::raw::c_int) -> ort_sys::OrtStatusPtr);
			return crate::error::status_to_result(unsafe {
				OrtSessionOptionsAppendExecutionProvider_ArmNN(session_builder.session_options_ptr, self.use_arena.into())
			})
			.map_err(Error::ExecutionProvider);
		}

		Err(Error::ExecutionProviderNotRegistered(self.as_str()))
	}
}
