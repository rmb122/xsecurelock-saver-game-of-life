use std::fmt;

/*
https://conwaylife.com/wiki/Run_Length_Encoded

<tag>	description
b	dead cell
o	alive cell
$	end of line

online editor:
https://conwaylife.com/

example:
#N 46P4H1V0
#C The smallest known c/4 orthogonal spaceship. Has period 4.
#C www.conwaylife.com/wiki/index.php?title=46P4H1V0
x = 19, y = 10, rule = b3/s23
3bo11bo3b$3bo11bo3b$2bobo9bobo2b2$bo3bo7bo3bob$bob6ob6obob$o7bobo7bo$o
7bobo7bo$o17bo$bob2ob2o3b2ob2obo!
*/

#[derive(Debug, Clone)]
pub struct LifeRLEParseError {
    msg: String,
}

impl LifeRLEParseError {
    fn new(msg: &str) -> LifeRLEParseError {
        return LifeRLEParseError {
            msg: msg.to_owned(),
        };
    }
}

impl fmt::Display for LifeRLEParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.msg.is_empty() {
            write!(f, "rle parse error")
        } else {
            write!(f, "rle parse error, {}", self.msg)
        }
    }
}

#[derive(Debug)]
pub struct LifeRLEParser<'a> {
    rle: &'a str,

    width: u32,
    height: u32,
    base_x: u32,
    base_y: u32,
}

impl<'a> LifeRLEParser<'a> {
    pub fn new(rle: &str) -> LifeRLEParser {
        return LifeRLEParser {
            rle,
            width: 0,
            height: 0,
            base_x: 0,
            base_y: 0,
        };
    }

    fn parse_rle_header_line(&mut self, line: &str) -> Result<(), LifeRLEParseError> {
        for segment in line
            .split(",")
            .map(|s| s.splitn(2, "=").map(|s| s.trim()).collect::<Vec<&str>>())
        {
            if segment.len() == 2 {
                match segment[0] {
                    "x" => {
                        self.width = segment[1]
                            .parse()
                            .map_err(|_| LifeRLEParseError::new("invalid width found"))?
                    }
                    "y" => {
                        self.height = segment[1]
                            .parse()
                            .map_err(|_| LifeRLEParseError::new("invalid height found"))?
                    }
                    "base_x" => {
                        self.base_x = segment[1]
                            .parse()
                            .map_err(|_| LifeRLEParseError::new("invalid base x found"))?
                    }
                    "base_y" => {
                        self.base_y = segment[1]
                            .parse()
                            .map_err(|_| LifeRLEParseError::new("invalid base y found"))?
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn parse_rle<F>(&mut self, mut alive_callback: F) -> Result<(), LifeRLEParseError>
    where
        F: FnMut(u32, u32),
    {
        let mut iter = self
            .rle
            .split("\n")
            .map(|l| l.trim()) // 去掉前后的空格
            .filter(|l| !l.is_empty() && !l.starts_with("#"));

        let header_line = iter.next();
        if let Some(header_line) = header_line {
            self.parse_rle_header_line(header_line)?
        } else {
            Err(LifeRLEParseError::new("empty rle found"))?
        };

        let mut curr_length = 0;
        let transform_curr_length = |length| {
            if length == 0 {
                return 1;
            } else {
                return length;
            }
        };
        let mut curr_x: u32 = 0;
        let mut curr_y: u32 = 0;

        'outer: for line in iter
        // 不为空且不是 # 开头
        {
            for char in line.chars() {
                match char {
                    _ if char >= '0' && char <= '9' => {
                        curr_length *= 10;
                        curr_length += char as u32 - '0' as u32;
                    }
                    'b' => {
                        curr_x += transform_curr_length(curr_length);
                        curr_length = 0;
                    }
                    'o' => {
                        for _ in 0..transform_curr_length(curr_length) {
                            alive_callback(self.base_x + curr_x, self.base_y + curr_y);
                            curr_x += 1;
                        }
                        curr_length = 0;
                    }
                    '$' => {
                        curr_x = 0;
                        curr_y += transform_curr_length(curr_length);
                        curr_length = 0;
                    }
                    '!' => {
                        if curr_length != 0 {
                            Err(LifeRLEParseError::new("invalid length found"))?
                        }
                        break 'outer;
                    }
                    _ => Err(LifeRLEParseError::new(&format!(
                        "invalid char `{}` found",
                        char
                    )))?,
                }
            }
        }

        Ok(())
    }
}

#[test]
fn parse_rle_test() {
    let rle = r###"
    #N 46P4H1V0
#C The smallest known c/4 orthogonal spaceship. Has period 4.
#C www.conwaylife.com/wiki/index.php?title=46P4H1V0
x = 19, y = 10, rule = b3/s23, base_x = 1, base_y = 0
3bo11bo3b$3bo11bo3b$2bobo9bobo2b2$bo3bo7bo3bob$bob6ob6obob$o7bobo7bo$o
7bobo7bo$o17bo$bob2ob2o3b2ob2obo!
"###;
    let mut parser = LifeRLEParser::new(rle);

    println!(
        "{:?}",
        parser.parse_rle(|x, y| {
            println!("x: {}, y: {}", x, y);
        })
    );
}
