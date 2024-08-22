pub mod vertex;
use std::mem::{self, MaybeUninit};

pub fn generate_line_vertices() -> [vertex::Vertex; 40]{
    let mut arr: [MaybeUninit<vertex::Vertex>; 40]  = [const { MaybeUninit::uninit() }; 40];
    const LINES_AMOUNT: usize = 5;
    // vertical
    for i in 0..LINES_AMOUNT {
        let x = (((i as f32 + 1.0 ) / (LINES_AMOUNT+1) as f32)*2.0-1.0) as f32;
        arr[i*2].write(vertex::vertex(x, -1.0));
        arr[i*2+1].write(vertex::vertex(x, 1.0));
    }
    //horizontal
    for i in 0..LINES_AMOUNT {
        let y = (((i as f32 + 1.0 ) / (LINES_AMOUNT+1) as f32)*2.0-1.0) as f32;
        arr[20+i*2].write(vertex::vertex(-1.0, y));
        arr[20+i*2+1].write(vertex::vertex(1.0, y));
    }

    unsafe { mem::transmute::<_, [vertex::Vertex; 40]>(arr)}

}
