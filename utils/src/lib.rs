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
}
