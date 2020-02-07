#[derive(Eq, PartialEq, Debug)]
pub struct Coord
{
    pub x: i16,
    pub y: i16
}

impl Coord
{
    pub fn new(x: i16, y: i16) -> Self
    {
        Self
        {
            x,
            y
        }
    }

    pub fn from_abs(abs: u32, col: u8) -> Self
    {
        let y = abs / col as u32;
        let x = abs % col as u32;

        Self::new(x as i16, y as i16)
    }

    pub fn to_abs(&self, col: u8) -> u32
    {
        self.y as u32 * col as u32 + self.x as u32
    }

    pub fn inc_x(&mut self)
    {
        self.x += 1;
    }

    pub fn inc_y(&mut self)
    {
        self.y += 1;
    }

    pub fn dec_x(&mut self)
    {
        self.x -= 1;
    }

    pub fn dec_y(&mut self)
    {
        self.y -= 1;
    }

    pub fn is_out_of_table(&self, col: u8) -> bool
    {
        if self.x < 0 || self.x >= col as i16 || self.y < 0 || self.y >= col as i16
        {
            true
        }
        else
        {
            false
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn new()
    {
        let expected = Coord { x: 3, y: 2 };
        let x = 3;
        let y = 2;

        let result = Coord::new(x, y);

        assert_eq!(result, expected);
    }

    #[test]
    fn from_abs()
    {
        let expected0 = Coord { x: 0, y: 2 };
        let expected1 = Coord { x: 3, y: 2 };
        let expected2 = Coord { x: 2, y: 5 };

        let result0 = Coord::from_abs(6, 3);
        let result1 = Coord::from_abs(11, 4);
        let result2 = Coord::from_abs(42, 8);

        assert_eq!(result0, expected0);
        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
    }

    #[test]
    fn to_abs()
    {
        let expected0 = 6;
        let expected1 = 11;
        let expected2 = 42;

        let result0 = Coord { x: 0, y: 2 }.to_abs(3);
        let result1 = Coord { x: 3, y: 2 }.to_abs(4);
        let result2 = Coord { x: 2, y: 5 }.to_abs(8);

        assert_eq!(result0, expected0);
        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
    }

    #[test]
    fn inc_x()
    {
        let expected = Coord { x: 1, y: 0 };

        let mut result = Coord { x: 0, y: 0 };
        result.inc_x();

        assert_eq!(result, expected);
    }

    #[test]
    fn inc_y()
    {
        let expected = Coord { x: 0, y: 1 };

        let mut result = Coord { x: 0, y: 0 };
        result.inc_y();

        assert_eq!(result, expected);
    }

    #[test]
    fn dec_x()
    {
        let expected = Coord { x: 0, y: 0 };

        let mut result = Coord { x: 1, y: 0 };
        result.dec_x();

        assert_eq!(result, expected);
    }

    #[test]
    fn dec_y()
    {
        let expected = Coord { x: 0, y: 0 };

        let mut result = Coord { x: 0, y: 1 };
        result.dec_y();

        assert_eq!(result, expected);
    }

    #[test]
    fn dec_x_on_x_zero()
    {
        let expected = Coord { x: -1, y: 0 };

        let mut result = Coord { x: 0, y: 0 };
        result.dec_x();

        assert_eq!(result, expected);
    }

    #[test]
    fn dec_y_on_y_zero()
    {
        let expected = Coord { x: 0, y: -1 };

        let mut result = Coord { x: 0, y: 0 };
        result.dec_y();

        assert_eq!(result, expected);
    }

    #[test]
    fn is_out_of_table_because_of_y_neg()
    {
        let expected = true;

        let result = Coord { x: 0, y: -1}.is_out_of_table(3);

        assert_eq!(result, expected);
    }

    #[test]
    fn is_out_of_table_because_of_x_neg()
    {
        let expected = true;

        let result = Coord { x: -1, y: 0}.is_out_of_table(3);

        assert_eq!(result, expected);
    }

    #[test]
    fn is_out_of_table_because_of_x_and_y_neg()
    {
        let expected = true;

        let result = Coord { x: -1, y: -1}.is_out_of_table(3);

        assert_eq!(result, expected);
    }

    #[test]
    fn is_out_of_table_because_of_x_supp_col()
    {
        let expected = true;

        let result = Coord { x: 3, y: 0}.is_out_of_table(3);

        assert_eq!(result, expected);
    }

    #[test]
    fn is_out_of_table_because_of_y_supp_col()
    {
        let expected = true;

        let result = Coord { x: 0, y: 3}.is_out_of_table(3);

        assert_eq!(result, expected);
    }

    #[test]
    fn is_out_of_table_false()
    {
        let expected = false;

        let result = Coord { x: 2, y: 2}.is_out_of_table(3);

        assert_eq!(result, expected);
    }
}