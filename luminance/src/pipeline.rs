//! Dynamic rendering pipelines.
//!
//! This module gives you materials to build *dynamic* rendering **pipelines**. A `Pipeline`
//! represents a functional stream that consumes geometric data and rasterizes them.

use blending;
use framebuffer::{ColorSlot, DepthSlot, Framebuffer, HasFramebuffer};
use shader::program::{HasProgram, Program};
use tessellation::{HasTessellation, Tessellation};
use texture::{Dimensionable, HasTexture, Layerable};

/// Trait to implement to add `Pipeline` support.
pub trait HasPipeline: HasFramebuffer + HasProgram + HasTessellation + HasTexture + Sized {
  /// Execute a pipeline command, resulting in altering the embedded framebuffer.
  fn run_pipeline<L, D, CS, DS>(cmd: &Pipeline<Self, L, D, CS, DS>)
    where L: Layerable,
          D: Dimensionable,
          D::Size: Copy,
          CS: ColorSlot<Self, L, D>,
          DS: DepthSlot<Self, L, D>;
  /// Execute a shading command.
  fn run_shading_command<T>(shading_cmd: &ShadingCommand<Self, T>);
}

/// A dynamic rendering pipeline. A *pipeline* is responsible of rendering into a `Framebuffer`.
///
/// `L` refers to the `Layering` of the underlying `Framebuffer`.
///
/// `D` refers to the `Dim` of the underlying `Framebuffer`.
///
/// `CS` and `DS` are – respectively – the *color* and *depth* `Slot` of the underlying
/// `Framebuffer`.
pub struct Pipeline<'a, C, L, D, CS, DS>
    where C: 'a + HasFramebuffer + HasProgram + HasTessellation + HasTexture,
          L: 'a + Layerable,
          D: 'a + Dimensionable,
          D::Size: Copy,
          CS: 'a + ColorSlot<C, L, D>,
          DS: 'a + DepthSlot<C, L, D> {
  /// The embedded framebuffer.
  pub framebuffer: &'a Framebuffer<C, L, D, CS, DS>,
  /// The color used to clean the framebuffer when  executing the pipeline.
  pub clear_color: [f32; 4],
  /// Shading commands to render into the embedded framebuffer.
  pub shading_commands: Vec<&'a SomeShadingCommand> // TODO: can we use a slice instead? &'a […]
}

impl<'a, C, L, D, CS, DS> Pipeline<'a, C, L, D, CS, DS>
    where C: HasPipeline,
          L: Layerable,
          D: Dimensionable,
          D::Size: Copy,
          CS: ColorSlot<C, L, D>,
          DS: DepthSlot<C, L, D> {
  /// Create a new pipeline.
  pub fn new(framebuffer: &'a Framebuffer<C, L, D, CS, DS>, clear_color: [f32; 4], shading_commands: Vec<&'a SomeShadingCommand>) -> Self {
    Pipeline {
      framebuffer: framebuffer,
      clear_color: clear_color,
      shading_commands: shading_commands
    }
  }

  /// Run a `Pipeline`.
  pub fn run(&self) {
    C::run_pipeline(self);
  }
}

/// This trait is used to add existential quantification to `ShadingCommands`. It should be
/// implemented by backends to enable their use in `Pipeline`s.
pub trait SomeShadingCommand { // TODO: maybe we can remove that and see how to type erase ShadingCommand?
  /// Execute a shading command.
  fn run_shading_command(&self);
}

impl<'a, C, T> SomeShadingCommand for ShadingCommand<'a, C, T> where C: 'a + HasPipeline {
  fn run_shading_command(&self) {
    C::run_shading_command(self);
  }
}

/// A dynamic *shading command*. A shading command gathers *render commands* under a shader
/// `Program`.
pub struct ShadingCommand<'a, C, T> where C: 'a + HasProgram + HasTessellation, T: 'a {
  /// Embedded program.
  pub program: &'a Program<C, T>,
  /// Shader interface update function.
  ///
  /// This function is called whenever the shading command is executed, and only once per execution.
  /// You can use it to update uniforms.
  pub update: Box<Fn(&T) + 'a>,
  /// Render commands to execute for this shading command.
  pub render_commands: Vec<RenderCommand<'a, C, T>>
}

impl<'a, C, T> ShadingCommand<'a, C, T> where C: 'a + HasProgram + HasTessellation {
  /// Create a new shading command.
  pub fn new<F: Fn(&T) + 'a>(program: &'a Program<C, T>, update: F, render_commands: Vec<RenderCommand<'a, C, T>>) -> Self {
    ShadingCommand {
      program: program,
      update: Box::new(update),
      render_commands: render_commands
    }
  }
}

/// A render command, which holds information on how to rasterize tessellation.
pub struct RenderCommand<'a, C, T> where C: 'a + HasTessellation {
  /// Color blending configuration. Set to `None` if you don’t want any color blending. Set it to
  /// `Some(equation, source, destination)` if you want to perform a color blending with the
  /// `equation` formula and with the `source` and `destination` blending factors.
  pub blending: Option<(blending::Equation, blending::Factor, blending::Factor)>,
  /// Should a depth test be performed?
  pub depth_test: bool,
  /// Shader interface update function.
  ///
  /// This function is called whenever the render command is executed, and only once per execution.
  /// You can use it to update uniforms.
  pub update: Box<Fn(&T) + 'a>,
  /// The embedded tessellation.
  pub tessellation: &'a Tessellation<C>,
  /// Number of instances of the tessellation to render.
  pub instances: u32,
  /// Rasterization size for points and lines.
  pub rasterization_size: Option<f32>
}

impl<'a, C, T> RenderCommand<'a, C, T> where C: 'a + HasTessellation {
  /// Create a new render command.
  pub fn new<F: Fn(&T) + 'a>(blending: Option<(blending::Equation, blending::Factor, blending::Factor)>, depth_test: bool, update: F, tessellation: &'a Tessellation<C>, instances: u32, rasterization_size: Option<f32>) -> Self {
    RenderCommand {
      blending: blending,
      depth_test: depth_test,
      update: Box::new(update),
      tessellation: tessellation,
      instances: instances,
      rasterization_size: rasterization_size
    }
  }
}