pub const SHADER: &str = r#"
    precision mediump float;

    
    attribute vec2 a_texCoord;
    varying vec2 v_texCoord;
    attribute vec4 aVertexPosition;

    uniform mat4 uModelViewMatrix;
    uniform mat4 uProjectionMatrix;
    
    void main() {

        // pass the texCoord to the fragment shader
        // The GPU will interpolate this value between points
        v_texCoord = a_texCoord;
        gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
    }
"#;