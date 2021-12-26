use arel::prelude::*;

#[arel]
#[has_and_belongs_to_many("users", struct = "User", join_table = "admins_users", foreign_key = "admin_id", association_foreign_key = "user_id")]
#[has_many("avatar_sources", struct = "AvatarSource", through = "users")]
struct Admin {
    #[arel(table_column_name="id")]
    id: Option<i64>,
}


#[arel(
    primary_key="id",
    has_many("children", struct = "User", foreign_key = "parent_id"),
    has_many("orders", struct = "Order", foreign_key = "user_id"),
    has_many("order_shops", struct = "OrderShop", through = "orders"),
    has_many("order_shop_products", struct = "OrderShopProduct", through = "order_shops", source = "order_products"),
    has_many("order_shop_product_snapshots", struct = "OrderShopProductSnapshot", through = "order_shop_products", source = "order_shop_snapshot"),
    has_many("order_shop_product_snapshots2", struct = "OrderShopProductSnapshot", through = "order_shops", source = "order_product_snapshots"),
    has_one("wallet", struct = "Wallet", foreign_key = "user_id"),
    has_one("wallet_extra", struct = "WalletExtraInfo", through = "wallet", source = "wallet_info"),
    has_and_belongs_to_many("avatars", struct = "Avatar", join_table = "users_avatars", foreign_key = "user_id", association_foreign_key = "avatar_id"),
    has_many("avatar_sources", struct = "AvatarSource", through = "avatars", source = "avatar_source"),
)]
struct User {
    #[arel(table_column_name="id")]
    uid: Option<i64>,
    parent_id: Option<i64>,
}

#[arel]
#[belongs_to("user", struct = "User", foreign_key = "user_id")]
#[has_many("order_shops", struct = "OrderShop")]
struct Order {
    id: Option<i32>,
    user_id: Option<i32>,
}

// foreign_key => default: OrderShop => order_shop_id in OrderShopProduct
#[arel(
has_many("order_products", struct = "OrderShopProduct"),
has_many("order_product_snapshots", struct = "OrderShopProduct", through = "order_products", source = "order_shop_snapshot"),
)]
struct OrderShop {
    id: Option<i32>,
    order_id: Option<i32>,
}

// foreign_key => default: OrderShopProduct => order_shop_product_id in order_shop_product_snapshots
#[arel]
#[has_one("order_shop_snapshot", struct = "OrderShopProductSnapshot")]
struct OrderShopProduct {
    id: Option<i32>,
    order_shop_id: Option<i32>,
}

#[arel]
struct OrderShopProductSnapshot {
    id: Option<i32>,
    order_shop_product_id: Option<i32>,
}

#[arel(
    belongs_to("user", struct = "User"),
    has_one("wallet_info", struct = "WalletExtraInfo", foreign_key = "owner_wallet_id")
)]
struct Wallet {
    id: Option<i32>,
    user_id: Option<i32>,
}

#[arel(
belongs_to("wallet", struct = "Wallet", foreign_key = "owner_wallet_id"),
)]
struct WalletExtraInfo {
    id: Option<i32>,
    owner_wallet_id: Option<i32>,
}

#[arel(
has_one("avatar_source", struct = "AvatarSource")
)]
struct Avatar {
    id: Option<i32>,
}

#[arel]
struct AvatarSource {
    id: Option<i32>,
    avatar_id: Option<i32>,
}

