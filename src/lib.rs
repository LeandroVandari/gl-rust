pub mod vertex;

pub fn generate_line_vertices(lines_amount: usize) -> Vec<vertex::Vertex> {
    let list_length = (lines_amount * 4) + 8;
    let mut list = Vec::with_capacity(list_length);
    for i in 0..lines_amount {
        let x = ((i as f32 + 1.0) / (lines_amount + 1) as f32) * 2.0 - 1.0;
        let y = x;

        //vertical
        list.push(vertex::vertex(x, -1.0, 0));
        list.push(vertex::vertex(x, 1.0, 0));

        //horizontal
        list.push(vertex::vertex(-1.0, y, 1));
        list.push(vertex::vertex(1.0, y, 1));
    }

    list.extend([
        vertex::vertex(-1.0, -1.0, 1),
        vertex::vertex(1.0, -1.0, 1),
        vertex::vertex(-1.0, 1.0, 1),
        vertex::vertex(1.0, 1.0, 1),
        vertex::vertex(-1.0, 1.0, 0),
        vertex::vertex(-1.0, -1.0, 0),
        vertex::vertex(1.0, 1.0, 0),
        vertex::vertex(1.0, -1.0, 0),
    ]);
    list
}
