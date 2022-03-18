#![feature(trait_alias)]
#![feature(int_log)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]


use std::marker::PhantomData;

use num_traits::{CheckedAdd, CheckedSub, Zero};

pub trait Spacing = Zero + CheckedAdd + CheckedSub + Ord + Copy;

// region constants

pub const MAX_CHUNK_DEPTH: usize = 9;
pub const CHUNK_INDEX_MASK: usize = 0xFF;
pub const MAX_CHUNK_SIZE: usize = 256;
pub const LINK_LENGTHS_SIZE: usize = 511;
pub const LINK_INDICES_ABOVE: [[usize; MAX_CHUNK_DEPTH]; MAX_CHUNK_SIZE] = generate_link_indices_above();
pub const LINK_LENGTH_DEGREE_INDICES: [usize; MAX_CHUNK_DEPTH] = generate_link_length_degree_indices();


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
	LINK_LENGTH_DEGREE_INDICES[degree] + (node_index >> degree)
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

pub const fn number_of_links(index: u8) -> usize {
	(index.trailing_zeros() + 1) as usize
}

// endregion

pub struct SublistsChunk<S: SpacedList<D>, D: Spacing> {
	/// The indices pointing to elements in the sublists vector, where the index in the array
	/// corresponds to the node index the sublist is *before*. If the sublist index is greater than
	/// or equal to the size of the sublists vector, it is to be understood as there not being a
	/// sublist at that position. Therefore, the values of this array will be initialised as 255
	/// (the maximum u8 value), so until the sublists vector is full, the unchanged array values
	/// will not be valid indices for the sublists vector.
	sublist_indices: [u8; MAX_CHUNK_SIZE],
	pub sublists: Vec<S>,
	phantom: PhantomData<D>,
}

impl<S: SpacedList<D>, D: Spacing> Default for SublistsChunk<S, D> {
	fn default() -> Self {
		SublistsChunk {
			sublist_indices: [255; MAX_CHUNK_SIZE],
			sublists: Vec::new(),
			phantom: PhantomData,
		}
	}
}

impl<S: SpacedList<D>, D: Spacing> SublistsChunk<S, D> {
	pub fn new() -> Self {
		Self::default()
	}

	fn add_sublist(&mut self, index: u8) -> &mut S {
		self.sublist_indices[index as usize] = self.sublists.len() as u8;
		self.sublists.push(Default::default());
		// SAFETY: the sublists vector must have a last element, because we just appended it and
		// nobody could have removed it in the meantime, because we own the vector.
		unsafe { self.sublists.last_mut().unwrap_unchecked() }
	}
}

pub trait SpacedList<D>: Default {
	fn append_node(&mut self, distance: D);

	fn node_at(&self, position: D) -> Option<Vec<usize>>;

	fn node_before(&self, position: D) -> Option<(Vec<usize>, D)>;

	fn node_after(&self, position: D) -> Option<(Vec<usize>, D)>;
}

pub struct SpacedListSkeleton<D: Spacing> {
	pub size: usize,
	pub total_length: D,
	pub offset: D,
	pub levels: Vec<Vec<ChunkSkeleton<D>>>,
	pub sublists: Vec<SublistsChunk<SpacedListSkeleton<D>, D>>,
}

impl<D: Spacing> Default for SpacedListSkeleton<D> {
	fn default() -> Self {
		SpacedListSkeleton {
			size: 0,
			total_length: Zero::zero(),
			offset: Zero::zero(),
			levels: Vec::new(),
			sublists: Vec::new(),
		}
	}
}

impl<D: Spacing> SpacedListSkeleton<D> {
	pub fn new() -> Self {
		Default::default()
	}

	fn make_space(&mut self, level: usize, distance: D) {
		if self.size == 0 {
			let chunk = ChunkSkeleton::<D>::new();
			self.levels.push(vec![chunk]);
			self.sublists.push(Default::default());
			return;
		}
		if level == self.levels.len() {
			let mut new_top = ChunkSkeleton::<D>::new();
			new_top.append_node(num_traits::zero());
			self.levels.push(vec![new_top]);
			return;
		}
		let last = self.levels[level].last().unwrap();
		let last_total_length = last.total_length;
		if last.size == MAX_CHUNK_SIZE {
			self.make_space(level + 1, num_traits::zero());
			let last_above = self.levels[level + 1].last_mut().unwrap();
			let new_last = ChunkSkeleton::<D>::new();
			last_above.append_node(last_total_length + distance);
			self.levels[level].push(new_last);
			if level == 0 {
				self.sublists.push(Default::default());
			}
		}
	}

