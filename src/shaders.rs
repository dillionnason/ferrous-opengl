pub const vertex_shader_src = r#"
    #version 150

    in vec3 position;
    in vec4 color;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    out vec4 v_color;

    void main() {
        v_color = color;
        mat4 modelview = view * model;
        gl_Position = perspective * modelview * vec4(position, 1.0);
    }
"#;

pub const fragment_shader_src = r#"
    #version 140

    in vec4 v_color;

    out vec4 color;

    void main() {
        color = v_color;
    }
"#;
