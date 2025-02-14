use std::collections::HashMap;

use lib::{Lint, LINTS};
use rnix::SyntaxKind;

pub fn lint_map_of(
    lints: &[&'static Box<dyn Lint>],
) -> HashMap<SyntaxKind, Vec<&'static Box<dyn Lint>>> {
    let mut map = HashMap::new();
    for lint in lints.iter() {
        let lint = *lint;
        let matches = lint.match_kind();
        for m in matches {
            map.entry(m)
                .and_modify(|v: &mut Vec<_>| v.push(lint))
                .or_insert_with(|| vec![lint]);
        }
    }
    map
}

pub fn lint_map() -> HashMap<SyntaxKind, Vec<&'static Box<dyn Lint>>> {
    lint_map_of(&*LINTS)
}
