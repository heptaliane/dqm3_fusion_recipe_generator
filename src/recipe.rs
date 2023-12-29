use super::data::{Monster, Parent};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub struct MonsterSpec {
    pub family: usize,
    pub rank: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub struct MonsterInfo {
    pub spec: Option<MonsterSpec>,
    pub monster_id: Option<usize>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct MonsterNode {
    pub data: MonsterInfo,
    pub children: Vec<Rc<RefCell<MonsterNode>>>,
}

struct MutableMonsterBranchIterator {
    root: Rc<RefCell<MonsterNode>>,
    indices: Vec<usize>,
    stop: bool,
}

impl MutableMonsterBranchIterator {
    fn new(root: Rc<RefCell<MonsterNode>>) -> Self {
        Self {
            root: root,
            indices: vec![],
            stop: false,
        }
    }

    fn get_branch(&self) -> Vec<Rc<RefCell<MonsterNode>>> {
        let mut branch = vec![self.root.clone()];
        let mut cursor = self.root.clone();
        for &index in self.indices.iter() {
            branch.push(cursor.borrow().children[index].clone());
            cursor = branch.last().unwrap().clone();
        }

        branch
    }
}

impl Iterator for MutableMonsterBranchIterator {
    type Item = Vec<Rc<RefCell<MonsterNode>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let branch = self.get_branch();

        // Next index
        if branch.last().unwrap().borrow().children.len() > 0 {
            self.indices.push(0);
            return Some(branch);
        }

        while let Some(index) = self.indices.pop() {
            let cursor = self.indices.len();
            if branch[cursor].borrow().children.len() > index + 1 {
                self.indices.push(index + 1);
                return Some(branch);
            }
        }
        self.stop = true;
        Some(branch)
    }
}

pub struct MonsterTreeBuilder {
    lut: HashMap<usize, Monster>,
    prefer_standard: bool,
}

impl MonsterTreeBuilder {
    pub fn new(monster_lut: HashMap<usize, Monster>) -> Self {
        Self {
            lut: monster_lut,
            prefer_standard: false,
        }
    }

    fn prefer_standard_fusion(&mut self, prefer_standard: bool) {
        self.prefer_standard = prefer_standard;
    }

    fn get_parents_info(&self, monster_id: usize) -> Vec<Vec<MonsterInfo>> {
        let monster = &self.lut[&monster_id];
        monster
            .parents
            .iter()
            .map(|ps| {
                ps.iter()
                    .map(|p| MonsterInfo {
                        spec: match p.family {
                            Some(id) => Some(MonsterSpec {
                                family: id,
                                rank: monster.rank,
                            }),
                            None => None,
                        },
                        monster_id: p.monster,
                    })
                    .collect()
            })
            .collect()
    }

    fn select_parents(&self, parents_list: Vec<Vec<MonsterInfo>>) -> Option<Vec<MonsterInfo>> {
        let count_standard_parents = |parents: &Vec<MonsterInfo>| {
            parents
                .iter()
                .filter(|info| info.spec.is_some())
                .collect::<Vec<&MonsterInfo>>()
                .len()
        };

        match self.prefer_standard {
            true => parents_list.into_iter().max_by_key(count_standard_parents),
            false => parents_list.into_iter().min_by_key(count_standard_parents),
        }
    }

    fn get_child_nodes(&self, monster_id: usize) -> Vec<Rc<RefCell<MonsterNode>>> {
        let parents_list = self.get_parents_info(monster_id);
        let parents = self.select_parents(parents_list);
        match parents {
            Some(ps) => ps
                .iter()
                .map(|data| {
                    Rc::new(RefCell::new(MonsterNode {
                        data: data.clone(),
                        children: vec![],
                    }))
                })
                .collect(),
            None => vec![],
        }
    }

    fn is_leaf_node(&self, branch: &Vec<Rc<RefCell<MonsterNode>>>) -> bool {
        let target_node = branch.last().unwrap();

        // No parents node is leaf
        match target_node.borrow().data.monster_id {
            Some(monster_id) => {
                let monster = &self.lut[&monster_id];
                if monster.parents.len() == 0 {
                    return true;
                }
            }
            None => return true,
        }

        // Cyclic branch is leaf
        let monster_ids = branch
            .iter()
            .map(|n| n.borrow().data.monster_id.unwrap())
            .collect::<HashSet<usize>>();
        let monster = &self.lut[&target_node.borrow().data.monster_id.unwrap()];
        let parent_ids = monster
            .parents
            .iter()
            .flatten()
            .filter(|p| p.monster.is_some())
            .map(|p| p.monster.unwrap())
            .collect::<HashSet<usize>>();
        if !monster_ids.is_disjoint(&parent_ids) {
            return true;
        }

        false
    }

    pub fn build(&self, monster_id: usize) -> MonsterNode {
        let root = Rc::new(RefCell::new(MonsterNode {
            data: MonsterInfo {
                monster_id: Some(monster_id),
                spec: None,
            },
            children: vec![],
        }));

        loop {
            let inner_branches: Vec<Vec<Rc<RefCell<MonsterNode>>>> =
                MutableMonsterBranchIterator::new(root.clone())
                    .filter(|b| b.last().unwrap().borrow().children.len() == 0)
                    .filter(|b| !self.is_leaf_node(&b))
                    .collect();
            for branch in inner_branches.iter() {
                let monster_id = branch.last().unwrap().borrow().data.monster_id.unwrap();
                branch.last().unwrap().try_borrow_mut().unwrap().children =
                    self.get_child_nodes(monster_id);
            }
            if inner_branches.len() == 0 {
                break;
            }
        }

        Rc::try_unwrap(root).unwrap().into_inner()
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

#[test]
fn test_mutable_monster_branch_iterator() {
    /*
     * 6 +- 4 +- 0
     *   |    +- 1
     *   |
     *   +- 5 +- 2
     *        +- 3
     */
    let leaves1 = [
        MonsterNode {
            data: MonsterInfo {
                spec: None,
                monster_id: Some(0),
            },
            children: vec![],
        },
        MonsterNode {
            data: MonsterInfo {
                spec: None,
                monster_id: Some(1),
            },
            children: vec![],
        },
    ];
    let leaves2 = [
        MonsterNode {
            data: MonsterInfo {
                spec: None,
                monster_id: Some(2),
            },
            children: vec![],
        },
        MonsterNode {
            data: MonsterInfo {
                spec: None,
                monster_id: Some(3),
            },
            children: vec![],
        },
    ];
    let middle_nodes = [
        MonsterNode {
            data: MonsterInfo {
                spec: None,
                monster_id: Some(4),
            },
            children: leaves1
                .iter()
                .map(|n| Rc::new(RefCell::new(n.clone())))
                .collect(),
        },
        MonsterNode {
            data: MonsterInfo {
                spec: None,
                monster_id: Some(5),
            },
            children: leaves2
                .iter()
                .map(|n| Rc::new(RefCell::new(n.clone())))
                .collect(),
        },
    ];
    let root = MonsterNode {
        data: MonsterInfo {
            spec: None,
            monster_id: Some(6),
        },
        children: middle_nodes
            .iter()
            .map(|n| Rc::new(RefCell::new(n.clone())))
            .collect(),
    };

    let mut itr = MutableMonsterBranchIterator::new(Rc::new(RefCell::new(root)));

    let expected_ids: Vec<Vec<usize>> = vec![
        vec![6],
        vec![6, 4],
        vec![6, 4, 0],
        vec![6, 4, 1],
        vec![6, 5],
        vec![6, 5, 2],
        vec![6, 5, 3],
    ];

    for expected in expected_ids {
        let branch = itr.next();
        assert!(branch.is_some());

        let actual = branch
            .unwrap()
            .iter()
            .map(|n| n.borrow().data.monster_id.unwrap())
            .collect::<Vec<usize>>();
        assert_eq!(actual, expected);
    }
    assert!(itr.next().is_none());
}

#[test]
fn test_get_parents_info() {
    use super::data::Parent;

    let lut: HashMap<usize, Monster> = HashMap::from([
        (
            0,
            Monster {
                name: "a".to_string(),
                rank: 0,
                family: 0,
                parents: vec![],
                habitats: HashMap::new(),
            },
        ),
        (
            1,
            Monster {
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
            },
        ),
    ]);

    let builder = MonsterTreeBuilder::new(lut);
    let actual1 = builder.get_parents_info(0);
    assert_eq!(actual1.len(), 0);

    let actual2 = builder.get_parents_info(1);
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
    let mut builder = MonsterTreeBuilder::new(HashMap::new());

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

    let parents1: Vec<Vec<MonsterInfo>> = vec![];
    let parents2 = vec![info1.clone(), info2.clone(), info3.clone()];

    let actual1f = builder.select_parents(parents1.clone());
    assert_eq!(actual1f, None);
    let actual2f = builder.select_parents(parents2.clone());
    assert!(actual2f.is_some());
    assert_eq!(actual2f.unwrap().clone(), info3.clone());

    builder.prefer_standard_fusion(true);
    let actual1t = builder.select_parents(parents1.clone());
    assert_eq!(actual1t, None);
    let actual2t = builder.select_parents(parents2.clone());
    assert!(actual2t.is_some());
    assert_eq!(actual2t.unwrap().clone(), info1.clone());
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

#[test]
fn test_build_recipe() {
    /*
     * 0 +- 1 +- 2 +- 3
     *   |    |    +- X
     *   |    |
     *   |    +- 3 +- 2
     *   |         +- X
     *   |
     *   +- 2 +- 3
     *        +- X
     */
    let lut: HashMap<usize, Monster> = HashMap::from([
        (
            0,
            Monster {
                name: "a".to_string(),
                rank: 0,
                family: 1,
                parents: vec![vec![
                    Parent {
                        monster: Some(1),
                        family: None,
                    },
                    Parent {
                        monster: Some(2),
                        family: None,
                    },
                ]],
                habitats: HashMap::new(),
            },
        ),
        (
            1,
            Monster {
                name: "a".to_string(),
                rank: 0,
                family: 0,
                parents: vec![vec![
                    Parent {
                        monster: Some(2),
                        family: None,
                    },
                    Parent {
                        monster: Some(3),
                        family: None,
                    },
                ]],
                habitats: HashMap::new(),
            },
        ),
        (
            2,
            Monster {
                name: "a".to_string(),
                rank: 0,
                family: 0,
                parents: vec![vec![
                    Parent {
                        monster: Some(3),
                        family: None,
                    },
                    Parent {
                        monster: None,
                        family: Some(0),
                    },
                ]],
                habitats: HashMap::new(),
            },
        ),
        (
            3,
            Monster {
                name: "a".to_string(),
                rank: 0,
                family: 0,
                parents: vec![vec![
                    Parent {
                        monster: Some(2),
                        family: None,
                    },
                    Parent {
                        monster: None,
                        family: Some(0),
                    },
                ]],
                habitats: HashMap::new(),
            },
        ),
    ]);

    let builder = MonsterTreeBuilder::new(lut);
    let actual = builder.build(0);

    let actual_ids = MutableMonsterBranchIterator::new(Rc::new(RefCell::new(actual)))
        .map(|b| {
            b.iter()
                .map(|n| n.borrow().data.monster_id)
                .collect::<Vec<Option<usize>>>()
        })
        .collect::<Vec<Vec<Option<usize>>>>();
    let expected_ids = vec![
        vec![Some(0)],
        vec![Some(0), Some(1)],
        vec![Some(0), Some(1), Some(2)],
        vec![Some(0), Some(1), Some(2), Some(3)],
        vec![Some(0), Some(1), Some(2), None],
        vec![Some(0), Some(1), Some(3)],
        vec![Some(0), Some(1), Some(3), Some(2)],
        vec![Some(0), Some(1), Some(3), None],
        vec![Some(0), Some(2)],
        vec![Some(0), Some(2), Some(3)],
        vec![Some(0), Some(2), None],
    ];
    assert_eq!(actual_ids, expected_ids);
}
