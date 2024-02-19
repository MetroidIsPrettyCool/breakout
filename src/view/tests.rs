use crate::view::Vertex;

#[test]
fn quad_scaling() {
    let q = super::quad(2.0, 2.0, [0.0, 0.0, 0.0]);
    assert_eq!(
        q,
        [
            Vertex {
                position: [1.0, 1.0, 0.0],
                color: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                color: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                color: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                color: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                color: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                color: [0.0, 0.0, 0.0],
            },
        ]
    );
}
