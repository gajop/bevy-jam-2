#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::mesh_view_bindings  view
#import bevy_pbr::utils               coords_to_viewport_uv
#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@fragment
fn fragment(
    in: MeshVertexOutput
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = coords_to_viewport_uv(in.position.xy, view.viewport);

    var output_color = textureSample(texture, our_sampler, uv);
    output_color.r *= 0.1;
    output_color.g *= 0.1;

    return output_color;
}
