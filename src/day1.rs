use crate::shared::Day;

pub struct Day1 {
    pub(crate) left: Vec<usize>,
    pub(crate) right: Vec<usize>,
}

impl Day for Day1 {
    type Res = usize;
    fn parse_input(str: String) -> Self {
        let mut d1 = Self{
            left: vec![],
            right: vec![],
        };
        let lines: Vec<&str> = str.split("\n").collect();
        for line in lines {
            let mut nums: Vec<usize> = line.split("   ")
                .collect::<Vec<&str>>()
                .into_iter()
                .filter(|x| !x.is_empty())
                .map(|str| str.parse::<usize>().expect("wrong input"))
                .collect();
            if nums.len() == 2 {
                d1.left.push(nums[0]);
                d1.right.push(nums[1]);
            }
        }
        d1.left.sort();
        d1.right.sort();
        return d1;
    }

    fn solve(&mut self) -> Self::Res {
        assert_eq!(self.left.len(), self.right.len());
        let len = self.left.len();
        let mut sum: usize = 0;
        for i in 0..len {
            let left = self.left[i];
            let right = self.right[i];
            if left > right {
                sum += left - right;
            } else {
                sum += right - left;
            }
        }
        return sum;
    }
}
