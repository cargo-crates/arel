pub mod mysql;

use crate::methods::{quote_table_name};
use crate::traits::ModelAble;
use crate::collectors::SqlString;
use crate::table::{SelectManager, update_manager::{UpdateManager, UpdateStatement}, insert_manager::{InsertManager, InsertStatement}, delete_manager::{DeleteManager, DeleteStatement}};
use crate::table::select_manager::select_statement::{SelectCore, SelectStatement};
use crate::methods;

pub fn accept_select_manager<'a, M: ModelAble>(select_manager: &'a SelectManager<M>, collector: &'a mut SqlString) -> anyhow::Result<&'a mut SqlString> {
    let ast: &SelectStatement<M> = &select_manager.ast;
    let cores: &Vec<SelectCore<M>> = &ast.cores;
    for core in cores {
        visit_arel_select_core(core, collector)?;
    }
    // SelectOptions
    if let Some(sql_literal) = ast.get_order_sql()? {
        collector.push_str(" ORDER BY ").push_str(&sql_literal.raw_sql);
    }
    if let Some(sql_literal) = ast.get_limit_sql()? {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }
    if let Some(sql_literal) = ast.get_offset_sql()? {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }
    if let Some(sql_literal) = ast.get_lock_sql()? {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }
    Ok(collector)
}

fn visit_arel_select_core<'a, M: ModelAble>(core: &'a SelectCore<M>, collector: &'a mut SqlString) -> anyhow::Result<&'a mut SqlString> {
    collector.push_str(&core.get_select_sql()?.raw_sql);
    collector.push_str(" FROM ");
    collector.push_str(&quote_table_name(&M::table_name()));
    if let Some(sql_literal) = core.get_joins_sql()? {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }
    if let Some(sql_literal) = core.get_where_sql()? {
        collector.push_str(" WHERE ").push_str(&sql_literal.raw_sql);
    }
    // core.groups
    if let Some(sql_literal) = core.get_group_sql()? {
        collector.push_str(" GROUP BY ").push_str(&sql_literal.raw_sql);
    }
    // core.havings
    if let Some(sql_literal) = core.get_having_sql()? {
        collector.push_str(" HAVING ").push_str(&sql_literal.raw_sql);
    }
    Ok(collector)
}

pub fn accept_update_manager<'a, M: ModelAble>(update_manager: &'a UpdateManager<M>, for_update_select_manager: Option<&mut SelectManager<M>>, collector: &'a mut SqlString) -> anyhow::Result<&'a mut SqlString> {
    let ast: &UpdateStatement<M> = &update_manager.ast;
    if let Some(sql_literal) = ast.get_update_sql()? {
        collector.push_str(&sql_literal.raw_sql);
    }

    let mut exists_where = false;
    if let Some(sql_literal) = ast.get_where_sql()? {
        collector.push_str(" WHERE ").push_str(&sql_literal.raw_sql);
        exists_where = true;
    }
    if let Some(subquery) = accept_subquery_select_manager(for_update_select_manager)? {
        collector.push_str(if exists_where { " AND " } else { " WHERE " });
        collector.push_str(&subquery);
    }
    Ok(collector)
}

pub fn accept_insert_manager<'a, M: ModelAble>(insert_manager: &'a InsertManager<M>, collector: &'a mut SqlString) -> anyhow::Result<&'a mut SqlString> {
    let ast: &InsertStatement<M> = &insert_manager.ast;
    if let Some(sql_literal) = ast.get_insert_sql()? {
        collector.push_str(&sql_literal.raw_sql);
    }
    Ok(collector)
}

pub fn accept_delete_manager<'a, M: ModelAble>(delete_manager: &'a DeleteManager<M>, for_delete_select_manager: Option<&mut SelectManager<M>>, collector: &'a mut SqlString) -> anyhow::Result<&'a mut SqlString> {
    let ast: &DeleteStatement<M> = &delete_manager.ast;
    collector.push_str("DELETE FROM ");
    collector.push_str(&quote_table_name(&M::table_name()));

    let mut exists_where = false;
    if let Some(sql_literal) = ast.get_where_sql()? {
        collector.push_str(" WHERE ").push_str(&sql_literal.raw_sql);
        exists_where = true;
    }
    if let Some(sql_literal) = ast.get_order_sql()? {
        collector.push_str(" ORDER BY ").push_str(&sql_literal.raw_sql);
    }
    if let Some(sql_literal) = ast.get_limit_sql()? {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }
    if let Some(sql_literal) = ast.get_offset_sql()? {
        collector.push_str(" ").push_str(&sql_literal.raw_sql);
    }
    if let Some(subquery) = accept_subquery_select_manager(for_delete_select_manager)? {
        collector.push_str(if exists_where { " AND " } else { " WHERE " });
        collector.push_str(&subquery);
    }
    Ok(collector)
}

fn accept_subquery_select_manager<M: ModelAble>(subquery_select_manager: Option<&mut SelectManager<M>>) -> anyhow::Result<Option<String>> {
    if let Some(subquery_select_manager) = subquery_select_manager {
        let mut select_collector = SqlString::default();
        let subquery = format!("SELECT `{}` FROM ({}) AS __arel_subquery_temp", M::primary_key(), accept_select_manager(subquery_select_manager, &mut select_collector)?.value);
        Ok(Some(format!("{} IN ({})", methods::table_column_name::<M>(M::primary_key()), subquery)))
    } else {
        Ok(None)
    }

}