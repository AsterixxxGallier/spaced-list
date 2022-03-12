#![feature(trait_alias)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use num_traits::{CheckedAdd, CheckedSub, Zero};

pub trait Spacing = Zero + CheckedAdd + CheckedSub + Ord + Copy;

// region constants

pub const MAX_CHUNK_DEPTH: usize = 8;
pub const CHUNK_INDEX_MASK: usize = 0xFF;
pub const MAX_CHUNK_SIZE: usize = 256;
pub const LINK_LENGTHS_SIZE: usize = 511;
pub const LINK_INDICES_ABOVE: [[usize; MAX_CHUNK_DEPTH]; MAX_CHUNK_SIZE] = generate_link_indices_above();
pub const LINK_LENGTH_DEGREE_INDICES: [usize; MAX_CHUNK_DEPTH] = generate_link_length_degree_indices();
// pub const LINK_LENGTH_NODE_INDICES: [usize; MAX_CHUNK_SIZE] = generate_link_length_node_indices();

// const fn generate_link_length_node_indices() -> [usize; MAX_CHUNK_SIZE] {
	// let mut indices = [0usize; MAX_CHUNK_SIZE];
	// let mut node_index = 0usize;
	// let mut link_index = 0usize;
	// while node_index < MAX_CHUNK_SIZE {
	// 	indices[node_index] = link_index;
	// 	link_index += number_of_links(node_index as u8) as usize;
	// 	node_index += 1
	// }
	// indices
// }

// pub const fn link_index(node_index: usize, degree: usize) -> usize {
// 	LINK_LENGTH_DEGREE_INDICES[node_index] + degree
// }

const fn generate_link_length_degree_indices() -> [usize; MAX_CHUNK_DEPTH] {
	let mut indices = [0usize; MAX_CHUNK_DEPTH];
	let mut link_index = 0usize;
	let mut degree = 0usize;
	while degree < MAX_CHUNK_DEPTH {
		indices[degree] = link_index;
		link_index += MAX_CHUNK_SIZE >> degree;
		degree += 1
	}
	indices
}

pub const fn link_index(node_index: usize, degree: usize) -> usize {
	LINK_LENGTH_DEGREE_INDICES[degree] + node_index
}

const fn generate_link_indices_above() -> [[usize; MAX_CHUNK_DEPTH]; MAX_CHUNK_SIZE] {
	let mut indices = [[0usize; MAX_CHUNK_DEPTH]; MAX_CHUNK_SIZE];
	let mut node_index = 1usize;
	while node_index < MAX_CHUNK_SIZE {
		let mut index = node_index;
		let mut degree = 0usize;
		while degree < MAX_CHUNK_DEPTH {
			if degree == 0 {
				index -= 1
			}
			indices[node_index][degree] = link_index(index, degree);
			index &= !(1 << degree);
			degree += 1
		}
		node_index += 1
	}
	indices
}

const fn number_of_links(index: u8) -> u32 {
	index.trailing_zeros() + 1
}

// endregion

pub struct ChunkSublists<'a, D: Spacing> {
	/// The indices pointing to elements in the sublists vector, where the index in the array
	/// corresponds to the node index the sublist is *before*. If the sublist index is greater than
	/// or equal to the size of the sublists vector, it is to be understood as there not being a
	/// sublist at that position. Therefore, the values of this array will be initialised as 255
	/// (the maximum u8 value), so until the sublists vector is full, the unchanged array values
	/// will not be valid indices for the sublists vector.
	sublist_indices: [u8; MAX_CHUNK_SIZE],
	pub sublists: Vec<&'a dyn SpacedList<D>>,
}

impl<'a, D: Spacing> Default for ChunkSublists<'a, D> {
	fn default() -> Self {
		ChunkSublists {
			sublist_indices: [255; MAX_CHUNK_SIZE],
			sublists: Vec::new()
		}
	}
}

impl<'a, D: Spacing> ChunkSublists<'a, D> {
	pub fn new() -> Self {
		Self::default()
	}
}

pub trait SpacedList<D> {
	fn append_node(&mut self, distance: D);

	fn node_at(&self, position: D) -> Option<Vec<usize>>;
}

pub struct SpacedListSkeleton<'a, D: Spacing> {
	pub size: usize,
	pub total_length: D,
	pub offset: D,
	pub levels: Vec<Vec<ChunkSkeleton<D>>>,
	pub sublists: Vec<ChunkSublists<'a, D>>
}

impl<D: Spacing> Default for SpacedListSkeleton<'_, D> {
	fn default() -> Self {
		SpacedListSkeleton {
			size: 0,
			total_length: Zero::zero(),
			offset: Zero::zero(),
			levels: Vec::new(),
			sublists: Vec::new()
		}
	}
}

impl<D: Spacing> SpacedListSkeleton<'_, D> {
	pub fn new() -> Self {
		Default::default()
	}

	fn top_chunk(&self) -> Option<&ChunkSkeleton<D>> {
		self.levels.last()?.last()
	}

	fn make_space(&mut self, level: usize, distance: D) {
		if self.size == 0 {
			let chunk = ChunkSkeleton::<D>::new();
			self.levels.push(vec![chunk]);
			return;
		}
		if level == self.levels.len() {
			let mut new_top = ChunkSkeleton::<D>::new();
			let old_top = self.top_chunk().unwrap();
			// TODO store old top somehow maybe possibly (append_element)? dunno rn
			new_top.append_node(num_traits::zero());
			// let old_top = self.top_chunk();
			// match old_top {
			// 	None => {}
			// 	Some(it) => { new_top.append_node(num_traits::zero()) }
			// }
			self.levels.push(vec![new_top]);
			return;
		}
		let last = self.levels[level].last().unwrap();
		let last_total_length = last.total_length;
		if last.size == MAX_CHUNK_SIZE {
			self.make_space(level + 1, num_traits::zero());
			let last_above = self.levels[level + 1].last_mut().unwrap();
			let new_last = ChunkSkeleton::<D>::new();
			// TODO store new last somehow maybe possibly (append_element)? dunno rn
			last_above.append_node(last_total_length + distance);
			self.levels[level].push(new_last);
		}
	}
}