	fn make_sublist_space(&mut self, sublist_index: usize) -> &mut Self {
		let sublist_chunk_index = sublist_index >> 8;
		let index_in_sublist_chunk = sublist_index & CHUNK_INDEX_MASK;
		let sublist_chunk = &mut self.sublists[sublist_chunk_index];
		let chunk_local_sublist_index = sublist_chunk.sublist_indices[index_in_sublist_chunk];
		if index_in_sublist_chunk >= sublist_chunk.sublists.len() {
			return sublist_chunk.add_sublist(index_in_sublist_chunk as u8);
		}
		&mut sublist_chunk.sublists[chunk_local_sublist_index as usize]
	}
}

impl<D: Spacing> SpacedList<D> for SpacedListSkeleton<D> {
	fn append_node(&mut self, distance: D) {
		self.make_space(0, distance);
		if self.size == 0 {
			self.offset = self.offset + distance
		}
		self.levels[0].last_mut().unwrap().append_node(distance);
		self.size += 1;
		self.total_length = self.total_length + distance;
	}

	/// returns the full index of the first node at the given position,
	/// or None if there is no node at that position
	fn node_at(&self, position: D) -> Option<Vec<usize>> {
		// region handle edge cases
		if self.size == 0 || position > self.total_length || position < self.offset {
			return None;
		}
		if position == self.offset {
			return Some(vec![0]);
		}
		// endregion

		// region subtract offset from position
		let position_without_offset = position - self.offset;
		// endregion

		// region define variables about the current state of the traversal
		let mut current_index = 0usize;
		let mut current_position: D = num_traits::zero();
		let mut degree = self.size.log2() as usize;
		let mut level = self.levels.len() - 1;
		// endregion

		loop {
			// region define variables for this step
			let chunks = &self.levels[level];
			let to_next_index = 1usize << degree;
			let next_index = current_index + to_next_index;
			// endregion

			if next_index < self.size {
				// region define variables relevant to the local chunk
				let chunk = &chunks[current_index >> (level * MAX_CHUNK_DEPTH)];
				let level_degree = level * MAX_CHUNK_DEPTH;
				let local_degree = degree % MAX_CHUNK_DEPTH;
				let local_index = current_index & (CHUNK_INDEX_MASK << level_degree) >> level_degree;
				// endregion

				// region find next position
				let to_next = chunk.link_lengths[link_index(local_index, local_degree)];
				let next_position = current_position + to_next;
				// endregion

				// region return if node found
				if next_position == position_without_offset {
					return Some(vec![next_index]);
				}
				// endregion

				// region go into the sublist if one exists
				if next_position > position_without_offset && degree == 0 {
					// region find sublist
					let sublist_index = next_index;
					let sublist_chunk_index = sublist_index >> 8;
					let index_in_sublist_chunk = sublist_index & CHUNK_INDEX_MASK;
					let sublist_chunk = &self.sublists[sublist_chunk_index];
					let sublist_index = sublist_chunk.sublist_indices[index_in_sublist_chunk];
					let sublist = sublist_chunk.sublists.get(sublist_index as usize);
					// endregion

					// region seek result in sublist
					return {
						let mut vec = vec![current_index];
						vec.append(&mut sublist?.node_at(position_without_offset - current_position)?);
						Some(vec)
					};
					// endregion
				}
				// endregion

				// region move ahead
				if next_position < position_without_offset {
					current_position = next_position;
					current_index = next_index;
				}
				// endregion
			}

			// region return None if unsuccessful at lowest level
			if degree == 0 {
				return None;
			}
			// endregion

			// region move below
			if degree % CHUNK_INDEX_MASK == 0 {
				level -= 1;
			}
			degree -= 1;
			// endregion
		}
	}

