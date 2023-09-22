use std::sync::Arc;

use vulkano::{command_buffer::allocator::StandardCommandBufferAllocator, device::{
    Device, DeviceCreateInfo, DeviceExtensions, Features, physical::PhysicalDevice, physical::PhysicalDeviceType,
    QueueCreateInfo, QueueFlags,
}, image::{ImageUsage}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, pipeline::graphics::viewport::Viewport, swapchain::{
    Surface, Swapchain, SwapchainCreateInfo,
}, sync::{self, GpuFuture}, Validated, Version, VulkanError, VulkanLibrary};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer};
use vulkano::device::Queue;
use vulkano::image::Image;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::swapchain::{acquire_next_image, SwapchainPresentInfo};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub trait Recorder {
    fn record(
        &self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        device: Arc<Device>,
        swapchain: Arc<Swapchain>,
        viewport: Viewport,
        image_index: usize,
    );
}

pub struct Renderer {
    pub command_buffer_recorders: Vec<Box<dyn Recorder>>,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub command_buffer_allocator: StandardCommandBufferAllocator,
    pub swapchain: Arc<Swapchain>,
    pub window: Arc<Window>,
    pub queue: Arc<Queue>,
    pub device: Arc<Device>,
    pub viewport: Viewport,
    pub allocator: Arc<StandardMemoryAllocator>,
}

impl Renderer {
    pub fn add_recorder<P>(&mut self, predicate: P) where P: Recorder + 'static, {
        self.command_buffer_recorders.push(Box::new(predicate));
    }

    pub fn new(window: Arc<Window>, event_loop: &EventLoop<()>) -> (Renderer, Vec<Arc<Image>>) {
        let library = VulkanLibrary::new().unwrap();
        let required_extensions = Surface::required_extensions(&event_loop);
        let mut enabled_layers = Vec::new();
        enabled_layers.push(String::from("VK_LAYER_KHRONOS_validation"));

        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY, // Include MoltenVK in search
                enabled_extensions: required_extensions,
                enabled_layers,
                ..Default::default()
            },
        ).unwrap();

        let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

        let mut device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| {
                supports_dynamic_rendering(p.as_ref())
            })
            .filter(|p| {
                p.supported_extensions().contains(&device_extensions)
            })
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })

                    .map(|i| (p, i as u32))
            })

            .min_by_key(|(p, _)| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                    _ => 5,
                }
            })
            .expect("no suitable physical device found");

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        if physical_device.api_version() < Version::V1_3 {
            device_extensions.khr_dynamic_rendering = true;
        }

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],

                enabled_extensions: device_extensions,

                enabled_features: Features {
                    dynamic_rendering: true,
                    multi_draw_indirect: true,
                    ..Features::empty()
                },

                ..Default::default()
            },
        )
            .unwrap();

        let queue = queues.next().unwrap();

        let (swapchain, images) = {
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(&surface, Default::default())
                .unwrap();

            // Choosing the internal format that the images will have.
            let image_format = device
                .physical_device()
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0;

            Swapchain::new(
                device.clone(),
                surface,
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count.max(2),
                    image_format,
                    image_extent: window.inner_size().into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),

                    ..Default::default()
                },
            ).unwrap()
        };

        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [0.0, 0.0],
            depth_range: 0.0..=1.0,
        };

        (Renderer {
            command_buffer_recorders: vec!(),
            swapchain,
            window,
            queue,
            recreate_swapchain: false,
            previous_frame_end: Some(sync::now(device.clone()).boxed()),
            command_buffer_allocator: StandardCommandBufferAllocator::new(device.clone(), Default::default()),
            device: device.clone(),
            viewport,
            allocator: Arc::new(StandardMemoryAllocator::new_default(device.clone())),
        }, images)
    }

    pub fn render(&mut self) {
        let (image_index, suboptimal, acquire_future) =
            match acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        let mut builder = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator,
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap();

        for x in &self.command_buffer_recorders {
            x.record(&mut builder, self.device.clone(), self.swapchain.clone(), self.viewport.clone(), image_index as usize);
        }

        // Finish building the command buffer by calling `build`.
        let command_buffer = builder.build().unwrap();

        let render_future = self.previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_index),
            )
            .then_signal_fence_and_flush();

        match render_future.map_err(Validated::unwrap) {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("failed to flush render_future: {e}");
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        }
    }
}

fn supports_dynamic_rendering(p: &PhysicalDevice) -> bool {
    p.api_version() >= Version::V1_3 || p.supported_extensions().khr_dynamic_rendering
}
