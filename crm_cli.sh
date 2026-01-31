use crate::models::*;
use chrono::{Duration, Utc};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePool::connect(database_url).await?;
    
    // Create subscribers table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS subscribers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            username TEXT NOT NULL,
            platform TEXT NOT NULL,
            tier TEXT NOT NULL DEFAULT 'Regular',
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

    // Create interactions table
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

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_subscriber_platform ON subscribers(platform)")
        .execute(&pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_subscriber_tier ON subscribers(tier)")
        .execute(&pool)
        .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_interaction_subscriber ON interactions(subscriber_id)")
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn create_subscriber(
    pool: &SqlitePool,
    subscriber: CreateSubscriber,
) -> Result<Subscriber, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let tier = subscriber.tier.unwrap_or_else(|| "Regular".to_string());

    let result = sqlx::query_as::<_, Subscriber>(
        r#"
        INSERT INTO subscribers (
            id, name, username, platform, tier, total_spent,
            birthday, notes, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, 0.0, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&id)
    .bind(&subscriber.name)
    .bind(&subscriber.username)
    .bind(&subscriber.platform)
    .bind(&tier)
    .bind(&subscriber.birthday)
    .bind(&subscriber.notes)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn get_subscriber(pool: &SqlitePool, id: &str) -> Result<Subscriber, sqlx::Error> {
    sqlx::query_as::<_, Subscriber>("SELECT * FROM subscribers WHERE id = ?")
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
    update: UpdateSubscriber,
) -> Result<Subscriber, sqlx::Error> {
    let now = Utc::now();
    
    // Build dynamic update query
    let mut updates = vec!["updated_at = ?".to_string()];
    let mut params: Vec<String> = vec![now.to_rfc3339()];
    
    if let Some(name) = update.name {
        updates.push("name = ?".to_string());
        params.push(name);
    }
    if let Some(username) = update.username {
        updates.push("username = ?".to_string());
        params.push(username);
    }
    if let Some(platform) = update.platform {
        updates.push("platform = ?".to_string());
        params.push(platform);
    }
    if let Some(tier) = update.tier {
        updates.push("tier = ?".to_string());
        params.push(tier);
    }
    if let Some(total_spent) = update.total_spent {
        updates.push("total_spent = ?".to_string());
        params.push(total_spent.to_string());
    }
    if let Some(last_purchase_date) = update.last_purchase_date {
        updates.push("last_purchase_date = ?".to_string());
        params.push(last_purchase_date.to_rfc3339());
    }
    if let Some(subscription_end_date) = update.subscription_end_date {
        updates.push("subscription_end_date = ?".to_string());
        params.push(subscription_end_date.to_rfc3339());
    }
    if let Some(last_interaction_date) = update.last_interaction_date {
        updates.push("last_interaction_date = ?".to_string());
        params.push(last_interaction_date.to_rfc3339());
    }
    if let Some(preferences) = update.preferences {
        updates.push("preferences = ?".to_string());
        params.push(preferences);
    }
    if let Some(notes) = update.notes {
        updates.push("notes = ?".to_string());
        params.push(notes);
    }
    if let Some(birthday) = update.birthday {
        updates.push("birthday = ?".to_string());
        params.push(birthday);
    }
    if let Some(favorite_content_types) = update.favorite_content_types {
        updates.push("favorite_content_types = ?".to_string());
        params.push(favorite_content_types);
    }

    let query = format!(
        "UPDATE subscribers SET {} WHERE id = ? RETURNING *",
        updates.join(", ")
    );

    let mut query_builder = sqlx::query_as::<_, Subscriber>(&query);
    for param in params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(id);

    query_builder.fetch_one(pool).await
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
    interaction: CreateInteraction,
) -> Result<Interaction, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    let result = sqlx::query_as::<_, Interaction>(
        r#"
        INSERT INTO interactions (id, subscriber_id, interaction_type, amount, notes, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING *
        "#,
    )
    .bind(&id)
    .bind(&interaction.subscriber_id)
    .bind(&interaction.interaction_type)
    .bind(&interaction.amount)
    .bind(&interaction.notes)
    .bind(now)
    .fetch_one(pool)
    .await?;

    // Update subscriber's last interaction date and possibly total spent
    if let Some(amount) = interaction.amount {
        sqlx::query(
            "UPDATE subscribers SET last_interaction_date = ?, total_spent = total_spent + ?, updated_at = ? WHERE id = ?"
        )
        .bind(now)
        .bind(amount)
        .bind(now)
        .bind(&interaction.subscriber_id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            "UPDATE subscribers SET last_interaction_date = ?, updated_at = ? WHERE id = ?"
        )
        .bind(now)
        .bind(now)
        .bind(&interaction.subscriber_id)
        .execute(pool)
        .await?;
    }

    Ok(result)
}

pub async fn get_subscriber_interactions(
    pool: &SqlitePool,
    subscriber_id: &str,
) -> Result<Vec<Interaction>, sqlx::Error> {
    sqlx::query_as::<_, Interaction>(
        "SELECT * FROM interactions WHERE subscriber_id = ? ORDER BY created_at DESC"
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

    let regular_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM subscribers WHERE tier = 'Regular'")
        .fetch_one(pool)
        .await?;

    let casual_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM subscribers WHERE tier = 'Casual'")
        .fetch_one(pool)
        .await?;

    let total_revenue: f64 = sqlx::query_scalar("SELECT COALESCE(SUM(total_spent), 0.0) FROM subscribers")
        .fetch_one(pool)
        .await?;

    // Count subscriptions expiring in the next 7 days
    let expiry_threshold = (Utc::now() + Duration::days(7)).to_rfc3339();
    let expiring_soon: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM subscribers WHERE subscription_end_date IS NOT NULL AND subscription_end_date <= ? AND subscription_end_date >= ?"
    )
    .bind(&expiry_threshold)
    .bind(Utc::now().to_rfc3339())
    .fetch_one(pool)
    .await?;

    Ok(SubscriberStats {
        total_subscribers,
        vip_count,
        regular_count,
        casual_count,
        total_revenue,
        expiring_soon,
    })
}

pub async fn get_expiring_subscriptions(
    pool: &SqlitePool,
    days: i64,
) -> Result<Vec<Subscriber>, sqlx::Error> {
    let expiry_threshold = (Utc::now() + Duration::days(days)).to_rfc3339();
    
    sqlx::query_as::<_, Subscriber>(
        "SELECT * FROM subscribers WHERE subscription_end_date IS NOT NULL AND subscription_end_date <= ? AND subscription_end_date >= ? ORDER BY subscription_end_date ASC"
    )
    .bind(&expiry_threshold)
    .bind(Utc::now().to_rfc3339())
    .fetch_all(pool)
    .await
}
