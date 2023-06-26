use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

pub fn tally(match_results: &str) -> String {
    let mut teams = score_teams(parse_matches(match_results));
    let mut output = String::from("Team                           | MP |  W |  D |  L |  P");

    while let Some(team) = teams.pop() {
        output.push_str(&format!(
            "\n{:31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            team.name, team.matches_played, team.wins, team.draws, team.losses, team.points
        ));
    }

    output
}

fn score_teams(teams: HashSet<Team>) -> BinaryHeap<Team> {
    teams
        .into_iter()
        .map(|mut t| {
            t.compute_score();
            t
        })
        .collect()
}

fn parse_matches(match_results: &str) -> HashSet<Team> {
    let mut teams: HashSet<Team> = HashSet::new();
    for ((mut t1, mut t2), r) in match_results.lines().flat_map(|line| {
        let mut match_iter = line.split(';');
        let team1 = match_iter.next().map(Team::from);
        let team2 = match_iter.next().map(Team::from);
        let result = match_iter
            .next()
            .and_then(|s| MatchResult::try_from(s).ok());
        team1.zip(team2).zip(result)
    }) {
        t1.matches_played += 1;
        t2.matches_played += 1;

        match r {
            MatchResult::Win => {
                t1.wins += 1;
                t2.losses += 1;
            }
            MatchResult::Loss => {
                t1.losses += 1;
                t2.wins += 1;
            }
            MatchResult::Draw => {
                t1.draws += 1;
                t2.draws += 1;
            }
        }

        if let Some(t) = teams.take(&t1) {
            teams.insert(t + t1);
        } else {
            teams.insert(t1);
        }

        if let Some(t) = teams.take(&t2) {
            teams.insert(t + t2);
        } else {
            teams.insert(t2);
        }
    }

    teams
}

#[derive(Default, Debug)]
struct Team {
    name: String,
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl Hash for Team {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl std::ops::Add for Team {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.losses += rhs.losses;
        self.wins += rhs.wins;
        self.draws += rhs.draws;
        self.matches_played += rhs.matches_played;
        self
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.points.eq(&other.points) && self.name.eq(&other.name)
    }
}

impl Eq for Team {}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.points
            .cmp(&other.points)
            .then(other.name.cmp(&self.name))
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Team {
    fn compute_score(&mut self) {
        self.points = (self.wins * 3) + self.draws;
    }
}

enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl From<&str> for Team {
    fn from(value: &str) -> Self {
        Self {
            name: value.to_string(),
            ..Default::default()
        }
    }
}

impl TryFrom<&str> for MatchResult {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        Ok(match value {
            "win" => Self::Win,
            "loss" => Self::Loss,
            "draw" => Self::Draw,
            _ => return Err(()),
        })
    }
}
