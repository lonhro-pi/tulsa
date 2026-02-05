use crate::models::*;
use chrono::Utc;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect(database_url).await?;

    // Create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS subscribers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            username TEXT NOT NULL,
            platform TEXT NOT NULL,
            tier TEXT NOT NULL DEFAULT 'Casual',
            total_spent REAL NOT NULL DEFAULT 0.0,
            last_purchase_date TEXT,
            subscription_end_date TEXT,
            last_interaction_date TEXT,
            preferences TEXT,
            notes TEXT,
            birthday TEXT,
            favorite_content_types TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS interactions (
            id TEXT PRIMARY KEY,
            subscriber_id TEXT NOT NULL,
            interaction_type TEXT NOT NULL,
            amount REAL,
            notes TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (subscriber_id) REFERENCES subscribers(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn create_subscriber(
    pool: &SqlitePool,
    payload: CreateSubscriber,
) -> Result<Subscriber, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let tier = payload.tier.unwrap_or_else(|| "Casual".to_string());

    sqlx::query(
        r#"
        INSERT INTO subscribers (
            id, name, username, platform, tier, total_spent,
            birthday, notes, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, 0.0, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.username)
    .bind(&payload.platform)
    .bind(&tier)
    .bind(&payload.birthday)
    .bind(&payload.notes)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    get_subscriber(pool, &id).await
}

pub async fn get_subscriber(pool: &SqlitePool, id: &str) -> Result<Subscriber, sqlx::Error> {
    sqlx::query_as::<_, Subscriber>(
        r#"
        SELECT * FROM subscribers WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn list_subscribers(
    pool: &SqlitePool,
    platform: Option<String>,
    tier: Option<String>,
) -> Result<Vec<Subscriber>, sqlx::Error> {
    let mut query = "SELECT * FROM subscribers WHERE 1=1".to_string();

    if platform.is_some() {
        query.push_str(" AND platform = ?");
    }
    if tier.is_some() {
        query.push_str(" AND tier = ?");
    }

    query.push_str(" ORDER BY updated_at DESC");

    let mut query_builder = sqlx::query_as::<_, Subscriber>(&query);

    if let Some(p) = platform {
        query_builder = query_builder.bind(p);
    }
    if let Some(t) = tier {
        query_builder = query_builder.bind(t);
    }

    query_builder.fetch_all(pool).await
}

pub async fn update_subscriber(
    pool: &SqlitePool,
    id: &str,
    payload: UpdateSubscriber,
) -> Result<Subscriber, sqlx::Error> {
    let now = Utc::now();

    // Update each field individually if present
    if let Some(name) = payload.name {
        sqlx::query("UPDATE subscribers SET name = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(username) = payload.username {
        sqlx::query("UPDATE subscribers SET username = ?, updated_at = ? WHERE id = ?")
            .bind(username)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(platform) = payload.platform {
        sqlx::query("UPDATE subscribers SET platform = ?, updated_at = ? WHERE id = ?")
            .bind(platform)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(tier) = payload.tier {
        sqlx::query("UPDATE subscribers SET tier = ?, updated_at = ? WHERE id = ?")
            .bind(tier)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(total_spent) = payload.total_spent {
        sqlx::query("UPDATE subscribers SET total_spent = ?, updated_at = ? WHERE id = ?")
            .bind(total_spent)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(last_purchase_date) = payload.last_purchase_date {
        sqlx::query("UPDATE subscribers SET last_purchase_date = ?, updated_at = ? WHERE id = ?")
            .bind(last_purchase_date)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(subscription_end_date) = payload.subscription_end_date {
        sqlx::query("UPDATE subscribers SET subscription_end_date = ?, updated_at = ? WHERE id = ?")
            .bind(subscription_end_date)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(last_interaction_date) = payload.last_interaction_date {
        sqlx::query("UPDATE subscribers SET last_interaction_date = ?, updated_at = ? WHERE id = ?")
            .bind(last_interaction_date)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(preferences) = payload.preferences {
        sqlx::query("UPDATE subscribers SET preferences = ?, updated_at = ? WHERE id = ?")
            .bind(preferences)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(notes) = payload.notes {
        sqlx::query("UPDATE subscribers SET notes = ?, updated_at = ? WHERE id = ?")
            .bind(notes)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(birthday) = payload.birthday {
        sqlx::query("UPDATE subscribers SET birthday = ?, updated_at = ? WHERE id = ?")
            .bind(birthday)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(favorite_content_types) = payload.favorite_content_types {
        sqlx::query("UPDATE subscribers SET favorite_content_types = ?, updated_at = ? WHERE id = ?")
            .bind(favorite_content_types)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
    }

    get_subscriber(pool, id).await
}

pub async fn delete_subscriber(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM subscribers WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn create_interaction(
    pool: &SqlitePool,
    payload: CreateInteraction,
) -> Result<Interaction, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query(
        r#"
        INSERT INTO interactions (id, subscriber_id, interaction_type, amount, notes, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&payload.subscriber_id)
    .bind(&payload.interaction_type)
    .bind(payload.amount)
    .bind(&payload.notes)
    .bind(now)
    .execute(pool)
    .await?;

    // Update subscriber's last interaction date and possibly total spent
    if let Some(amount) = payload.amount {
        sqlx::query(
            r#"
            UPDATE subscribers 
            SET last_interaction_date = ?,
                last_purchase_date = ?,
                total_spent = total_spent + ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(now)
        .bind(now)
        .bind(amount)
        .bind(now)
        .bind(&payload.subscriber_id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"
            UPDATE subscribers 
            SET last_interaction_date = ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(now)
        .bind(now)
        .bind(&payload.subscriber_id)
        .execute(pool)
        .await?;
    }

    sqlx::query_as::<_, Interaction>("SELECT * FROM interactions WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn get_subscriber_interactions(
    pool: &SqlitePool,
    subscriber_id: &str,
) -> Result<Vec<Interaction>, sqlx::Error> {
    sqlx::query_as::<_, Interaction>(
        r#"
        SELECT * FROM interactions 
        WHERE subscriber_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(subscriber_id)
    .fetch_all(pool)
    .await
}

pub async fn get_stats(pool: &SqlitePool) -> Result<SubscriberStats, sqlx::Error> {
    let total_subscribers: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM subscribers")
        .fetch_one(pool)
        .await?;

    let vip_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM subscribers WHERE tier = 'VIP'")
        .fetch_one(pool)
        .await?;

    let regular_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM subscribers WHERE tier = 'Regular'")
            .fetch_one(pool)
            .await?;

    let casual_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM subscribers WHERE tier = 'Casual'")
            .fetch_one(pool)
            .await?;

    let total_revenue: Option<f64> =
        sqlx::query_scalar("SELECT SUM(total_spent) FROM subscribers")
            .fetch_one(pool)
            .await?;

    let expiring_soon: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM subscribers 
        WHERE subscription_end_date IS NOT NULL 
        AND datetime(subscription_end_date) BETWEEN datetime('now') AND datetime('now', '+7 days')
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(SubscriberStats {
        total_subscribers,
        vip_count,
        regular_count,
        casual_count,
        total_revenue: total_revenue.unwrap_or(0.0),
        expiring_soon,
    })
}

pub async fn get_expiring_subscriptions(
    pool: &SqlitePool,
    days: i64,
) -> Result<Vec<Subscriber>, sqlx::Error> {
    sqlx::query_as::<_, Subscriber>(
        r#"
        SELECT * FROM subscribers 
        WHERE subscription_end_date IS NOT NULL 
        AND datetime(subscription_end_date) BETWEEN datetime('now') AND datetime('now', '+' || ? || ' days')
        ORDER BY subscription_end_date ASC
        "#,
    )
    .bind(days)
    .fetch_all(pool)
    .await
}
