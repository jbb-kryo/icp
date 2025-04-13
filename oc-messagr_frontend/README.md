Messagr - Cross-Platform Messaging Aggregator
=============================================

Messagr is a powerful cross-platform messaging aggregator built on the Internet Computer Protocol (ICP), allowing you to connect, search, and analyze your conversations across multiple messaging platforms in one unified interface.

Features
--------

-   **Multi-Platform Integration**: Connect Telegram, Slack, Discord, Twitter, Facebook Messenger, and WhatsApp
-   **Unified Message Hub**: Access all your conversations in a single interface
-   **AI-Powered Search**: Ask natural language questions about your conversations
-   **Advanced Filtering**: Search by platform, time range, sender, content type, and more
-   **Conversation Insights**: Get AI-generated insights about conversation topics, sentiment, and key points
-   **Secure & Private**: Your data stays in your own canister on the Internet Computer
-   **Responsive Design**: Works seamlessly on desktop and mobile devices

Technology Stack
----------------

-   **Backend**: Rust deployed as an ICP canister
-   **Frontend**: React with TailwindCSS
-   **AI Integration**: OpenChat SDK for natural language processing
-   **Search**: Advanced indexing with Tantivy
-   **Authentication**: Internet Identity

Getting Started
---------------

### Prerequisites

-   [DFX SDK](https://internetcomputer.org/docs/current/developer-tools/install) (version 0.12.0 or later)
-   [Node.js](https://nodejs.org/) (version 16 or later)
-   [Rust](https://www.rust-lang.org/tools/install) (version 1.58 or later)
-   Internet Computer identity with cycles

### Local Development Setup

1.  Clone the repository:

bash

```
git clone https://github.com/yourusername/messagr.git
cd messagr
```

1.  Install dependencies:

bash

```
# Install frontend dependencies
cd messagr_frontend
npm install

# Install Rust dependencies
cd ../messagr_app
cargo build
```

1.  Start the local IC replica:

bash

```
dfx start --clean --background
```

1.  Deploy the canisters locally:

bash

```
dfx deploy
```

1.  Start the frontend development server:

bash

```
cd messagr_frontend
npm start
```

1.  Open your browser and navigate to `http://localhost:8080`

### Connecting Platforms

To connect messaging platforms, you'll need API credentials for each service:

1.  **Telegram**: Get a bot token from [BotFather](https://t.me/BotFather)
2.  **Slack**: Create an app at [api.slack.com](https://api.slack.com/apps)
3.  **Discord**: Register an app on the [Discord Developer Portal](https://discord.com/developers/applications)
4.  **Twitter**: Apply for API access at [developer.twitter.com](https://developer.twitter.com/)
5.  **Facebook**: Create an app at [developers.facebook.com](https://developers.facebook.com/)
6.  **WhatsApp**: Set up WhatsApp Business API integration

Detailed instructions for each platform are available in the [Setup Guide](docs/platform-setup.md).

Deployment
----------

### Deploying to the Internet Computer

1.  Build the project for production:

bash

```
dfx build --network ic
```

1.  Deploy to the IC mainnet:

bash

```
dfx deploy --network ic
```

1.  Fund your canisters with cycles:

bash

```
dfx ledger --network ic top-up $(dfx canister --network ic id messagr_app) --amount 1
dfx ledger --network ic top-up $(dfx canister --network ic id messagr_frontend) --amount 0.5
```

1.  Access your deployed application at:

```
https://<frontend_canister_id>.ic0.app
```

Usage Guide
-----------

### Basic Navigation

-   **Dashboard**: Overview of connected platforms and recent activity
-   **Conversations**: Browse all conversations across platforms
-   **Settings**: Connect platforms and configure application preferences

### AI-Powered Search

Click the AI toggle in the search bar to enable intelligent queries like:

-   "Find messages about project deadlines from Slack last week"
-   "Show me all attachments shared in WhatsApp conversations with Alice"
-   "What did the marketing team discuss about the new campaign on Discord?"

### Conversation Insights

In any conversation, click the "AI Insights" button to view:

-   Key entities and topics
-   Sentiment analysis
-   Conversation timeline
-   Key decisions and action items

Maintenance
-----------

### Managing Indices

To maintain optimal search performance:

1.  Optimize indices regularly:

bash

```
dfx canister --network ic call messagr_app optimize_indices
```

1.  Rebuild indices if search results become inconsistent:

bash

```
dfx canister --network ic call messagr_app rebuild_indices
```

### Monitoring Cycle Usage

Check your canister cycle balance:

bash

```
dfx canister --network ic status messagr_app
```

Architecture
------------

The application consists of two main canisters:

1.  **messagr_app (Rust)**: Backend with platform connectors, message storage, and intelligent querying
2.  **messagr_frontend (React)**: User interface for interacting with the backend

Key components include:

-   **Platform Connectors**: Integrate with messaging service APIs
-   **Storage System**: Efficiently stores messages and conversations
-   **Indexing Engine**: Enables fast search across large message volumes
-   **OpenChat Integration**: Provides AI-powered search and insights

Contributing
------------

We welcome contributions! Please see <CONTRIBUTING.md> for guidelines.

License
-------

This project is licensed under the MIT License - see the <LICENSE> file for details.

Acknowledgments
---------------

-   [OpenChat](https://oc.app) for their AI capabilities
-   [DFINITY](https://dfinity.org) for the Internet Computer Protocol
-   All the messaging platforms for their APIs

Support
-------

If you encounter any issues or have questions, please:

-   Check the [FAQ](docs/faq.md)
-   Submit an issue on GitHub
-   Join our community on [Discord](https://discord.gg/messagr)

* * * * *

Built with ❤️ on the Internet Computer