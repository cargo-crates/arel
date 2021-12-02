pub mod mysql;

use crate::methods::{quote_table_name};
use crate::traits::ModelAble;
use crate::collectors::SqlString;
use crate::table::{
    SelectManager,
    update_manager::{UpdateManager, UpdateStatement},
    insert_manager::{InsertManager, InsertStatement},
};
use crate::table::select_manager::select_statement::{SelectCore, SelectStatement};
use crate::methods;

pub fn accept_select_manager<'a, M: ModelAble>(select_manager: &'a SelectManager<M>, collector: &'a mut SqlString) -> &'a mut SqlString {
    let ast: &SelectStatement<M> = &select_manager.ast;
    let cores: &Vec<SelectCore<M>> = &ast.cores;
    for core in cores {
        visit_arel_select_core(core, collector);
    }
    // SelectOptions
    if let Some(sql_literal) = ast.get_order_sql() {
        collector.push_str(" ORDER BY ").push_str(&sql_literal.raw_sql);
    }
    if let Some(sql_literal) = ast.get_limit_sql() {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }
    if let Some(sql_literal) = ast.get_offset_sql() {
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
    if let Some(sql_literal) = core.get_group_sql() {
        collector.push_str(" GROUP BY ").push_str(&sql_literal.raw_sql);
    }
    // core.havings
    if let Some(sql_literal) = core.get_having_sql() {
        collector.push_str(" HAVING ").push_str(&sql_literal.raw_sql);
    }
    collector
}

pub fn accept_update_manager<'a, M: ModelAble>(update_manager: &'a UpdateManager<M>, for_update_select_manager: Option<&mut SelectManager<M>>, collector: &'a mut SqlString) -> &'a mut SqlString {
    let ast: &UpdateStatement<M> = &update_manager.ast;
    if let Some(sql_literal) = ast.get_update_sql() {
        collector.push_str(&sql_literal.raw_sql);
    }
    if let Some(for_update_select_manager) = &for_update_select_manager {
        let mut select_collector = SqlString::default();
        let sub_query = format!("SELECT `{}` FROM ({}) AS __arel_subquery_temp", M::primary_key(), accept_select_manager(for_update_select_manager, &mut select_collector).value);
        collector.push_str(&format!(" WHERE {} IN ({})", methods::table_column_name::<M>(M::primary_key()), sub_query));
    } else if let Some(sql_literal) = ast.get_where_sql() {
        collector.push_str(" WHERE ").push_str(&sql_literal.raw_sql);
    }
    collector
}

pub fn accept_insert_manager<'a, M: ModelAble>(insert_manager: &'a InsertManager<M>, collector: &'a mut SqlString) -> &'a mut SqlString {
    let ast: &InsertStatement<M> = &insert_manager.ast;
    if let Some(sql_literal) = ast.get_insert_sql() {
        collector.push_str(&sql_literal.raw_sql);
    }
    collector
}