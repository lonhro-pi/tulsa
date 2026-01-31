# tulsa

Tulsa - Creator CRM System
In recognition of Roy and Leon Daley
A legitimate creator relationship management system built in Rust for managing subscriber relationships, tracking interactions, and monitoring engagement.
Features
1. CRM Dashboard (Current)
Web GUI: Beautiful, modern interface for managing your creator business
Dashboard with real-time statistics
Subscriber management with filtering
Interaction tracking and history
Expiring subscription alerts
REST API: Full-featured API for custom integrations
CLI Tool: Command-line interface for quick operations
Subscriber Management: Store and manage subscriber information
Interaction Tracking: Record all interactions (messages, purchases, tips, renewals)
Analytics: View stats on subscribers, revenue, and engagement
Expiring Subscriptions: Track and identify subscriptions that need renewal
Coming Soon
Message Template System
Content Scheduler
Advanced Analytics Dashboard
Setup
Prerequisites
Rust 1.70 or higher
SQLite 3
Quick Start
cd creator_crm
./start.sh
This will build and start the server.
Manual Installation
Build the project:
cargo build --release
Run the server:
cargo run
Access the Application
Web GUI: Open your browser to http://127.0.0.1:3000
API Endpoint: http://127.0.0.1:3000/api
CLI Tool: Use ./crm_cli.sh for command-line access
Configuration
Set the database URL (optional):
export DATABASE_URL=sqlite://path/to/your/database.db
Default: sqlite://creator_crm.db
API Documentation
Subscribers
Create Subscriber
POST /api/subscribers
Content-Type: application/json
{
  "name": "John Doe",
  "username": "johndoe123",
  "platform": "OnlyFans",
  "tier": "Regular",
  "birthday": "1995-06-15",
  "notes": "Prefers morning posts"
}
List Subscribers
GET /api/subscribers?platform=OnlyFans&tier=VIP
Get Subscriber
GET /api/subscribers/:id
Update Subscriber
PUT /api/subscribers/:id
Content-Type: application/json
{
  "tier": "VIP",
  "total_spent": 250.00,
  "subscription_end_date": "2026-03-01T00:00:00Z",
  "notes": "Upgraded to VIP tier"
}
Delete Subscriber
DELETE /api/subscribers/:id
Interactions
Create Interaction
POST /api/interactions
Content-Type: application/json
{
  "subscriber_id": "uuid-here",
  "interaction_type": "purchase",
  "amount": 35.00,
  "notes": "Custom video request"
}
Interaction types: message, purchase, tip, renewal
Get Subscriber Interactions
GET /api/subscribers/:id/interactions
Analytics
Get Stats
GET /api/stats
Returns:
{
  "total_subscribers": 150,
  "vip_count": 25,
  "regular_count": 100,
  "casual_count": 25,
  "total_revenue": 15000.00,
  "expiring_soon": 5
}
Get Expiring Subscriptions
GET /api/expiring?days=7
Returns subscribers with subscriptions expiring within the specified days.
Usage Examples
Using curl
Add a new subscriber:
curl -X POST http://127.0.0.1:3000/api/subscribers \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jane Smith",
    "username": "janesmith",
    "platform": "OnlyFans",
    "tier": "VIP"
  }'
Record an interaction:
curl -X POST http://127.0.0.1:3000/api/interactions \
  -H "Content-Type: application/json" \
  -d '{
    "subscriber_id": "SUBSCRIBER_ID_HERE",
    "interaction_type": "tip",
    "amount": 25.00,
    "notes": "Thank you tip"
  }'
Get stats:
curl http://127.0.0.1:3000/api/stats
Find expiring subscriptions:
curl http://127.0.0.1:3000/api/expiring?days=14
Tier System
VIP: High-value subscribers (typically $500+ total spent)
Regular: Active subscribers with moderate engagement
Casual: Infrequent subscribers or lower spending
Tiers can be manually assigned or updated based on spending habits.
Best Practices
Regular Updates: Keep subscriber information current
Track All Interactions: Record every meaningful interaction for better insights
Monitor Expiring Subscriptions: Check expiring subscriptions weekly
Review Analytics: Use stats to understand your subscriber base
Personalization: Use notes and preferences fields for authentic engagement
Data Privacy
All data is stored locally in SQLite
No third-party services or external APIs
You control all subscriber information
Regular backups recommended
Development
Run tests:
cargo test
Run with logging:
RUST_LOG=debug cargo run
Future Features
Message template library
Content scheduling system
Revenue forecasting
Custom reporting
Multi-user support with authentication
Mobile app companion
License
MIT
