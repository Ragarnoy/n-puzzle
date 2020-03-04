pub mod coord;

// This function take a `&str` as input and split it on new lines.
// `start_with` is to pass the char or str we want to use as comment's start.
// This function return a Vec<String> for which each element is a non-empty line without any comment.
// e.g. the string "this is a test\n# this is a comment\nthis is another line # and another comment"
// passed through this function will gives you ["this is a test", "this is another line "]
pub fn remove_comment_by_line(input: &str, start_with: &str) -> Vec<String>
{
	input.lines().filter_map(|line| {
		let line = line.trim();
		if line.starts_with(start_with) || line.is_empty()
		{
			return None;
		}
		if let Some((index, _)) = line.match_indices(start_with).next()
		{
			return Some(line[..index].trim().into());
		}
		Some(line.into())
	}).collect()
}

pub fn snail_sort(input: &Vec<u16>, maxabs: u8) -> Vec<u16>
{
	let mut output: Vec<u16> = (0..maxabs as u16 * maxabs as u16).collect();
	let mut dir = coord::Coord {x: 1, y: 0};
	let mut cur = coord::Coord {x: 0, y: 0};

	let mut min = coord::Coord {x: 0, y: 0};
	let mut max = coord::Coord {x: maxabs as i16 - 1, y: maxabs as i16 - 1};

	for n in input.iter()
	{
		output[cur.to_abs(maxabs) as usize] = *n;
		cur.x += dir.x;
		cur.y += dir.y;

		if cur.x > max.x
		{
			dir.x = 0;
			dir.y = 1;
			min.y += 1;
			cur.x = max.x;
			cur.y += 1;
		}
		else if cur.x < min.x
		{
			dir.x = 0;
			dir.y = -1;
			max.y -= 1;
			cur.x = min.x;
			cur.y -= 1;
		}
		else if cur.y > max.y
		{
			dir.x = -1;
			dir.y = 0; 
			max.x -= 1; 
			cur.y = max.y; 
			cur.x -= 1;
		}
		else if cur.y < min.y
		{
			dir.x = 1; 
			dir.y = 0; 
			min.x += 1; 
			cur.y = min.y; 
			cur.x += 1;
		}
	}
	output
}

#[cfg(test)]
mod tests
{
	mod remove_comment_by_line
	{
		use crate::remove_comment_by_line;
		#[test]
		fn one_line_without_newline_nor_comment_empty_pattern()
		{
			let line = "this a test";
			let result = remove_comment_by_line(line, "");

			assert!(result.is_empty());
		}

		#[test]
		fn one_line_without_newline_nor_comment_sharp_pattern()
		{
			let line = "this a test";
			let expect = vec!(line);
			let result = remove_comment_by_line(line, "#");

			assert!(!result.is_empty());
			assert_eq!(result, vec!(line));
		}

		#[test]
		fn one_line_without_newline_with_comment_sharp_pattern()
		{
			let expect = vec!("this a test");
			let line = "this a test # here is a comment";
			let result = remove_comment_by_line(line, "#");

			assert!(!result.is_empty());
			assert_eq!(result, expect);
		}

		#[test]
		fn two_lines_with_one_comment_in_line_sharp_pattern()
		{
			let expect = vec!("this a test", "this is another test line");
			let line = "this a test # here is a comment\nthis is another test line";
			let result = remove_comment_by_line(line, "#");

			assert!(!result.is_empty());
			assert_eq!(result, expect);
		}

		#[test]
		fn two_lines_with_comment_start_line_sharp_pattern()
		{
			let expect = vec!("this a test");
			let line = "this a test\n   	# here is a comment";
			let result = remove_comment_by_line(line, "#");

			assert!(!result.is_empty());
			assert_eq!(result, expect);
		}

		#[test]
		fn three_lines_with_comment_start_line_and_mid_line_sharp_pattern()
		{
			let expect = vec!("this a test", "this is another line");
			let line = "this a test\n   	# here is a comment\nthis is another line # with another comment";
			let result = remove_comment_by_line(line, "#");

			assert!(!result.is_empty());
			assert_eq!(result, expect);
		}
	}

	#[test]
	fn snail_three_by_three()
	{
		use crate::snail_sort;
		let mut test_vec: Vec<u16> = (1..9).collect();
		test_vec.push(0);
		assert_eq!(snail_sort(&test_vec, 3), vec![1, 2, 3, 8, 0, 4, 7, 6, 5]);
	}

	#[test]
	fn snail_four_by_four()
	{
		use crate::snail_sort;
		let mut test_vec: Vec<u16> = (1..16).collect();
		test_vec.push(0);
		assert_eq!(snail_sort(&test_vec, 4), vec![1, 2, 3, 4, 12, 13, 14, 5, 11, 0, 15, 6, 10, 9, 8, 7]);
	}

	#[test]
	fn snail_five_by_five()
	{
		use crate::snail_sort;
		let mut test_vec: Vec<u16> = (1..25).collect();
		test_vec.push(0);
		assert_eq!(snail_sort(&test_vec, 5), vec![1, 2, 3, 4, 5,16, 17, 18, 19, 6, 15, 24, 0, 20, 7, 14, 23, 22, 21, 8, 13, 12, 11, 10, 9]);
	}
}
