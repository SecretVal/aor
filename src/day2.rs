use crate::shared::*;

type Report = Vec<usize>;
    pub(crate) fn is_save(report: &Report) -> bool {
        let mut save = true;
        let mut inc = false;
        let mut dec = false;

        for i in 0..report.len()-1 {
            let current = report[i];
            let next = report[i + 1];
            if i == 0 {
                if current > next {
                    dec = true;
                } else {
                    inc = true;
                }
            }
            if current < next && dec {
                save = false;
                break;
            } else if current > next && inc {
                save = false;
                break;
            } else if diff(current, next) < 1 || diff(current, next) > 3 {
                save = false;
                break;
            }
        }
        return save;
    }

#[derive(Debug)]
pub struct Day2 {
    reports: Vec<Report>
}

impl Day for Day2 {
    type Res = usize;
    fn parse_input(str: String) -> Self {
        let mut d2 = Day2 {
            reports: vec![],
        };
        let reports: Vec<_> = str.split("\n").collect();
        for report in reports {
            let nums: Vec<usize> = report
                                   .split_whitespace()
                                   .map(|x| x.parse::<usize>().expect("Invalid input"))
                                   .collect();

            if nums.len() > 0 {
                d2.reports.push(nums);
            }
        }
        return d2;
    }

    fn solve_part1(&mut self) -> Self::Res {
        let mut save_reports: Self::Res = 0;
        for report in &self.reports {
            if is_save(report) {
                save_reports += 1;
            }
        }
        return save_reports;
    }

    fn solve_part2(&mut self) -> Self::Res {
        let mut save_reports: Self::Res = 0;
        for report in &self.reports {
            if is_save(report) {
                save_reports += 1;
            } else {
                for i in 0..report.len() {
                    let mut new_rep = report.clone();
                    new_rep.remove(i);
                    if is_save(&new_rep) {
                        save_reports += 1;
                        break;
                    }
                }
            }
        }
        return save_reports;
    }

}


