pub const SHADER: &str = r#"
    attribute vec2 a_texCoord;
    varying vec2 v_texCoord;
    
    void main() {

    // pass the texCoord to the fragment shader
    // The GPU will interpolate this value between points
    v_texCoord = a_texCoord;
    }
"#;