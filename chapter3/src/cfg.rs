use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Terminal {
    Name(String),
    Epsilon,
}
impl ToString for Terminal {
    fn to_string(&self) -> String {
        match self {
            Terminal::Name(n) => n.clone(),
            Terminal::Epsilon => "".to_string(),
        }
    }
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Symbol {
    Terminal(Terminal),
    Nonterminal(String),
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Terminal(n) => write!(f, "{}", n.to_string()),
            Symbol::Nonterminal(nt) => write!(f, "{}", nt),
        }
    }
}
#[derive(Hash)]
pub struct Production(pub Symbol, pub Vec<Symbol>);

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbols: Vec<String> = self.1.iter().map(|s| s.to_string()).collect();
        writeln!(f, "{} -> {}", self.0.to_string(), symbols.join(" "))
    }
}

pub fn first_follow_nullable_set(
    productions: Vec<Production>,
) -> (
    HashSet<Symbol>,
    HashMap<Symbol, HashSet<Symbol>>,
    HashMap<Symbol, HashSet<Symbol>>,
) {
    let mut nullable_set: HashSet<Symbol> = HashSet::new();
    let mut first_set: HashMap<Symbol, HashSet<Symbol>> = HashMap::new();
    let mut follow_set: HashMap<Symbol, HashSet<Symbol>> = HashMap::new();

    // 初始化 对于每个终结符 t, FIRST[t] = {t}
    for production in productions.iter() {
        for symbol in &production.1 {
            let t = match symbol {
                Symbol::Terminal(t) => t,
                _ => continue,
            };
            match t {
                Terminal::Name(_) => {
                    first_set
                        .entry(symbol.clone())
                        .or_insert_with(HashSet::new)
                        .insert(symbol.clone());
                }
                _ => continue,
            }
        }
    }

    let mut is_change = true;

    while is_change {
        is_change = false;
        for production in productions.iter() {
            let mut i_is_nullable = true;
            for i in 0..production.1.len() {
                let yi = unsafe { production.1.get_unchecked(i) };

                // if Y1 ... Yi-1 都是可空的 then first_set[X] = first_set[X] U first_set[Yi]
                if i_is_nullable {
                    let first_set_y_opt = first_set.get(yi).cloned();
                    let first_set_x = first_set
                        .entry(production.0.clone())
                        .or_insert_with(|| HashSet::new());

                    if let Some(first_set_y) = first_set_y_opt {
                        let first_set_x_origin_len = first_set_x.len();
                        first_set_x.extend(first_set_y);
                        is_change = first_set_x_origin_len != first_set_x.len() || is_change;
                    }
                }

                match yi {
                    Symbol::Terminal(t) => match t {
                        Terminal::Name(_) => i_is_nullable = false,
                        Terminal::Epsilon => i_is_nullable = i_is_nullable && true,
                    },
                    Symbol::Nonterminal(_) => {
                        i_is_nullable = i_is_nullable && nullable_set.contains(yi);
                    }
                }

                let mut j_is_nullable = true;
                for j in (i + 1)..production.1.len() {
                    let yj = unsafe { production.1.get_unchecked(j) };

                    // if Yi+1...Yj-1 都是可空的 then FOLLOW[Yi] = FOLLOW[Yi] U FOLLOW[Yj]
                    if j_is_nullable {
                        let first_set_yj = first_set.get(yj).cloned().unwrap_or_else(HashSet::new);

                        let follow_set_yi_set = follow_set
                            .entry(yi.clone())
                            .or_insert_with(|| HashSet::new());

                        let follow_set_yi_origin_len = follow_set_yi_set.len();
                        follow_set_yi_set.extend(first_set_yj);
                        is_change =
                            follow_set_yi_origin_len != follow_set_yi_set.len() || is_change;
                    }

                    match yj {
                        Symbol::Terminal(t) => match t {
                            Terminal::Name(_) => j_is_nullable = false,
                            Terminal::Epsilon => j_is_nullable = j_is_nullable && true,
                        },
                        Symbol::Nonterminal(_) => {
                            j_is_nullable = j_is_nullable && nullable_set.contains(yi);
                        }
                    }
                }

                if j_is_nullable {
                    let follow_set_x = follow_set
                        .get(&production.0)
                        .cloned()
                        .unwrap_or_else(HashSet::new);

                    // if Yi+1...Yk 都是可空的 then FOLLOW[Yi] = FOLLOW[Yi] U FOLLOW[X]
                    let follow_set_yi_set = follow_set
                        .entry(yi.clone())
                        .or_insert_with(|| HashSet::new());

                    let follow_yi_set_origin_len = follow_set_yi_set.len();
                    follow_set_yi_set.extend(follow_set_x);
                    is_change = follow_yi_set_origin_len != follow_set_yi_set.len() || is_change;
                }
            }

            if i_is_nullable {
                // if 所有 Yi 都是可空的 then nullable[X] = true
                is_change = nullable_set.insert(production.0.clone()) || is_change;
            }
        }
    }
    (nullable_set, first_set, follow_set)
}

