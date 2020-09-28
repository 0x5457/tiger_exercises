use std::{collections::HashMap, collections::HashSet, fmt};

use crate::cfg::{Production, Symbol};

#[derive(Hash)]
struct ProductionWrapper {
    production: Production,
    cursor: usize,
}

impl fmt::Display for ProductionWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> ", self.production.0.to_string())?;

        for (i, symbol) in self.production.1.iter().enumerate() {
            if i == self.cursor {
                write!(f, ". ")?;
            }
            write!(f, "{} ", symbol.to_string())?;
        }
        Ok(())
    }
}

fn closure(productionWrappers: HashSet<ProductionWrapper>) {
    let mut productionWrapperMapping: HashMap<String, Vec<&ProductionWrapper>> =
        HashMap::with_capacity(productionWrappers.len());
    for productionWrapper in productionWrappers.iter() {
        productionWrapperMapping
            .entry(productionWrapper.production.0.to_string())
            .or_insert(vec![])
            .push(productionWrapper);
    }

}

#[test]
fn test() {
    let p = ProductionWrapper {
        production: Production(
            Symbol::Nonterminal("Z".to_string()),
            vec![
                Symbol::Nonterminal("X".to_string()),
                Symbol::Nonterminal("Y".to_string()),
                Symbol::Nonterminal("Z".to_string()),
            ],
        ),
        cursor: 2,
    };
    println!("{}", p);
}
