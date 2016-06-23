# Specification

## Extensions

Enabled extensions and associated methods must be checked at compile time.
Extensions may affect existing enum values, valid parameters and add functions.

### Example

A Python example to show how this should work for instance functions.

```
def createInstance(extensions):
	print("Creating instance with:", ', '.join(extensions))
	return 0x1

def getInstanceProcAddr(instance, name):
	print("Getting function:", name)
	def proc(*args):
		print(args)
		return None
	return proc

class Instance:
	def __init__(self):
		extensions = type(self).extensions()
		self.handle = createInstance(extensions)
		self._create_device = getInstanceProcAddr(self, "create_device")

	@classmethod
	def extensions(cls):
		return []

	def create_device(self):
		return self._create_device(self.handle)

class Debug(Instance):
	def __init__(self):
		super().__init__()
		self._add_callback = getInstanceProcAddr(self, "add_callback")

	@classmethod
	def extensions(cls):
		return ["debug"] + super().extensions()

	def log(self, error):
		print(error)

	def add_callback(self, f):
		self._add_callback(self.handle, f)

class XCBSurface(Instance):
	def __init__(self):
		print("Initializing surface")
		super().__init__()
		self._create_surface = getInstanceProcAddr(self, "create_surface")

	@classmethod
	def extensions(cls):
		return ["xcb"] + super().extensions()

	def create_surface(self):
		return self._create_surface(self.handle)

class MyInstance(Debug, XCBSurface, Instance):
	pass

i = MyInstance()
i.add_callback(lambda x: x)
i.log("foobar")
i.create_surface()
```

### Open Questions
#### Enum values

How to deal with add additional enum values? For example, `VK_EXT_debug_report`
adds an additional `VK_RESULT` to every function. Most extensions will add to
`VkStructureType`.

## Command Buffer State

State of command buffers should be checked at compile time. These are:

- Initial
  After `vkAllocateCommandBuffers` or `vkResetCommandBuffer`.
  Allowed actions: begin, reset
- Recording
  After `vkBeginCommandBuffer`.
  Allowed actions: `vkCmd*`, reset
- Executable
  After `vkEndCommandBuffer`
  Allowed actions: submit, reset, begin (if reset is allowed)
- Pending Execution
  After `vkQueueSubmit`
  Allowed actions: *None* (Command Pool may not be reset)

Reset of individual command buffers is allowed if the pool is created with
`VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT`.

### Begin parameters

If command buffer is begun with `VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT`,
it may only be submitted once.

`VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT` indicates a secondary command
buffer is entirely inside a render pass. The subpass must have been begun
with `VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS`

`VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT` indicates a primary or secondary
command buffer may be submitted or recorded into a primary command buffer
respectively while it is pending execution.

`pInheritanceInfo` describes what states a secondary command buffer inherits
from the primary command buffer it is recorded into. This includes:

- render pass (with `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`)
- subpass (with `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`)
- framebuffer (optional with `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`)
- occlusionQueryEnable (with *inherited queries* feature)
- queryFlags (with occlusionQueryEnable)
- pipelineStatistics (with pipeline statistics queries)

## Command Validity

Certain states must be fulfilled when executing `vkCmd*`. Here is a list,
details are TODO.

### vkCmdBindPipeline

Controls the state for `vkCmdDraw*`. The command pool must support the
pipeline's operation type.

If the subpass has no attachments and the *variable multisample rate* feature is
not enabled, the pipeline must have the same rasterizationSamples count as the
previous pipeline.

#### Open Questions

What is the use of a subpass without attachments?

### Dynamic State

Dynamic state is set with the commands `vkCmdSetViewport`, `vkCmdSetScissor`,
`vkCmdSetLineWidth`, `vkCmdSetDepthBias` `vkCmdSetBlendConstants`,
`vkCmdSetDepthBounds`, `vkCmdSetStencilCompareMask`, `vkCmdSetStencilWriteMask`
and `vkCmdSetStencilReference`. The corresponding dynamic state must be enabled
on the current pipeline.

