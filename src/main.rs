use std::mem;
use spaced_list::{LINK_INDICES_ABOVE, number_of_links};


fn main() {
	// let mut skeleton = ChunkSkeleton::<u64>::new();
	// println!("size: {}; align: {}", mem::size_of_val(&skeleton), mem::align_of_val(&skeleton));
	// skeleton.append_node(10);
	// skeleton.append_node(10);
	// skeleton.append_node(10);
	// skeleton.append_node(10);

	for (index, array) in LINK_INDICES_ABOVE.iter().enumerate() {
		print!("{:08b}: ", index);
		for x in &array[..number_of_links(index as u8)] {
			print!("{:09b} ", x)
		}
		println!()
	}
}
