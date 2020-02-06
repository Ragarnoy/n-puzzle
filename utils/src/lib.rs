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
			return Some(line[..index].into());
		}
		Some(line.into())
	}).collect()
}

#[cfg(test)]
mod tests {
	use crate::*;

    #[test]
    fn one_line_without_newline_nor_comment() {
		let line = "this a test";
		let result = remove_comment_by_line(line, "");

		assert!(!result.is_empty());
		assert_eq!(result, vec!(line));
    }
}
