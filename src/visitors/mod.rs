pub mod mysql;

use crate::methods::{quote_table_name};
use crate::traits::ModelAble;
use crate::collectors::SqlString;
use crate::table::{
    SelectManager,
    update_manager::{UpdateManager, UpdateStatement}
};
use crate::table::select_manager::select_statement::{SelectCore, SelectStatement};

pub fn accept_select_manager<'a, M: ModelAble>(select_manager: &'a SelectManager<M>, collector: &'a mut SqlString) -> &'a mut SqlString {
    let ast: &SelectStatement<M> = &select_manager.ast;
    let cores: &Vec<SelectCore<M>> = &ast.cores;

    for core in cores {
        visit_arel_select_core(core, collector);
    }
    // SelectOptions
    if let Some(sql_literal) = ast.get_order_sql() {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }

    if let Some(sql_literal) = ast.get_lock_sql() {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }

    collector
}

fn visit_arel_select_core<'a, M: ModelAble>(core: &'a SelectCore<M>, collector: &'a mut SqlString) -> &'a mut SqlString {
    collector.push_str(&core.get_select_sql().raw_sql);

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

pub fn accept_update_manager<'a, M: ModelAble>(update_manager: &'a UpdateManager<M>, collector: &'a mut SqlString) -> &'a mut SqlString {
    let ast: &UpdateStatement<M> = &update_manager.ast;

    if let Some(sql_literal) = ast.get_update_sql() {
        collector.push_str(&sql_literal.raw_sql);
    }

    if let Some(sql_literal) = ast.get_where_sql() {
        collector.push_str(" WHERE ").push_str(&sql_literal.raw_sql);
    }

    collector
}