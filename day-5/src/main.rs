// Advent of Code - Day 5: Print Queue

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Update {
    pages: Vec<u32>,
}

impl FromIterator<u32> for Update {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        Update {
            pages: iter.into_iter().collect(),
        }
    }
}

impl Update {
    /// return middle page number
    fn middle_page(&self) -> &u32 {
        self.pages.get(self.pages.len() / 2).unwrap()
    }

    /// check if pages are sorted according to rules
    fn is_sorted(&self, rules: &[&Rule]) -> bool {
        rules.iter().all(|rule| rule.check(self))
    }

    /// apply the ordering of a rule to an update
    fn apply_rule(&mut self, rule: &Rule) {
        if let (Some(before_idx), Some(after_idx)) = (
            self.pages.iter().position(|&p| p == rule.before),
            self.pages.iter().position(|&p| p == rule.after),
        ) {
            let page = self.pages.remove(before_idx);
            self.pages.insert(after_idx, page);
        }
    }

    /// repeatedly apply rules that are vialoted untill update is correctly sorted
    fn fix(&mut self, rules: &[Rule]) {
        // filter rules that do not apply
        let rules = rules
            .iter()
            .filter(|rule| self.pages.contains(&rule.before) && self.pages.contains(&rule.after))
            .collect::<Vec<&Rule>>();

        // apply violating rules until update is sorted according to all rules
        while !self.is_sorted(&rules) {
            for rule in &rules {
                if !rule.check(self) {
                    self.apply_rule(rule);
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

impl Rule {
    fn new(before: u32, after: u32) -> Self {
        Self { before, after }
    }

    fn check(&self, update: &Update) -> bool {
        if update.pages.contains(&self.before) && update.pages.contains(&self.after) {
            let before_idx = update.pages.iter().position(|&page| page == self.before);
            let after_idx = update.pages.iter().position(|&page| page == self.after);
            return before_idx < after_idx;
        };
        true
    }
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let (rules, updates) = input
        .split_once("\n\n")
        .map(|(part1, part2)| {
            let rules = part1
                .lines()
                .map(|l| {
                    l.split_once('|')
                        .map(|(l, r)| {
                            let before = l.parse::<u32>().unwrap();
                            let after = r.parse::<u32>().unwrap();
                            Rule::new(before, after)
                        })
                        .unwrap()
                })
                .collect::<Vec<_>>();
            let updates = part2
                .lines()
                .map(|l| {
                    l.split(',')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Update>()
                })
                .collect::<Vec<_>>();
            (rules, updates)
        })
        .unwrap();
    (rules, updates)
}

fn solve_part_one(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);
    updates
        .iter()
        // filter all correct updates
        .filter(|update| rules.iter().all(|rule| rule.check(update)))
        // sum of all middle pages for each correct update
        .map(|update| update.middle_page())
        .sum::<u32>()
}

fn solve_part_two(input: &str) -> u32 {
    let (rules, mut updates) = parse_input(input);
    // find all incorrect updates
    let mut incorrect_updates: Vec<&mut Update> = updates
        .iter_mut()
        .filter(|update| rules.iter().any(|rule| !rule.check(update)))
        .collect();
    // fix each incorrect update
    incorrect_updates.iter_mut().for_each(|update| {
        update.fix(&rules);
    });
    incorrect_updates
        .iter()
        .map(|update| update.middle_page())
        .sum()
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one:\n{part_one_answer}");
    let part_two_answer = solve_part_two(INPUT);
    println!("Part two:\n{part_two_answer}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 143);
    }

    #[test]
    fn part2() {
        let example_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 123);
    }
}
