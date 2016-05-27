# VKRS

Vulkan/Rust layer. For now the aim is just to learn about wrapping C in Rust,
including proper use of `unsafe` and macros.

## Aims

All use of the Vulkan API will be allowed, and there will be a minimum of
run-time checks. They will be kept off the hot paths and used only determine
platform-specific features such as available memory types.

For now structs are manually generated, this should be automated. Depends on a C parser or
KhronosGrou/Vulkan-Docs#120.