impl<D: Spacing> SpacedList<D> for SpacedListSkeleton<'_, D> {
	fn append_node(&mut self, distance: D) {
		self.make_space(0, distance);
		if self.size == 0 {
			self.offset = self.offset + distance
		}
		self.levels[0].last_mut().unwrap().append_node(distance);
		self.size += 1;
		self.total_length = self.total_length + distance;
	}

	fn node_at(&self, position: D) -> Option<Vec<usize>> {
		if self.size == 0 || position > self.total_length || position < self.offset {
			return None
		}

		let mut current_index = 0usize;
		let mut current_position: D = num_traits::zero();
		let mut degree = self.levels.len() * MAX_CHUNK_DEPTH - 1;
		let mut level = self.levels.len() - 1;
		// TODO set this somewhere somehow
		let mut position_before_level_0 = num_traits::zero();
		loop {
			let chunks = &self.levels[level];
			let to_next_index = 1usize << level;
			let next_index = current_index + to_next_index;
			if next_index < self.size {
				let chunk = &chunks[current_index >> (level * MAX_CHUNK_DEPTH)];
				let level_degree = level * MAX_CHUNK_DEPTH;
				let local_degree = degree % MAX_CHUNK_DEPTH;
				let local_index = current_index & (CHUNK_INDEX_MASK << level_degree) >> level_degree;
				let to_next = chunk.link_lengths[link_index(local_index, local_degree)];
				let next_position = current_position + to_next;
				if next_position == position {
					return Some(vec![current_index])
				}
				if next_position > position && degree == 0 {
					let sublist_index = current_index + 1;
					let sublists = &self.sublists[sublist_index >> MAX_CHUNK_DEPTH];
					let local_sublist_index = sublists.sublist_indices[sublist_index & CHUNK_INDEX_MASK];
					let sublist = sublists.sublists.get(local_sublist_index as usize)?;
					return sublist.node_at(position - position_before_level_0)
				}
				if next_position < position {
					current_position = next_position;
					current_index = next_index;
				}
			}
			level -= 1
		}
	}
}

pub struct ChunkSkeleton<D: Spacing> {
	pub link_lengths: [D; LINK_LENGTHS_SIZE],
	pub total_length: D,
	pub size: usize,
}

impl<D: Spacing> Default for ChunkSkeleton<D> {
	fn default() -> Self {
		Self {
			size: 0,
			total_length: num_traits::zero(),
			link_lengths: [num_traits::zero(); LINK_LENGTHS_SIZE],
		}
	}
}

impl<D: Spacing> ChunkSkeleton<D> {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn append_node(&mut self, distance: D) {
		if self.size == 0 {
			self.size = 1;
			return;
		}
		let link_indices = LINK_INDICES_ABOVE[self.size];
		for index in link_indices {
			self.link_lengths[index] = self.link_lengths[index] + distance
		}
		self.size += 1;
		self.total_length = self.total_length + distance
	}
}

#[cfg(test)]
mod tests {
	use crate::{link_index, LINK_INDICES_ABOVE, LINK_LENGTH_DEGREE_INDICES, number_of_links};

	#[test]
	fn test_link_index() {
		assert_eq!(link_index(0, 0), 0);
		assert_eq!(link_index(0, 1), 1);
		assert_eq!(link_index(0, 2), 2);
		assert_eq!(link_index(0, 3), 3);
		assert_eq!(link_index(0, 4), 4);
		assert_eq!(link_index(0, 5), 5);
		assert_eq!(link_index(0, 6), 6);
		assert_eq!(link_index(0, 7), 7);
		assert_eq!(link_index(0, 8), 8);
		assert_eq!(link_index(1, 0), 9);
		assert_eq!(link_index(2, 0), 10);
		assert_eq!(link_index(2, 1), 11);
		assert_eq!(link_index(3, 0), 12);
		assert_eq!(link_index(4, 0), 13);
		assert_eq!(link_index(4, 1), 14);
		assert_eq!(link_index(4, 2), 15);
		assert_eq!(link_index(5, 0), 16);
	}

	#[test]
	fn test_link_indices_above() {
		assert_eq!(LINK_INDICES_ABOVE[0], [0; 8]);
		assert_eq!(LINK_INDICES_ABOVE[1], [0, 1, 2, 3, 4, 5, 6, 7]);
		assert_eq!(LINK_INDICES_ABOVE[2], [9, 1, 2, 3, 4, 5, 6, 7]);
		assert_eq!(LINK_INDICES_ABOVE[3], [10, 11, 2, 3, 4, 5, 6, 7]);
		assert_eq!(LINK_INDICES_ABOVE[4], [12, 11, 2, 3, 4, 5, 6, 7]);
	}

	#[test]
	fn test_link_length_node_indices() {
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[0], 0);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[1], 9);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[2], 10);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[3], 12);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[4], 13);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[5], 16);
	}

	#[test]
	fn test_number_of_links() {
		assert_eq!(number_of_links(0), 9);
		assert_eq!(number_of_links(1), 1);
		assert_eq!(number_of_links(2), 2);
		assert_eq!(number_of_links(3), 1);
		assert_eq!(number_of_links(4), 3);
		assert_eq!(number_of_links(128), 8);
		assert_eq!(number_of_links(255), 1);
	}
}
