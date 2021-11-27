pub mod mysql;

use crate::methods::{quote_table_name, table_column_name};
use crate::traits::ModelAble;
use crate::collectors::SqlString;
use crate::table::{SelectManager, select_statement::{SelectStatement, SelectCore}};

pub fn accept_select_manager<'a, M: ModelAble>(select_manager: &'a SelectManager<M>, collector: &'a mut SqlString) -> &'a mut SqlString {
    let ast: &SelectStatement<M> = &select_manager.ast;
    let cores: &Vec<SelectCore<M>> = &ast.cores;
    for core in cores {
        visit_arel_select_core(core, collector);
    }
    collector
}

fn visit_arel_select_core<'a, M: ModelAble>(core: &'a SelectCore<M>, collector: &'a mut SqlString) -> &'a mut SqlString {
    collector.push_str("SELECT ");
    collector.push_str(&table_column_name::<M>("*"));
    collector.push_str(" FROM ");
    collector.push_str(&quote_table_name(&M::table_name()));

    if let Some(sql_literal) = core.get_joins_sql() {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }

    if let Some(sql_literal) = core.get_where_sql() {
        collector.push_str(" WHERE ").push_str(&sql_literal.raw_sql);
    }
    // core.groups
    // core.havings
    // core.groups
    collector
}