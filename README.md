Tulsa - Quick Start Guide
In recognition of Roy and Leon Daley
Getting Started in 3 Minutes
1. Start the Server
cd creator_crm
./start.sh
The server will start on http://127.0.0.1:3000
2. Open the Web GUI
Open your browser and go to: **http://127.0.0.1:3000**
You'll see the Tulsa Creator CRM Dashboard with:
Dashboard: Overview of your subscriber base and revenue
Subscribers: Manage all your subscribers
Interactions: Track all interactions
Expiring Soon: Monitor subscriptions that need renewal
3. Add Your First Subscriber
Using the Web GUI:
Click "Subscribers" in the navigation
Click "+ Add Subscriber"
Fill in the details:
Name: The subscriber's name
Username: Their username on the platform
Platform: Select from OnlyFans, Instagram, Twitter, etc.
Tier: VIP, Regular, or Casual
Birthday (optional): For personalized engagement
Notes (optional): Any preferences or special info
Using the CLI:
./crm_cli.sh add-subscriber "Jane Smith" janesmith OnlyFans VIP
4. Record an Interaction
Using the Web GUI:
Click "Interactions" in the navigation
Click "+ Add Interaction"
Select the subscriber
Choose interaction type (message, purchase, tip, renewal)
Enter amount (if applicable)
Add notes (optional)
Using the CLI:
./crm_cli.sh add-interaction SUBSCRIBER_ID purchase 50.00 "Custom video request"
5. View Analytics
Go to the Dashboard to see:
Total subscribers by tier
Total revenue
Subscriptions expiring soon
Recent activity
Common Tasks
Update Subscriber Tier
When someone becomes a VIP or changes spending habits:
GUI: Click on a subscriber → Edit → Change tier
CLI:
./crm_cli.sh update-tier SUBSCRIBER_ID VIP
Check Expiring Subscriptions
GUI: Click "Expiring Soon" tab
CLI:
./crm_cli.sh expiring 14  # Next 14 days
View All Interactions for a Subscriber
GUI: Click on any subscriber card to see their full profile and interaction history
CLI:
./crm_cli.sh get-interactions SUBSCRIBER_ID
Filter Subscribers
GUI: Use the dropdown filters in the Subscribers view
CLI:
./crm_cli.sh list-subscribers OnlyFans VIP
View Overall Statistics
CLI:
./crm_cli.sh stats
Tips for Success
Track Everything: Record every meaningful interaction to build a complete picture
Use Tiers Wisely: 
VIP: $500+ lifetime spending
Regular: Active, moderate spending
Casual: Infrequent or trial subscribers
Monitor Expiring: Check weekly for subscriptions ending soon
Add Notes: Use the notes field for preferences, favorite content, etc.
Update Regularly: Keep subscriber information current
Workflow Example
Daily Routine:
Check Dashboard for overview
Review any new interactions
Check "Expiring Soon" for follow-ups needed
After Each Interaction:
Log the interaction immediately
Update notes if you learned something new
Update tier if spending pattern changes
Weekly Review:
Review expiring subscriptions
Check overall stats
Identify VIP subscribers who haven't interacted recently
Plan personalized outreach
Data Management
Backup Your Database
cp creator_crm.db creator_crm_backup_$(date +%Y%m%d).db
Change Database Location
export DATABASE_URL=sqlite:///path/to/your/database.db
./start.sh
Troubleshooting
Port Already in Use
If port 3000 is already in use, edit src/main.rs and change the port:
let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
Then rebuild: cargo build
Database Locked
Only one instance can access the database at a time. Make sure you don't have multiple servers running.
Can't Access GUI
Make sure you're using http://127.0.0.1:3000 not https://
Next Steps
Now that you have the CRM system running, you can:
Import Existing Data: Use the API or CLI to bulk import subscribers
Customize: Modify the code to fit your specific needs
Integrate: Use the API to connect with other tools
Expand: Add the message template system (coming soon)
Getting Help
Check the README.md for detailed API documentation
Review the code in src/ for customization ideas
Use ./crm_cli.sh help for CLI command reference
Security Reminder
This system stores data locally on your machine
Back up your database regularly
Don't expose the server to the internet without proper authentication
Keep your subscriber data private and secure
