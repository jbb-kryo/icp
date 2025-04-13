OC Messagr Frontend
===================

A cross-platform messaging aggregator frontend for the Internet Computer Protocol (ICP), allowing you to connect and query your conversations across Telegram, Slack, Discord, Twitter, Facebook, and WhatsApp.

Features
--------

-   **Unified Interface**: Access all your messaging platforms in one place
-   **AI-Powered Search**: Ask natural language questions about your conversations
-   **Cross-Platform Querying**: Find messages across different platforms with a single query
-   **Conversation Analytics**: Get AI-generated insights about your conversations
-   **Platform Management**: Connect, sync, and manage different messaging platforms
-   **Advanced Filtering**: Filter messages by platform, time, content type, and more
-   **Dark/Light Mode**: Customizable interface theme

Prerequisites
-------------

-   [Node.js](https://nodejs.org/) (v16 or later)
-   [npm](https://www.npmjs.com/) (v7 or later)
-   [dfx](https://internetcomputer.org/docs/current/developer-tools/install) (v0.15.0 or later)
-   [Rust](https://www.rust-lang.org/tools/install) (for backend development)

Quick Start
-----------

### 1\. Clone the repository

bash

```
git clone https://github.com/yourusername/oc-messagr.git
cd oc-messagr/messagr_frontend
```

### 2\. Install dependencies

bash

```
npm install
```

### 3\. Configure environment

Create a `.env` file in the root of the frontend directory:

```
DFX_NETWORK=local
MESSAGR_APP_CANISTER_ID=your_backend_canister_id
OPENCHAT_CANISTER_ID=xomae-vyaaa-aaaaq-aabhq-cai
```

### 4\. Start the development server

bash

```
npm start
```

Visit `http://localhost:8080` in your browser.

Building for Production
-----------------------

### 1\. Build the frontend assets

bash

```
npm run build
```

### 2\. Deploy to the Internet Computer

bash

```
dfx deploy --network ic messagr_frontend
```

Connecting to a Backend Canister
--------------------------------

The frontend requires a deployed instance of the Messagr backend canister. You can either:

1.  Deploy your own backend (see the main repository README)
2.  Connect to an existing backend by updating the `MESSAGR_APP_CANISTER_ID` environment variable

To update the canister ID after deployment:

bash

```
dfx canister call messagr_frontend update_backend_canister '("rrkah-fqaaa-aaaaa-aaaaq-cai")'
```

Project Structure
-----------------

```
messagr_frontend/
├── package.json         # Dependencies and scripts
├── dfx.json             # IC deployment configuration
├── webpack.config.js    # Build configuration
├── src/
│   ├── index.html       # Entry HTML file
│   ├── index.jsx        # Main React entry point
│   ├── assets/          # Static assets (CSS, icons, etc.)
│   ├── components/      # React components
│   ├── pages/           # Page components
│   ├── hooks/           # Custom React hooks
│   ├── context/         # React context providers
│   └── utils/           # Utility functions
└── public/              # Public static assets
```

Key Components
--------------

-   **AIEnhancedQueryInput**: Intelligent search input with AI capabilities
-   **ConversationInsights**: AI-generated analytics for conversations
-   **MessageView**: Conversation and message display
-   **PlatformSettings**: Platform connection management

Authentication
--------------

The app uses Internet Identity for authentication, which is built into the ICP ecosystem. When running locally, you'll be prompted to create or use a local development identity.

Customization
-------------

### Styling

The app uses Tailwind CSS for styling. To customize the appearance:

1.  Edit `src/assets/styles/main.css` for global styles
2.  Modify component classes for specific element styling
3.  Update theme variables in the CSS root to change color schemes

### API Configuration

Connection parameters for different platforms can be adjusted in the corresponding platform settings components.

Deployment Options
------------------

### Option 1: Deploy with the Backend

If you're also deploying the backend, you can deploy both canisters in one step:

bash

```
# From the project root
dfx deploy --network ic
```

### Option 2: Deploy Frontend Only

To deploy just the frontend to connect to an existing backend:

bash

```
# From the frontend directory
dfx deploy --network ic messagr_frontend
```

### Option 3: Static Hosting

You can also build the frontend for static hosting:

bash

```
npm run build:static
```

This generates static assets in the `dist` directory, which can be hosted on any static hosting service.

Troubleshooting
---------------

### Common Issues

1.  **Connection to backend fails**:
    -   Verify the backend canister ID is correct
    -   Ensure the backend canister is running
    -   Check browser console for network errors
2.  **Authentication issues**:
    -   Clear browser cache and cookies
    -   Try using a different browser
    -   Verify Internet Identity is working properly
3.  **Platform connection failures**:
    -   Confirm API credentials are correct
    -   Check if redirect URIs are properly configured
    -   Ensure the platform's API services are operational

Contributing
------------

Contributions are welcome! Please feel free to submit a Pull Request.

1.  Fork the repository
2.  Create your feature branch (`git checkout -b feature/amazing-feature`)
3.  Commit your changes (`git commit -m 'Add some amazing feature'`)
4.  Push to the branch (`git push origin feature/amazing-feature`)
5.  Open a Pull Request

License
-------

This project is licensed under the MIT License - see the LICENSE file for details.

Acknowledgments
---------------

-   [OpenChat SDK](https://openchat.com/sdk) for AI-powered messaging features
-   [Internet Computer](https://internetcomputer.org/) for the decentralized hosting platform
-   [DFINITY Foundation](https://dfinity.org/) for creating the Internet Computer Protocol

Contact
-------

Project Link: <https://github.com/yourusername/oc-messagr>

For support or questions, please open an issue on the GitHub repository.