async fn init_db() -> anyhow::Result<()> {
    let db_state = arel::visitors::get_or_init_db_state(|| Box::pin(async {
        sqlx::any::AnyPoolOptions::new().max_connections(5).connect("sqlite::memory:").await
    })).await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS admins
            (
                id          INTEGER PRIMARY KEY NOT NULL
            );"
    ).execute(db_state.pool()).await?;
    for _ in 0..10 {
        Admin::create(json!({})).execute().await?;
    }


    sqlx::query("CREATE TABLE IF NOT EXISTS users
            (
                id          INTEGER PRIMARY KEY NOT NULL
            );"
    ).execute(db_state.pool()).await?;
    for _ in 0..10 {
        User::create(json!({})).execute().await?;
    }

    sqlx::query("CREATE TABLE IF NOT EXISTS orders
            (
                id          INTEGER PRIMARY KEY NOT NULL,
                user_id INTEGER
            );"
    ).execute(db_state.pool()).await?;
    for _ in 0..10 {
        Order::create(json!({
            "user_id": 1
        })).execute().await?;
    }

    sqlx::query("CREATE TABLE IF NOT EXISTS wallets
            (
                id          INTEGER PRIMARY KEY NOT NULL,
                user_id INTEGER
            );"
    ).execute(db_state.pool()).await?;
    for _ in 0..10 {
        Wallet::create(json!({
            "user_id": 1
        })).execute().await?;
    }

    Ok(())
}

