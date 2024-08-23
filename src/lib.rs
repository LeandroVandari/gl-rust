pub mod vertex;
use std::mem::{self, MaybeUninit};

pub const LINES_AMOUNT: usize = 5;
const LINE_LIST_LENGTH: usize = (LINES_AMOUNT * 4) + 8;

pub fn generate_line_vertices() -> [vertex::Vertex; LINE_LIST_LENGTH] {
    let mut arr: [MaybeUninit<vertex::Vertex>; LINE_LIST_LENGTH] =
        [const { MaybeUninit::uninit() }; LINE_LIST_LENGTH];
    // vertical
    for i in 0..LINES_AMOUNT {
        let x = ((i as f32 + 1.0) / (LINES_AMOUNT + 1) as f32) * 2.0 - 1.0;
        arr[i * 2].write(vertex::vertex(x, -1.0, 0));
        arr[i * 2 + 1].write(vertex::vertex(x, 1.0, 0));
    }
    //horizontal
    for i in 0..LINES_AMOUNT {
        let y = ((i as f32 + 1.0) / (LINES_AMOUNT + 1) as f32) * 2.0 - 1.0;
        arr[(LINES_AMOUNT * 2) + i * 2].write(vertex::vertex(-1.0, y, 1));
        arr[(LINES_AMOUNT * 2) + i * 2 + 1].write(vertex::vertex(1.0, y, 1));
    }

    arr[LINE_LIST_LENGTH - 8].write(vertex::vertex(-1.0, -1.0, 1));
    arr[LINE_LIST_LENGTH - 7].write(vertex::vertex(1.0, -1.0, 1));

    arr[LINE_LIST_LENGTH - 6].write(vertex::vertex(-1.0, 1.0, 1));
    arr[LINE_LIST_LENGTH - 5].write(vertex::vertex(1.0, 1.0, 1));

    arr[LINE_LIST_LENGTH - 4].write(vertex::vertex(-1.0, 1.0, 0));
    arr[LINE_LIST_LENGTH - 3].write(vertex::vertex(-1.0, -1.0, 0));

    arr[LINE_LIST_LENGTH - 2].write(vertex::vertex(1.0, 1.0, 0));
    arr[LINE_LIST_LENGTH - 1].write(vertex::vertex(1.0, -1.0, 0));

    let new_arr = unsafe {
        mem::transmute::<
            [MaybeUninit<vertex::Vertex>; LINE_LIST_LENGTH],
            [vertex::Vertex; LINE_LIST_LENGTH],
        >(arr)
    };
    new_arr
}
