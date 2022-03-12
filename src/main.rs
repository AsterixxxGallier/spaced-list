use spaced_list::{LINK_INDICES_ABOVE, LINK_LENGTH_DEGREE_INDICES};


fn main() {
	// let mut skeleton = ChunkSkeleton::<u64>::new();
	// println!("size: {}; align: {}", mem::size_of_val(&skeleton), mem::align_of_val(&skeleton));
	// skeleton.append_node(10);
	// skeleton.append_node(10);
	// skeleton.append_node(10);
	// skeleton.append_node(10);

	// for array in LINK_INDICES_ABOVE {
	// 	for x in array {
	// 		print!("{:09b} ", x)
	// 	}
	// 	println!()
	// }
	for x in LINK_LENGTH_DEGREE_INDICES {
		print!("{:09b} ", x)
	}
}
