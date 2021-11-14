use crate::renderer::wgpu_state::WgpuState;
use crate::renderer::ui_renderer::UiRenderer;

pub fn do_render_pass(
    wgpu_state: &WgpuState,
    ui_renderer: &mut UiRenderer
) {
    let possible_surface_texture = wgpu_state.surface.get_current_texture();
    let surface_texture = possible_surface_texture.unwrap();
    {
        let frame = &surface_texture.texture;
        let frame_view = &frame.create_view(&Default::default());
        let mut encoder = wgpu_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass world"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Default::default()),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }
        ui_renderer.draw_ui(frame_view, &mut encoder, wgpu_state);
        wgpu_state.queue.submit(std::iter::once(encoder.finish()));
    }
    wgpu::SurfaceTexture::present(surface_texture);
}