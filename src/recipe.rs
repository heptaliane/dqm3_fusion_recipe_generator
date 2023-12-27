use super::data::{Monster, Parent};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
pub enum Limitation {
    Rank([Option<usize>; 2]),
    Scoutable(bool),
    Standard(bool),
    HideScouted(bool),
}

#[derive(PartialEq, Clone, Debug)]
pub struct MonsterSpec {
    family: usize,
    rank: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub struct MonsterInfo {
    spec: Option<MonsterSpec>,
    monster_id: Option<usize>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct MonsterNode {
    monster_id: usize,
    children: Vec<MonsterNode>,
}

fn get_parents_info(monster: &Monster) -> Vec<Vec<MonsterInfo>> {
    monster
        .parents
        .iter()
        .map(|parents| {
            parents
                .iter()
                .map(|parent| MonsterInfo {
                    spec: match parent.family {
                        Some(id) => Some(MonsterSpec {
                            family: id,
                            rank: monster.rank,
                        }),
                        None => None,
                    },
                    monster_id: parent.monster,
                })
                .collect()
        })
        .collect()
}

fn select_parents(
    parents_list: &Vec<Vec<MonsterInfo>>,
    prefer_standard: bool,
) -> Option<&Vec<MonsterInfo>> {
    let count_standard_parents = |parents: &&Vec<MonsterInfo>| {
        parents
            .iter()
            .filter(|info| info.spec.is_some())
            .collect::<Vec<&MonsterInfo>>()
            .len()
    };

    match prefer_standard {
        true => parents_list.iter().max_by_key(count_standard_parents),
        false => parents_list.iter().min_by_key(count_standard_parents),
    }
}

fn validate_monster_rank(monster: &Monster, rank_range: &[Option<usize>; 2]) -> bool {
    match rank_range {
        [Some(min), _] if min > &monster.rank => false,
        [_, Some(max)] if max < &monster.rank => false,
        _ => true,
    }
}

fn is_scoutable(monster: &Monster) -> bool {
    monster.habitats.len() > 0
}

// pub fn create_recipe_tree(
//     monster_id: usize,
//     monster_lut: &HashMap<usize, Monster>,
//     priority: Vec<Limitation>,
// ) -> MonsterNode {
// }

#[test]
fn test_get_parents_info() {
    use super::data::Parent;

    let monster1 = Monster {
        name: "a".to_string(),
        rank: 0,
        family: 0,
        parents: vec![],
        habitats: HashMap::new(),
    };
    let actual1 = get_parents_info(&monster1);
    assert_eq!(actual1.len(), 0);

    let monster2 = Monster {
        name: "a".to_string(),
        rank: 0,
        family: 1,
        parents: vec![
            vec![
                Parent {
                    monster: None,
                    family: Some(2),
                },
                Parent {
                    monster: None,
                    family: Some(3),
                },
            ],
            vec![
                Parent {
                    monster: Some(4),
                    family: None,
                },
                Parent {
                    monster: Some(5),
                    family: None,
                },
            ],
        ],
        habitats: HashMap::new(),
    };
    let actual2 = get_parents_info(&monster2);
    assert_eq!(actual2.len(), 2);
    assert_eq!(actual2[0].len(), 2);
    assert_eq!(
        actual2[0][0],
        MonsterInfo {
            spec: Some(MonsterSpec { rank: 0, family: 2 }),
            monster_id: None
        }
    );
    assert_eq!(
        actual2[0][1],
        MonsterInfo {
            spec: Some(MonsterSpec { rank: 0, family: 3 }),
            monster_id: None
        }
    );
}

#[test]
fn test_select_parents() {
    let parents1: Vec<Vec<MonsterInfo>> = vec![];
    let actual1t = select_parents(&parents1, true);
    assert_eq!(actual1t, None);
    let actual1f = select_parents(&parents1, false);
    assert_eq!(actual1f, None);

    let info1 = vec![
        MonsterInfo {
            spec: Some(MonsterSpec { family: 0, rank: 1 }),
            monster_id: None,
        },
        MonsterInfo {
            spec: Some(MonsterSpec { family: 2, rank: 1 }),
            monster_id: None,
        },
    ];
    let info2 = vec![
        MonsterInfo {
            spec: Some(MonsterSpec { family: 0, rank: 1 }),
            monster_id: None,
        },
        MonsterInfo {
            spec: None,
            monster_id: Some(3),
        },
    ];
    let info3 = vec![
        MonsterInfo {
            spec: None,
            monster_id: Some(3),
        },
        MonsterInfo {
            spec: None,
            monster_id: Some(4),
        },
    ];
    let parents2 = vec![info1.clone(), info2.clone(), info3.clone()];
    let actual2t = select_parents(&parents2, true);
    assert!(actual2t.is_some());
    assert_eq!(actual2t.unwrap().clone(), info1.clone());
    let actual2f = select_parents(&parents2, false);
    assert!(actual2f.is_some());
    assert_eq!(actual2f.unwrap().clone(), info3.clone());
}

#[test]
fn test_validate_monster_rank() {
    let monster = Monster {
        name: "a".to_string(),
        rank: 3,
        family: 0,
        parents: vec![],
        habitats: HashMap::new(),
    };
    assert_eq!(validate_monster_rank(&monster, &[Some(0), Some(2)]), false);
    assert_eq!(validate_monster_rank(&monster, &[Some(0), Some(3)]), true);
    assert_eq!(validate_monster_rank(&monster, &[Some(0), Some(7)]), true);
    assert_eq!(validate_monster_rank(&monster, &[Some(3), Some(7)]), true);
    assert_eq!(validate_monster_rank(&monster, &[Some(4), Some(7)]), false);
    assert_eq!(validate_monster_rank(&monster, &[None, Some(2)]), false);
    assert_eq!(validate_monster_rank(&monster, &[None, Some(3)]), true);
    assert_eq!(validate_monster_rank(&monster, &[None, Some(4)]), true);
    assert_eq!(validate_monster_rank(&monster, &[Some(2), None]), true);
    assert_eq!(validate_monster_rank(&monster, &[Some(3), None]), true);
    assert_eq!(validate_monster_rank(&monster, &[Some(4), None]), false);
    assert_eq!(validate_monster_rank(&monster, &[None, None]), true);
}

#[test]
fn test_is_scoutable() {
    let monster1 = Monster {
        name: "a".to_string(),
        rank: 0,
        family: 0,
        parents: vec![],
        habitats: HashMap::new(),
    };
    assert_eq!(is_scoutable(&monster1), false);

    use super::data::AreaCondition;
    let monster2 = Monster {
        name: "a".to_string(),
        rank: 0,
        family: 0,
        parents: vec![],
        habitats: vec![(
            0,
            AreaCondition {
                conditions: vec![(0, vec![true, false])].into_iter().collect(),
            },
        )]
        .into_iter()
        .collect(),
    };
    assert_eq!(is_scoutable(&monster2), true);
}
