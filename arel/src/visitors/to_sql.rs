use crate::methods::{quote_table_name};
use crate::traits::ArelAble;
use crate::collectors::Sql;
use crate::table::{SelectManager, update_manager::{UpdateManager, UpdateStatement}, insert_manager::{InsertManager, InsertStatement}, delete_manager::{DeleteManager, DeleteStatement}};
use crate::table::select_manager::select_statement::{SelectCore, SelectStatement};
use crate::methods;

pub fn accept_select_manager<'a, M: ArelAble>(select_manager: &'a SelectManager<M>, sql: &'a mut Sql) -> anyhow::Result<&'a mut Sql> {
    let ast: &SelectStatement<M> = &select_manager.ast;
    let cores: &Vec<SelectCore<M>> = &ast.cores;
    for core in cores {
        visit_arel_select_core(core, sql)?;
    }
    // SelectOptions
    if let Some(sub_sql) = ast.get_order_sql()? {
        sql.push_str(" ORDER BY ").push_from_sql(&sub_sql);
    }
    if let Some(sub_sql) = ast.get_limit_sql()? {
        sql.push_str(" ").push_from_sql(&sub_sql);
    }
    if let Some(sub_sql) = ast.get_offset_sql()? {
        sql.push_str(" ").push_from_sql(&sub_sql);
    }
    if let Some(sub_sql) = ast.get_lock_sql()? {
        sql.push_str(" ").push_from_sql(&sub_sql);
    }
    Ok(sql)
}

fn visit_arel_select_core<'a, M: ArelAble>(core: &'a SelectCore<M>, sql: &'a mut Sql) -> anyhow::Result<&'a mut Sql> {
    sql.push_from_sql(&core.get_select_sql()?);
    sql.push_str(" FROM ");
    sql.push_str(&quote_table_name(&M::table_name()));
    if let Some(sub_sql) = core.get_joins_sql()? {
        sql.push_str(" ").push_from_sql(&sub_sql);
    }
    if let Some(sub_sql) = core.get_where_sql()? {
        sql.push_str(" WHERE ").push_from_sql(&sub_sql);
    }
    // core.groups
    if let Some(sub_sql) = core.get_group_sql()? {
        sql.push_str(" GROUP BY ").push_from_sql(&sub_sql);
    }
    // core.havings
    if let Some(sub_sql) = core.get_having_sql()? {
        sql.push_str(" HAVING ").push_from_sql(&sub_sql);
    }
    Ok(sql)
}

pub fn accept_update_manager<'a, M: ArelAble>(update_manager: &'a UpdateManager<M>, for_update_select_manager: Option<&mut SelectManager<M>>, sql: &'a mut Sql) -> anyhow::Result<&'a mut Sql> {
    let ast: &UpdateStatement<M> = &update_manager.ast;
    if let Some(sub_sql) = ast.get_update_sql()? {
        sql.push_from_sql(&sub_sql);
    }

    let mut exists_where = false;
    if let Some(sub_sql) = ast.get_where_sql()? {
        sql.push_str(" WHERE ").push_from_sql(&sub_sql);
        exists_where = true;
    }
    if let Some(sub_sql) = ast.get_order_sql()? {
        sql.push_str(" ORDER BY ").push_from_sql(&sub_sql);
    }
    if let Some(sub_sql) = ast.get_limit_sql()? {
        sql.push_str(" ").push_from_sql(&sub_sql);
    }
    if let Some(sub_sql) = ast.get_offset_sql()? {
        sql.push_str(" ").push_from_sql(&sub_sql);
    }
    if let Some(subquery) = accept_subquery_select_manager(for_update_select_manager)? {
        sql.push_str(if exists_where { " AND " } else { " WHERE " });
        sql.push_str(&subquery);
    }
    Ok(sql)
}

pub fn accept_insert_manager<'a, M: ArelAble>(insert_manager: &'a InsertManager<M>, sql: &'a mut Sql) -> anyhow::Result<&'a mut Sql> {
    let ast: &InsertStatement<M> = &insert_manager.ast;
    if let Some(sub_sql) = ast.get_insert_sql()? {
        sql.push_from_sql(&sub_sql);
    }
    Ok(sql)
}

pub fn accept_delete_manager<'a, M: ArelAble>(delete_manager: &'a DeleteManager<M>, for_delete_select_manager: Option<&mut SelectManager<M>>, sql: &'a mut Sql) -> anyhow::Result<&'a mut Sql> {
    let ast: &DeleteStatement<M> = &delete_manager.ast;
    sql.push_str("DELETE FROM ");
    sql.push_str(&quote_table_name(&M::table_name()));

    let mut exists_where = false;
    if let Some(sub_sql) = ast.get_where_sql()? {
        sql.push_str(" WHERE ").push_from_sql(&sub_sql);
        exists_where = true;
    }
    if let Some(sub_sql) = ast.get_order_sql()? {
        sql.push_str(" ORDER BY ").push_from_sql(&sub_sql);
    }
    if let Some(sub_sql) = ast.get_limit_sql()? {
        sql.push_str(" ").push_from_sql(&sub_sql);
    }
    if let Some(sub_sql) = ast.get_offset_sql()? {
        sql.push_str(" ").push_from_sql(&sub_sql);
    }
    if let Some(subquery) = accept_subquery_select_manager(for_delete_select_manager)? {
        sql.push_str(if exists_where { " AND " } else { " WHERE " });
        sql.push_str(&subquery);
    }
    Ok(sql)
}

fn accept_subquery_select_manager<M: ArelAble>(subquery_select_manager: Option<&mut SelectManager<M>>) -> anyhow::Result<Option<String>> {
    if let Some(subquery_select_manager) = subquery_select_manager {
        let mut select_collector = Sql::default();
        let subquery = format!("SELECT `{}` FROM ({}) AS __arel_subquery_temp", M::primary_key(), accept_select_manager(subquery_select_manager, &mut select_collector)?.value);
        Ok(Some(format!("{} IN ({})", methods::table_column_name::<M>(M::primary_key()), subquery)))
    } else {
        Ok(None)
    }

}