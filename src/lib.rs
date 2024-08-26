use std::mem::{self, MaybeUninit};

use vertex::vertex;

pub mod vertex;

pub fn generate_line_vertices(lines_amount: usize) -> Vec<vertex::Vertex> {
    let list_length = (lines_amount * 4) + 8;
    let mut list = Vec::with_capacity(list_length);
    for i in 0..lines_amount {
        let x = ((i as f32 + 1.0) / (lines_amount + 1) as f32) * 2.0 - 1.0;
        let y = x;

        //vertical
        list.push(vertex(x, -1.0, 0));
        list.push(vertex(x, 1.0, 0));

        //horizontal
        list.push(vertex(-1.0, y, 1));
        list.push(vertex(1.0, y, 1));
    }

    // list.extend([
    //     vertex(-1.0, -1.0, 1),
    //     vertex(1.0, -1.0, 1),
    //     vertex(-1.0, 1.0, 1),
    //     vertex(1.0, 1.0, 1),
    //     vertex(-1.0, 1.0, 0),
    //     vertex(-1.0, -1.0, 0),
    //     vertex(1.0, 1.0, 0),
    //     vertex(1.0, -1.0, 0),
    // ]);

    list
}

pub fn create_line_vertices_at_coord(coord: f32) -> [vertex::Vertex; 4] {
    let mut arr = [MaybeUninit::uninit(); 4];

    arr[0].write(vertex(coord, -1.0, 0));
    arr[1].write(vertex(coord, 1.0, 0));

    arr[2].write(vertex(-1.0, coord, 1));
    arr[3].write(vertex(1.0, coord, 1));

    unsafe { mem::transmute::<_, [vertex::Vertex; 4]>(arr) }
}
