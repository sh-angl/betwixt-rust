pub const SHADER: &str = r#"
    precision mediump float;
    
    // our texture
    uniform sampler2D u_image;
    uniform vec4 u_tint;
    
    // the texCoords passed in from the vertex shader.
    varying vec2 v_texCoord;

    
    void main() {
        // Look up a color from the texture.
        gl_FragColor = texture2D(u_image, v_texCoord) * u_tint;
    }
"#;