struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@binding(0) @group(0) var<uniform> camera: CameraUniform;

struct Output {
    @builtin(position) Position :vec4<f32>,
    @location(0) vColor : vec4<f32>,
};


@vertex
fn vs_main(@location(0) pos: vec4<f32>, @location(1) color: vec4<f32>) -> Output {
    var output: Output;
    output.Position = camera.view_proj * pos; 
    output.vColor = color ;
    return output;
}

@fragment
fn fs_main(@location(0) vColor: vec4<f32> ) -> @location(0) vec4<f32> {
    return vColor ;
}