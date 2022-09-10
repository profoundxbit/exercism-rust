use std::collections::HashMap;

#[derive(Debug, Default)]
struct TeamRecord {
    wins: u8,
    losses: u8,
    draws: u8,
}

pub fn tally(match_results: &str) -> String {
    let mut matches_map: HashMap<&str, TeamRecord> = HashMap::new();
    match_results.lines().for_each(|l| {
        let match_vec = l.split(';').take(3).collect::<Vec<&str>>();
        if let [team_one, team_two, match_result] = &match_vec[..] {
            match *match_result {
                "win" => {
                    matches_map
                        .entry(team_one)
                        .and_modify(|e| e.wins += 1)
                        .or_insert(TeamRecord {
                            wins: 1,
                            ..Default::default()
                        });
                    matches_map
                        .entry(team_two)
                        .and_modify(|e| e.losses += 1)
                        .or_insert(TeamRecord {
                            losses: 1,
                            ..Default::default()
                        });
                }
                "loss" => {
                    matches_map
                        .entry(team_one)
                        .and_modify(|e| e.losses += 1)
                        .or_insert(TeamRecord {
                            losses: 1,
                            ..Default::default()
                        });
                    matches_map
                        .entry(team_two)
                        .and_modify(|e| e.wins += 1)
                        .or_insert(TeamRecord {
                            wins: 1,
                            ..Default::default()
                        });
                }
                "draw" => {
                    matches_map
                        .entry(team_one)
                        .and_modify(|e| e.draws += 1)
                        .or_insert(TeamRecord {
                            draws: 1,
                            ..Default::default()
                        });
                    matches_map
                        .entry(team_two)
                        .and_modify(|e| e.draws += 1)
                        .or_insert(TeamRecord {
                            draws: 1,
                            ..Default::default()
                        });
                }
                _ => panic!("Matches can either be won, lossed, or end in a draw"),
            };
        }
    });

    let mut matches_vec = matches_map.iter().collect::<Vec<_>>();
    matches_vec.sort_by(|team_one, team_two| {
        let team_one_points = team_one.1.wins * 3 + team_one.1.draws;
        let team_two_points = team_two.1.wins * 3 + team_two.1.draws;
        let team_one_tuple = (team_one_points, team_one.0);
        let team_two_tuple = (team_two_points, team_two.0);
        match team_one_tuple.0.cmp(&team_two_tuple.0) {
            std::cmp::Ordering::Equal => team_two_tuple.1.cmp(team_one_tuple.1),
            result => result,
        }
    });
    matches_vec.reverse();

    let mut result_table = format!(
        "{:<30} | {:^1} | {:>2} | {:>2} | {:>2} | {:>2}",
        "Team", "MP", "W", "D", "L", "P"
    );
    for (team, record) in &matches_vec {
        let matches_played = record.wins + record.losses + record.draws;
        let points = record.wins * 3 + record.draws;
        let row = format!(
            "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team, matches_played, record.wins, record.draws, record.losses, points
        );
        result_table.push_str(&row);
    }
    result_table
}
