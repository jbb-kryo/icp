# OpenChat Messagr Frontend
A powerful cross-platform messaging aggregator with AI-powered search capabilities built for the Internet Computer Protocol.

## Overview
OpenChat Messagr is a unified messaging interface that connects to multiple platforms (Telegram, Slack, Discord, Twitter, Facebook, and WhatsApp) and allows you to search, analyze, and organize your conversations using advanced AI capabilities powered by the OpenChat SDK.

### Features

- Cross-Platform Integration: Connect and aggregate messages from 6 major messaging platforms
- AI-Powered Search: Ask natural language questions about your conversations
- Advanced Filtering: Search by platform, time, sender, and content type
- Conversation Insights: Get AI-generated summaries, sentiment analysis, and topic extraction
- Secure & Private: Your data stays in your personal canister on the Internet Computer
- Responsive Design: Works seamlessly on desktop and mobile

## Getting Started
### Prerequisites

Node.js 16+
npm or yarn
Internet Computer SDK (dfx)
Internet Identity for authentication

### Installation

# Clone the repository:

bashgit 

clone https://github.com/openchat/messagr.git

cd messagr/messagr_frontend

### Install dependencies:

bashnpm install

Configure environment variables:
Create a .env file with:

MESSAGR_CANISTER_ID=YOUR_CANISTER_ID_HERE
OPENCHAT_CANISTER_ID=xomae-vyaaa-aaaaq-aabhq-cai

Start the development server:

bashnpm start
Connecting Platforms

Navigate to Settings → Platforms
Click "Connect" for the platform you want to add
Follow the authentication flow for that platform
Once connected, your messages will begin syncing

Usage
Basic Search
Type your query in the search bar at the top of the dashboard or conversations page:

"Show messages from Alice about the budget"
"Find conversations from last week on Slack"
"Show all WhatsApp messages with attachments"

AI-Enhanced Search
Toggle the AI button next to the search bar to enable AI-powered search:

"What was that project deadline John mentioned last month?"
"Summarize what we discussed about the marketing campaign"
"What's the overall sentiment about the new product launch?"

Conversation Insights

Open any conversation
Click the "AI Insights" button in the top right
View AI-generated insights including:

Key topics
Sentiment analysis
Important entities
Conversation timeline
Action items



Architecture
The frontend is built using:

React 18
Tailwind CSS for styling
React Router for navigation
React Query for data fetching
Internet Computer JavaScript API
OpenChat SDK for AI capabilities

Folder Structure
messagr_frontend/
├── package.json
├── src/
│   ├── components/     # Reusable UI components
│   ├── pages/          # Main application pages
│   ├── hooks/          # Custom React hooks
│   ├── context/        # React context providers
│   ├── utils/          # Utility functions
│   ├── assets/         # Static assets
│   └── declarations/   # Auto-generated IC interfaces
└── public/             # Public static files
Development
Building for Production
bashnpm run build
Deploying to the IC
bashdfx deploy --network ic
Running Tests
bashnpm test
Platform-Specific Setup
Each messaging platform requires specific API credentials. See the Setup Guide for detailed instructions on setting up each platform.
Contributing
We welcome contributions! Please see CONTRIBUTING.md for details on how to get started and our development process.
Security
All message data is stored in your personal canister on the Internet Computer. Your authentication tokens for messaging platforms are encrypted before storage. For more details, see our Security Policy.
License
This project is licensed under the MIT License - see the LICENSE file for details.
Acknowledgments

Built with OpenChat SDK
Powered by the Internet Computer Protocol
UI components inspired by Tailwind UI


For questions, support, or feature requests, please open an issue or contact us at messagr@openchat.org.