	/// returns the full index of the last node before the given position (and the distance from
	/// that node to the position), or None if there is no node at that position
	fn node_before(&self, position: D) -> Option<(Vec<usize>, D)> {
		// region handle edge cases
		if self.size == 0 || position <= self.offset {
			return None;
		}
		// TODO handle edge case where if position > self.totalLength, the last node is returned
		// endregion

		// region subtract offset from position
		let position_without_offset = position - self.offset;
		// endregion

		// region define variables about the current state of the traversal
		let mut current_index = 0usize;
		let mut current_position: D = num_traits::zero();
		let mut degree = self.size.log2() as usize;
		let mut level = self.levels.len() - 1;
		// endregion

		loop {
			// region define variables for this step
			let chunks = &self.levels[level];
			let to_next_index = 1usize << degree;
			let next_index = current_index + to_next_index;
			// endregion

			if next_index < self.size {
				// region define variables relevant to the local chunk
				let chunk = &chunks[current_index >> (level * MAX_CHUNK_DEPTH)];
				let level_degree = level * MAX_CHUNK_DEPTH;
				let local_degree = degree % MAX_CHUNK_DEPTH;
				let local_index = current_index & (CHUNK_INDEX_MASK << level_degree) >> level_degree;
				// endregion

				// region find next position
				let to_next = chunk.link_lengths[link_index(local_index, local_degree)];
				let next_position = current_position + to_next;
				// endregion

				if next_position >= position_without_offset/* && degree == 0*/ {
					// the current position is just before the target position

					// region find sublist
					let sublist_index = next_index;
					let sublist_chunk_index = sublist_index >> 8;
					let index_in_sublist_chunk = sublist_index & CHUNK_INDEX_MASK;
					let sublist_chunk = &self.sublists[sublist_chunk_index];
					let sublist_index = sublist_chunk.sublist_indices[index_in_sublist_chunk];
					let sublist = sublist_chunk.sublists.get(sublist_index as usize);
					// endregion
					return match sublist {
						Some(sublist) if degree == 0 => {
							// TODO handle case where the sublist is empty
							// TODO double check that the last node of the sublist is definitely not contained in sub-sublists
							Some((vec![current_index, sublist.size - 1], position_without_offset - (current_position + sublist.total_length)))
						}
						_ => {
							Some((vec![current_index], position_without_offset - current_position))
						}
					}
				}

				// region move ahead
				if next_position < position_without_offset {
					current_position = next_position;
					current_index = next_index;
				}
				// endregion
			}

			// region return None if unsuccessful at lowest level
			if degree == 0 {
				return None;
			}
			// endregion

			// region move below
			if degree % CHUNK_INDEX_MASK == 0 {
				level -= 1;
			}
			degree -= 1;
			// endregion
		}
	}