For `vkCmdSetDepthBias`, if *depth bias clamping* feature is not available,
`depthBiasClamp` must be 0.

### Render Pass

Some commands may only be called inside or outside a render pass.

#### Inside
`vkCmdDraw*`, `vkCmdClearAttachments`, `vkCmdEndRenderPass`, `vkCmdNextSubpass`
#### Outside
`vkCmdCopy*`, `vkCmdBlitImage`, `vkCmdUpdateBuffer`, `vkCmdFillBuffer`, `vkCmdClearColorImage`, `vkCmdClearDepthStencilImage`, `vkCmdResolveImage`, `vkCmdResetQueryPool`, `vkCmdCopyQueryPoolResults`, `vkCmdBeginRenderPass`
#### Either
`vkCmdBind*`, `vkCmd*Query`, `vkCmdPipelineBarrier`, `vkCmdExecuteCommands` (depending on `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`)

### Memory

Regions may not overlap in memory.

All regions referred to must be contained within a buffer and it's allocation.

Any uniform or storage buffer data must be contained entirely in the
corresponding buffer, unless *robust buffer access* feature is enabled.

### Usage & Layouts

Transfer operations must have `VK_BUFFER/IMAGE_USAGE_TRANSFER_SRC/DST_BIT` set.

Image layout must be `VK_IMAGE_LAYOUT_TRANSFER_*_OPTIMAL`, or
`VK_IMAGE_LAYOUT_GENERAL`.

Multisample count must be 1 for blits and copy to/from buffer, and match for
image copy.

Formats must be compatible.

Regions must match physical device image transfer granularity of the physical
device. If the images are compressed, the regions must match block sizes.

Attachments must have the correct usage bits set for their use when beginning a
renderpass.

### Command Pool

Command pool must support transfer, graphics or compute operations. Graphics and
compute imply transfer support.


### Primary vs Secondary

Renderpasses and subpasses may only be begun in a primary command buffer. `vkCmdExecuteCommands` must be called from a primary command buffer

### vkCmdBindDescriptorSets

Descriptor sets must match pipeline layout (also specified).

Descriptor sets may not be updated between recording and command completion.

### vkCmdDraw*

A valid graphics pipeline must be bound.

Each descriptor set used by the bound pipeline must have a compatible descriptor
set bound.

Each push constant used by the bound pipeline must have been set.

Any dynamic state of the pipeline must be set.

### vkCmdDispatch*

A valid compute pipeline must be bound.

Each descriptor set used by the bound pipeline must have a compatible descriptor
set bound.

Each push constant used by the bound pipeline must have been set.

### vkCmdBlitImage

Command pool must support graphics operation.

`VK_FILTER_LINEAR` must be supported by format.

### vkCmdClear*

Queue must support graphics operation

### vkCmdClearColorImage

Image must not be compressed.

### vkCmdClearAttachments

Aspect mask must match attachment type of renderpass.

### vkCmdSet/ResetEvent

Pipeline stage must be enabled

### vkCmdWaitEvents

May not wait for a host-set event inside a renderpass.

### vkCmdPipelineBarrier

Can transition image layouts. Renderpass must have a subpass self-dependency if
this is called inside a renderpass.

### vkCmdExecuteCommands

May be called in a query depending on inheritance settings.

Must have been recorded with a compatible render pass if called within a renderpass.

#### Open Auestions

How can a secondary command buffer be recorded with a render pass if it may not
call `vkCmdBeginRenderPass`?

## Submission

Each submission contains zero or more command buffers, zero or more wait/signal
semaphores, and may signal a fence. A submission without any command buffers may
signal a fence when all previous work is complete (execution may overlap but not
be re-ordered).

`vkQueueSubmit` considered expensive. Probably acceptable to require waiting on
submissions whose command buffers will be re-used (can batch multiple buffers
per submission).

## Semaphores

Each queued wait must have a prior signal that is not waited upon.
