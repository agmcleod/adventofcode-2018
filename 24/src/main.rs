use std::collections::{HashMap, HashSet};
use std::cmp::{self, Ordering};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum DamageType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Team {
    Infection,
    Immune,
}

#[derive(Clone, Debug)]
struct Group {
    id: usize,
    units: usize,
    hit_points: usize,
    attack_damage: usize,
    attack_type: DamageType,
    weaknesses: HashSet<DamageType>,
    immunities: HashSet<DamageType>,
    initiative: usize,
    team: Team,
}

impl Group {
    fn new(id: usize, units: usize, hit_points: usize, attack_damage: usize, attack_type: DamageType, weaknesses_list: Vec<DamageType>, immunities_list: Vec<DamageType>, initiative: usize, team: Team) -> Self {
        let mut weaknesses = HashSet::new();
        for weakness in &weaknesses_list {
            weaknesses.insert(weakness.to_owned());
        }

        let mut immunities = HashSet::new();
        for immunity in &immunities_list {
            immunities.insert(immunity.to_owned());
        }

        Group{
            id,
            units,
            hit_points,
            attack_damage,
            attack_type,
            weaknesses,
            immunities,
            initiative,
            team,
        }
    }
}

fn main() {
    // let mut groups = vec![
    //     Group::new(1, 17, 5390, 4507, DamageType::Fire, vec![DamageType::Bludgeoning, DamageType::Radiation], Vec::new(), 2, Team::Immune),
    //     Group::new(2, 989, 1274, 25, DamageType::Slashing, vec![DamageType::Bludgeoning, DamageType::Slashing], vec![DamageType::Fire], 3, Team::Immune),
    //     Group::new(3, 801, 4706, 116, DamageType::Bludgeoning, vec![DamageType::Radiation], Vec::new(), 1, Team::Infection),
    //     Group::new(4, 4485, 2961, 12, DamageType::Slashing, vec![DamageType::Fire, DamageType::Cold], vec![DamageType::Radiation], 4, Team::Infection),
    // ];

    let mut groups = vec![
        Group::new(1, 2987, 5418, 17, DamageType::Cold, vec![DamageType::Cold, DamageType::Bludgeoning], vec![DamageType::Slashing], 5, Team::Immune),
        Group::new(2, 1980, 9978, 47, DamageType::Cold, vec![], vec![DamageType::Cold], 19, Team::Immune),
        Group::new(3, 648, 10733, 143, DamageType::Fire, vec![], vec![DamageType::Radiation, DamageType::Fire, DamageType::Slashing], 9, Team::Immune),
        Group::new(4, 949, 3117, 29, DamageType::Fire, vec![], vec![], 10, Team::Immune),
        Group::new(5, 5776, 5102, 8, DamageType::Radiation, vec![DamageType::Cold], vec![DamageType::Slashing], 15, Team::Immune),
        Group::new(6, 1265, 4218, 24, DamageType::Radiation, vec![], vec![DamageType::Radiation], 16, Team::Immune),
        Group::new(7, 3088, 10066, 28, DamageType::Slashing, vec![DamageType::Slashing], vec![], 1, Team::Immune),
        Group::new(8, 498, 1599, 28, DamageType::Bludgeoning, vec![DamageType::Radiation], vec![DamageType::Bludgeoning], 11, Team::Immune),
        Group::new(9, 3705, 10764, 23, DamageType::Cold, vec![], vec![], 7, Team::Immune),
        Group::new(10, 3431, 3666, 8, DamageType::Bludgeoning, vec![DamageType::Slashing], vec![DamageType::Bludgeoning], 8, Team::Immune),

        Group::new(11, 2835, 33751, 21, DamageType::Bludgeoning, vec![DamageType::Cold], vec![], 13, Team::Infection),
        Group::new(12, 4808, 32371, 11, DamageType::Cold, vec![DamageType::Radiation], vec![DamageType::Bludgeoning], 14, Team::Infection),
        Group::new(13, 659, 30577, 88, DamageType::Slashing, vec![DamageType::Fire], vec![DamageType::Radiation], 12, Team::Infection),
        Group::new(14, 5193, 40730, 14, DamageType::Cold, vec![DamageType::Slashing], vec![DamageType::Radiation, DamageType::Fire, DamageType::Bludgeoning], 20, Team::Infection),
        Group::new(15, 1209, 44700, 71, DamageType::Fire, vec![DamageType::Bludgeoning, DamageType::Radiation], vec![], 18, Team::Infection),
        Group::new(16, 6206, 51781, 13, DamageType::Fire, vec![], vec![DamageType::Cold], 4, Team::Infection),
        Group::new(17, 602, 22125, 73, DamageType::Cold, vec![DamageType::Radiation, DamageType::Bludgeoning], vec![], 3, Team::Infection),
        Group::new(18, 5519, 37123, 12, DamageType::Radiation, vec![DamageType::Slashing, DamageType::Fire], vec![], 2, Team::Infection),
        Group::new(19, 336, 23329, 134, DamageType::Cold, vec![DamageType::Fire], vec![DamageType::Cold, DamageType::Bludgeoning, DamageType::Radiation], 17, Team::Infection),
        Group::new(20, 2017, 50511, 42, DamageType::Fire, vec![], vec![DamageType::Bludgeoning], 6, Team::Infection),
    ];

    loop {
        groups.sort_by(|a, b| {
            match (b.units * b.attack_damage).cmp(&(a.units * a.attack_damage)) {
                Ordering::Equal => b.initiative.cmp(&a.initiative),
                n => n,
            }
        });

        let mut attackers_to_defenders = HashMap::new();
        let mut chosen_targets = HashSet::new();

        for group in &groups {
            let mut best_id = 0;
            let mut best_damage = 0;
            let mut best_initiative = 0;
            let mut best_effective_power = std::usize::MAX;
            for target in &groups {
                if target.id == group.id || target.team == group.team || target.immunities.contains(&group.attack_type) || chosen_targets.contains(&target.id) {
                    continue
                }

                let mut damage = group.attack_damage * group.units;
                if target.weaknesses.contains(&group.attack_type) {
                    damage *= 2;
                }

                if damage > best_damage {
                    best_damage = damage;
                    best_id = target.id;
                    best_initiative = target.initiative;
                    best_effective_power = target.units * target.attack_damage;
                } else if damage == best_damage {
                    if target.units * target.attack_damage > best_effective_power {
                        best_id = target.id;
                        best_initiative = target.initiative;
                        best_effective_power = target.units * target.attack_damage;
                    } else if target.units * target.attack_damage == best_effective_power {
                        if target.initiative > best_initiative {
                            best_id = target.id;
                            best_initiative = target.initiative;
                        }
                    }
                }
            }

            if best_id > 0 {
                attackers_to_defenders.insert(group.id, best_id);
                chosen_targets.insert(best_id);
            }
        }

        groups.sort_by(|a, b| b.initiative.cmp(&a.initiative));

        let attacking_ids: Vec<usize> = groups.iter().map(|g| g.id).collect();
        for attacker in &attacking_ids {
            if !attackers_to_defenders.contains_key(&attacker) {
                continue
            }

            let target = attackers_to_defenders.get(&attacker).unwrap();

            let attacker: Group = groups.iter().filter(|g| g.id == *attacker).nth(0).unwrap().to_owned();

            if attacker.units == 0 {
                continue
            }

            let target: &mut Group = groups.iter_mut().filter(|g| g.id == *target).nth(0).unwrap();

            let mut damage = attacker.attack_damage * attacker.units;
            if target.weaknesses.contains(&attacker.attack_type) {
                damage *= 2;
            }

            // println!("Damage {} killing {} dealt by {} to {}", damage, cmp::min(damage / target.hit_points, target.units), attacker.id, target.id);
            target.units -= cmp::min(damage / target.hit_points, target.units);
        }

        groups.retain(|group| group.units > 0);

        let mut last_team = None;
        let mut found_two = false;
        for group in &groups {
            if last_team.is_none() {
                last_team = Some(group.team);
            } else if last_team.unwrap() != group.team {
                found_two = true;
            }
        }

        if !found_two {
            println!("Part one: {}", groups.iter().fold(0, |sum, group| sum + group.units));
            break
        }
    }
}