	fn node_after(&self, _position: D) -> Option<(Vec<usize>, D)> {
		todo!()
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
	use crate::{link_index, LINK_INDICES_ABOVE, LINK_LENGTH_DEGREE_INDICES, number_of_links, SpacedList, SpacedListSkeleton};

	#[test]
	fn test_link_index() {
		assert_eq!(link_index(0, 0), 0);
		assert_eq!(link_index(1, 0), 1);
		assert_eq!(link_index(2, 0), 2);
		assert_eq!(link_index(3, 0), 3);
		assert_eq!(link_index(0, 1), 256);
		assert_eq!(link_index(1, 1), 256);
		assert_eq!(link_index(2, 1), 256 + 1);
		assert_eq!(link_index(0, 2), 256 + 128);
		assert_eq!(link_index(1, 2), 256 + 128);
		assert_eq!(link_index(2, 2), 256 + 128);
		assert_eq!(link_index(3, 2), 256 + 128);
		assert_eq!(link_index(4, 2), 256 + 128 + 1);
		assert_eq!(link_index(5, 2), 256 + 128 + 1);
		assert_eq!(link_index(6, 2), 256 + 128 + 1);
		assert_eq!(link_index(7, 2), 256 + 128 + 1);
		assert_eq!(link_index(8, 2), 256 + 128 + 2);
		assert_eq!(link_index(0, 3), 256 + 128 + 64);
		assert_eq!(link_index(0, 4), 256 + 128 + 64 + 32);
		assert_eq!(link_index(0, 5), 256 + 128 + 64 + 32 + 16);
		assert_eq!(link_index(0, 6), 256 + 128 + 64 + 32 + 16 + 8);
		assert_eq!(link_index(0, 7), 256 + 128 + 64 + 32 + 16 + 8 + 4);
		assert_eq!(link_index(0, 8), 256 + 128 + 64 + 32 + 16 + 8 + 4 + 2);
	}

	#[test]
	fn test_link_indices_above() {
		assert_eq!(LINK_INDICES_ABOVE[0], [0; 9]);
		assert_eq!(LINK_INDICES_ABOVE[1], [
			link_index(0, 0),
			link_index(0, 1),
			link_index(0, 2),
			link_index(0, 3),
			link_index(0, 4),
			link_index(0, 5),
			link_index(0, 6),
			link_index(0, 7),
			link_index(0, 8),
		]);
		assert_eq!(LINK_INDICES_ABOVE[2], [
			link_index(1, 0),
			link_index(0, 1),
			link_index(0, 2),
			link_index(0, 3),
			link_index(0, 4),
			link_index(0, 5),
			link_index(0, 6),
			link_index(0, 7),
			link_index(0, 8),
		]);
		assert_eq!(LINK_INDICES_ABOVE[3], [
			link_index(2, 0),
			link_index(2, 1),
			link_index(0, 2),
			link_index(0, 3),
			link_index(0, 4),
			link_index(0, 5),
			link_index(0, 6),
			link_index(0, 7),
			link_index(0, 8),
		]);
		assert_eq!(LINK_INDICES_ABOVE[4], [
			link_index(3, 0),
			link_index(2, 1),
			link_index(2, 2),
			link_index(0, 3),
			link_index(0, 4),
			link_index(0, 5),
			link_index(0, 6),
			link_index(0, 7),
			link_index(0, 8),
		]);
		assert_eq!(LINK_INDICES_ABOVE[5], [
			link_index(4, 0),
			link_index(4, 1),
			link_index(4, 2),
			link_index(1, 3),
			link_index(0, 4),
			link_index(0, 5),
			link_index(0, 6),
			link_index(0, 7),
			link_index(0, 8),
		]);
	}

	#[test]
	fn test_link_length_degree_indices() {
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[0], 0);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[1], 256);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[2], 256 + 128);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[3], 256 + 128 + 64);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[4], 256 + 128 + 64 + 32);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[5], 256 + 128 + 64 + 32 + 16);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[6], 256 + 128 + 64 + 32 + 16 + 8);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[7], 256 + 128 + 64 + 32 + 16 + 8 + 4);
		assert_eq!(LINK_LENGTH_DEGREE_INDICES[8], 256 + 128 + 64 + 32 + 16 + 8 + 4 + 2);
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

	#[test]
	fn test_append_node_and_node_at() {
		let mut list = SpacedListSkeleton::new();
		list.append_node(1);
		assert_eq!(list.offset, 1);
		assert_eq!(list.levels[0][0].link_lengths, [0; 511]);

		list.append_node(2);
		assert_eq!(list.offset, 1);
		assert_eq!(list.levels[0][0].link_lengths[link_index(0, 0)], 2);

		list.append_node(3);
		assert_eq!(list.offset, 1);
		assert_eq!(list.levels[0][0].link_lengths[link_index(0, 0)], 2);
		assert_eq!(list.levels[0][0].link_lengths[link_index(1, 0)], 3);
		assert_eq!(list.levels[0][0].link_lengths[link_index(0, 1)], 5);
		assert_eq!(list.levels[0][0].link_lengths[link_index(0, 2)], 5);

		assert_eq!(list.node_at(0), None);
		assert_eq!(list.node_at(1), Some(vec![0]));
		assert_eq!(list.node_at(2), None);
		assert_eq!(list.node_at(3), Some(vec![1]));
	}

	#[test]
	fn test_append_node_and_node_at_with_sublists() {
		let mut list = SpacedListSkeleton::new();
		list.append_node(1);
		assert_eq!(list.offset, 1);
		assert_eq!(list.levels[0][0].link_lengths, [0; 511]);

		list.append_node(4);
		assert_eq!(list.offset, 1);
		assert_eq!(list.levels[0][0].link_lengths[link_index(0, 0)], 4);

		let sublist = list.make_sublist_space(1);

		sublist.append_node(1);
		sublist.append_node(2);

		assert_eq!(list.node_at(0), None);
		assert_eq!(list.node_at(1), Some(vec![0]));
		assert_eq!(list.node_at(2), Some(vec![0, 0]));
		assert_eq!(list.node_at(3), None);
		assert_eq!(list.node_at(4), Some(vec![0, 1]));
		assert_eq!(list.node_at(5), Some(vec![1]));
		assert_eq!(list.node_at(6), None);
	}

	#[test]
	fn test_append_node_and_node_before() {
		let mut list = SpacedListSkeleton::new();
		list.append_node(1);
		assert_eq!(list.offset, 1);
		assert_eq!(list.levels[0][0].link_lengths, [0; 511]);

		list.append_node(2);
		assert_eq!(list.offset, 1);
		assert_eq!(list.levels[0][0].link_lengths[link_index(0, 0)], 2);

		list.append_node(3);
		assert_eq!(list.offset, 1);
		assert_eq!(list.levels[0][0].link_lengths[link_index(0, 0)], 2);
		assert_eq!(list.levels[0][0].link_lengths[link_index(1, 0)], 3);
		assert_eq!(list.levels[0][0].link_lengths[link_index(0, 1)], 5);
		assert_eq!(list.levels[0][0].link_lengths[link_index(0, 2)], 5);

		assert_eq!(list.node_before(0), None);
		assert_eq!(list.node_before(1), None);
		assert_eq!(list.node_before(2), Some((vec![0], 1)));
		assert_eq!(list.node_before(3), Some((vec![0], 2)));
		assert_eq!(list.node_before(4), Some((vec![1], 1)));
	}
}
