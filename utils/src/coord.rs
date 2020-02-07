#[derive(Eq, PartialEq, Debug)]
pub struct Coord
{
    x: u16,
    y: u16
}

impl Coord
{
    pub fn new(x: u16, y: u16) -> Self
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

        Self::new(x as u16, y as u16)
    }

    pub fn to_abs(&self, col: u8) -> u32
    {
        self.y as u32 * col as u32 + self.x as u32
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
}