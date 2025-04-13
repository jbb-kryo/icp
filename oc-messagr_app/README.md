Messagr: Cross-Platform Message Aggregator for ICP
==================================================

Messagr is a Rust-based application deployed as an Internet Computer Protocol (ICP) canister that allows you to connect multiple messaging platforms (Telegram, Slack, Discord, Twitter, Facebook Messenger, and WhatsApp) and intelligently query your conversations across all platforms.

Features
--------

-   **Multi-Platform Integration**: Connect and sync messages from Telegram, Slack, Discord, Twitter, Facebook, and WhatsApp
-   **Unified Message Storage**: All your messages in one secure place on the Internet Computer
-   **Intelligent Querying**: Ask natural language questions about your conversations
-   **Cross-Platform Search**: Find messages across all your platforms with a single query
-   **Privacy-Focused**: Your data stays in your canister, under your control
-   **ICP Native**: Built for the Internet Computer using Rust
-   **AI-Powered Analysis**: Leverage OpenChat SDK for intelligent conversation insights
-   **Advanced Indexing**: Sophisticated search and filtering capabilities

Architecture
------------

The application is structured as a Rust canister for ICP with the following components:

-   **Auth Modules**: Platform-specific authentication handlers
-   **Connectors**: API clients for each messaging platform
-   **Storage**: Stable memory storage for messages and conversations
-   **Query Engine**: Natural language processing for intelligent queries
-   **OpenChat SDK Integration**: Leveraging ICP's native chat functionality
-   **Advanced Indexing**: Efficient search and retrieval system
-   **React Frontend**: Modern, responsive user interface

Installation
------------

### Prerequisites

-   [DFX SDK](https://internetcomputer.org/docs/current/developer-tools/install) for Internet Computer development
-   Rust and Cargo
-   Node.js and npm (for frontend development)

### Setup

1.  Clone the repository:

bash

```
git clone https://github.com/yourusername/messagr.git
cd messagr
```

1.  Install dependencies:

bash

```
npm install
```

1.  Deploy the canister to the local network:

bash

```
dfx start --background
dfx deploy
```

1.  Deploy to the IC mainnet:

bash

```
dfx deploy --network ic
```

Using Messagr
-------------

### Connecting Platforms

To connect a messaging platform, you'll need to:

1.  Obtain API credentials for the platform
2.  Call the `connect_platform` method with the appropriate auth config
3.  Authorize the application via OAuth (where applicable)

Example for connecting Telegram:

javascript

```
// Using the IC agent
await agent.call("messagr_app", "connect_platform", {
  platform: { Telegram: null },
  token: "your-telegram-bot-token",
  api_key: null,
  api_secret: null,
  redirect_uri: null
});
```

### Syncing Messages

To sync messages from a connected platform:

javascript

```
await agent.call("messagr_app", "sync_messages", { Telegram: null });
```

### Querying Conversations

The power of Messagr comes from its ability to query across platforms:

javascript

```
// Basic query
const result = await agent.query("messagr_app", "query_conversations", "find messages about project deadlines from last week");

// AI-enhanced query
const aiResult = await agent.query("messagr_app", "ai_enhanced_query", "What did Alice say about the budget in our last meeting?");

// Platform-specific AI query
const platformResult = await agent.query("messagr_app", "ai_query_platform", [
  "find all attachments shared by John",
  { Slack: null }
]);
```

Platform Integration Details
----------------------------

### Telegram

-   Uses Bot API
-   Requires a bot token from BotFather

### Slack

-   Uses OAuth 2.0 and Slack API
-   Requires App credentials from Slack API console

### Discord

-   Uses OAuth 2.0 and Discord Bot API
-   Requires Bot token and application credentials

### Twitter

-   Uses OAuth 1.0a
-   Requires API key, API secret, and access tokens

### Facebook Messenger

-   Uses Graph API and webhooks
-   Requires Page Access Token and App credentials

### WhatsApp

-   Uses WhatsApp Business API
-   Requires Business Account and API credentials

Development
-----------

### Project Structure

```
messagr_app/
├── Cargo.toml
├── dfx.json
├── src/
│   ├── messagr_app/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs             # Main canister code
│   │       ├── auth/              # Authentication modules
│   │       ├── connectors/        # API connectors
│   │       ├── storage/           # Data storage and indexing
│   │       ├── query/             # Query processing
│   │       ├── indexing/          # Advanced search indexing
│   │       └── openchat/          # OpenChat SDK integration
│   └── declarations/              # Auto-generated canister interfaces
├── messagr_frontend/              # React frontend
└── README.md
```

### Adding a New Platform

To add support for a new messaging platform:

1.  Create new authentication module in `src/messagr_app/src/auth/`
2.  Create new connector in `src/messagr_app/src/connectors/`
3.  Add the platform to the `Platform` enum in `lib.rs`
4.  Update the Candid interface in `messagr_app.did`

Advanced Features
-----------------

### AI-Enhanced Queries

The OpenChat SDK integration enables sophisticated natural language understanding:

javascript

```
// Topic analysis
const analysis = await agent.query("messagr_app", "analyze_topic", "quarterly budget");

// Generate conversation insights
const insights = await agent.call("messagr_app", "generate_conversation_insights", "conversation_id_here");
```

### Advanced Indexing

The sophisticated indexing system supports complex queries:

javascript

```
// Advanced search with filters
const results = await agent.query("messagr_app", "advanced_search", [
  "project deadline",          // query
  { Slack: null },             // platform
  [1672531200000],             // start_time (millis)
  [1672704000000],             // end_time (millis)
  [],                          // conversation_id
  [],                          // sender_id
  [true],                      // has_attachments
  [],                          // attachment_type
  [],                          // is_reply
  [],                          // in_thread
  [],                          // is_edited
  "timestamp",                 // sort_by
  "desc",                      // sort_direction
  [50],                        // limit
  [0]                          // offset
]);

// Optimize indices for better performance
await agent.call("messagr_app", "optimize_indices");

// Get index statistics
const stats = await agent.query("messagr_app", "get_index_stats");
```

Security Considerations
-----------------------

-   All access tokens are stored in the canister's stable memory
-   Platform API requests are made via canister outbound HTTP calls
-   OAuth flows require proper redirect URI configuration
-   Users should review platform-specific permissions

Limitations
-----------

-   ICP canisters have limitations for HTTP outbound calls
-   Some platforms may require external relays for webhooks
-   Real-time updates are subject to canister update cycles

Contributing
------------

Contributions are welcome! Please feel free to submit a Pull Request.

License
-------

This project is licensed under the MIT License - see the LICENSE file for details.

Acknowledgements
----------------

-   [OpenChat SDK](https://github.com/open-chat-labs/open-chat) for AI capabilities
-   [Tantivy](https://github.com/quickwit-oss/tantivy) for text indexing
-   [Internet Computer](https://internetcomputer.org/) for the blockchain platform