async fn main_test() -> anyhow::Result<()> {
    init_db().await?;

    // belongs_to
    let o1 = Order::fetch_last().await?;
    assert_eq!(o1.user()?.to_sql_string()?, "SELECT * FROM \"users\" WHERE \"users\".\"id\" = 1");
    assert_eq!(Order::user_join_string(), "INNER JOIN orders ON users.id = orders.user_id");

    // has_many
    let u1 = User::query().fetch_one().await?;
    assert_eq!(u1.children()?.to_sql_string()?, "SELECT * FROM \"users\" WHERE \"users\".\"parent_id\" = 1"); // 自关联
    assert_eq!(User::children_join_string(), "INNER JOIN users ON users.parent_id = users.id");
    assert_eq!(u1.orders()?.to_sql_string()?, "SELECT * FROM \"orders\" WHERE \"orders\".\"user_id\" = 1");
    assert_eq!(User::orders_join_string(), "INNER JOIN users ON orders.user_id = users.id");
    // has_one
    assert_eq!(u1.wallet()?.to_sql_string()?, "SELECT * FROM \"wallets\" WHERE \"wallets\".\"user_id\" = 1 LIMIT 1");
    assert_eq!(User::wallet_join_string(), "INNER JOIN users ON wallets.user_id = users.id");
    let w1 = u1.wallet()?.fetch_one().await?;
    assert_eq!(w1.wallet_info()?.to_sql_string()?, "SELECT * FROM \"wallet_extra_infos\" WHERE \"wallet_extra_infos\".\"owner_wallet_id\" = 1 LIMIT 1");
    assert_eq!(Wallet::wallet_info_join_string(), "INNER JOIN wallets ON wallet_extra_infos.owner_wallet_id = wallets.id");

    // has_many through
    assert_eq!(u1.order_shops()?.to_sql_string()?, "SELECT * FROM \"order_shops\" INNER JOIN orders ON order_shops.order_id = orders.id WHERE orders.user_id = 1");
    assert_eq!(User::order_shops_join_string(), "INNER JOIN orders ON order_shops.order_id = orders.id INNER JOIN users ON orders.user_id = users.id");
    assert_eq!(u1.order_shop_products()?.to_sql_string()?, "SELECT * FROM \"order_shop_products\" INNER JOIN order_shops ON order_shop_products.order_shop_id = order_shops.id INNER JOIN orders ON order_shops.order_id = orders.id WHERE orders.user_id = 1");
    assert_eq!(User::order_shop_products_join_string(), "INNER JOIN order_shops ON order_shop_products.order_shop_id = order_shops.id INNER JOIN orders ON order_shops.order_id = orders.id INNER JOIN users ON orders.user_id = users.id");
    assert_eq!(u1.order_shop_product_snapshots()?.to_sql_string()?, "SELECT * FROM \"order_shop_product_snapshots\" INNER JOIN order_shop_products ON order_shop_product_snapshots.order_shop_product_id = order_shop_products.id INNER JOIN order_shops ON order_shop_products.order_shop_id = order_shops.id INNER JOIN orders ON order_shops.order_id = orders.id WHERE orders.user_id = 1");
    assert_eq!(u1.order_shop_product_snapshots()?.to_sql_string()?, u1.order_shop_product_snapshots2()?.to_sql_string()?);
    assert_eq!(User::order_shop_product_snapshots_join_string(), "INNER JOIN order_shop_products ON order_shop_product_snapshots.order_shop_product_id = order_shop_products.id INNER JOIN order_shops ON order_shop_products.order_shop_id = order_shops.id INNER JOIN orders ON order_shops.order_id = orders.id INNER JOIN users ON orders.user_id = users.id");
    assert_eq!(User::order_shop_product_snapshots_join_string(), User::order_shop_product_snapshots2_join_string());

    // has_one through
    assert_eq!(u1.wallet_extra()?.to_sql_string()?, "SELECT * FROM \"wallet_extra_infos\" INNER JOIN wallets ON wallet_extra_infos.owner_wallet_id = wallets.id WHERE wallets.user_id = 1 LIMIT 1");
    assert_eq!(User::wallet_extra_join_string(), "INNER JOIN wallets ON wallet_extra_infos.owner_wallet_id = wallets.id INNER JOIN users ON wallets.user_id = users.id");

    // has_and_belongs_to_many
    assert_eq!(u1.avatars()?.to_sql_string()?, "SELECT * FROM \"avatars\" INNER JOIN users_avatars ON avatars.id = users_avatars.avatar_id WHERE users_avatars.user_id = 1");
    assert_eq!(User::avatars_join_string(), "INNER JOIN users_avatars ON avatars.id = users_avatars.avatar_id INNER JOIN users ON users_avatars.user_id = users.id");
    assert_eq!(u1.avatar_sources()?.to_sql_string()?, "SELECT * FROM \"avatar_sources\" INNER JOIN avatars ON avatar_sources.avatar_id = avatars.id INNER JOIN users_avatars ON avatars.id = users_avatars.avatar_id WHERE users_avatars.user_id = 1");
    assert_eq!(User::avatar_sources_join_string(), "INNER JOIN avatars ON avatar_sources.avatar_id = avatars.id INNER JOIN users_avatars ON avatars.id = users_avatars.avatar_id INNER JOIN users ON users_avatars.user_id = users.id");
    let a1 = Admin::fetch_first().await?;
    assert_eq!(a1.users()?.to_sql_string()?, "SELECT * FROM \"users\" INNER JOIN admins_users ON users.id = admins_users.user_id WHERE admins_users.admin_id = 1");
    assert_eq!(Admin::users_join_string(), "INNER JOIN admins_users ON users.id = admins_users.user_id INNER JOIN admins ON admins_users.admin_id = admins.id");
    assert_eq!(a1.avatar_sources()?.to_sql_string()?, "SELECT * FROM \"avatar_sources\" INNER JOIN avatars ON avatar_sources.avatar_id = avatars.id INNER JOIN users_avatars ON avatars.id = users_avatars.avatar_id INNER JOIN users ON users_avatars.user_id = users.id INNER JOIN admins_users ON users.id = admins_users.user_id WHERE admins_users.admin_id = 1");
    assert_eq!(Admin::avatar_sources_join_string(), "INNER JOIN avatars ON avatar_sources.avatar_id = avatars.id INNER JOIN users_avatars ON avatars.id = users_avatars.avatar_id INNER JOIN users ON users_avatars.user_id = users.id INNER JOIN admins_users ON users.id = admins_users.user_id INNER JOIN admins ON admins_users.admin_id = admins.id");


    Ok(())
}

#[test]
fn test_association() {
    assert!(
        match tokio_test::block_on(main_test()) {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("err: {:?}", e);
                Err(e)
            }
        }.is_ok());
}
