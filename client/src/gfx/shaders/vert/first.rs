pub const SHADER: &str = r#"
    attribute vec4 a_vertexPosition;
    attribute vec2 a_texCoord;
    
    varying vec2 v_texCoord;

    uniform mat4 u_mMatrix;
    uniform mat4 u_vpMatrix;

    void main() {
        gl_Position = a_vertexPosition * u_mMatrix;
        v_texCoord = a_texCoord;
        // gl_Position = u_ProjectionMatrix * u_ModelViewMatrix * a_VertexPosition;
    }
"#;