#[test]
fn test_first_follow_nullable_set() {
    let z1 = Production(
        Symbol::Nonterminal("Z".to_string()),
        vec![Symbol::Terminal(Terminal::Name("d".to_string()))],
    );
    let z2 = Production(
        Symbol::Nonterminal("Z".to_string()),
        vec![
            Symbol::Nonterminal("X".to_string()),
            Symbol::Nonterminal("Y".to_string()),
            Symbol::Nonterminal("Z".to_string()),
        ],
    );

    let y1 = Production(
        Symbol::Nonterminal("Y".to_string()),
        vec![Symbol::Terminal(Terminal::Epsilon)],
    );
    let y2 = Production(
        Symbol::Nonterminal("Y".to_string()),
        vec![Symbol::Terminal(Terminal::Name("c".to_string()))],
    );

    let x1 = Production(
        Symbol::Nonterminal("X".to_string()),
        vec![Symbol::Nonterminal("Y".to_string())],
    );

    let x2 = Production(
        Symbol::Nonterminal("X".to_string()),
        vec![Symbol::Terminal(Terminal::Name("a".to_string()))],
    );

    let test_case = vec![(
        vec![z1, z2, y1, y2, x1, x2],
        // nullable set
        vec![
            Symbol::Nonterminal("X".to_owned()),
            Symbol::Nonterminal("Y".to_owned()),
        ],
        // first_set
        vec![
            (
                Symbol::Nonterminal("X".to_owned()),
                vec![
                    Symbol::Terminal(Terminal::Name("a".to_owned())),
                    Symbol::Terminal(Terminal::Name("c".to_owned())),
                ],
            ),
            (
                Symbol::Nonterminal("Y".to_owned()),
                vec![Symbol::Terminal(Terminal::Name("c".to_owned()))],
            ),
            (
                Symbol::Nonterminal("Z".to_owned()),
                vec![
                    Symbol::Terminal(Terminal::Name("a".to_owned())),
                    Symbol::Terminal(Terminal::Name("c".to_owned())),
                    Symbol::Terminal(Terminal::Name("d".to_owned())),
                ],
            ),
        ],
        // follow_set
        vec![
            (
                Symbol::Nonterminal("X".to_owned()),
                vec![
                    Symbol::Terminal(Terminal::Name("a".to_owned())),
                    Symbol::Terminal(Terminal::Name("c".to_owned())),
                    Symbol::Terminal(Terminal::Name("d".to_owned())),
                ],
            ),
            (
                Symbol::Nonterminal("Y".to_owned()),
                vec![
                    Symbol::Terminal(Terminal::Name("a".to_owned())),
                    Symbol::Terminal(Terminal::Name("c".to_owned())),
                    Symbol::Terminal(Terminal::Name("d".to_owned())),
                ],
            ),
        ],
    )];

    for (productions, nullable, first, follow) in test_case {
        let (nullable_set, first_set, follow_set) = first_follow_nullable_set(productions);

        assert!(nullable.iter().all(|s| nullable_set.contains(s)));

        for (s, c) in first {
            let s = first_set.get(&s).unwrap();
            assert!(c.iter().all(|c| s.contains(c)));
        }

        for (s, c) in follow {
            let s = follow_set.get(&s).unwrap();
            assert!(c.iter().all(|c| s.contains(c)));
        }
    }
}
