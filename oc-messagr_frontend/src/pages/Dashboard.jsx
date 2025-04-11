import React from "react";
import { Link } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";
import { usePlatforms } from "../hooks/usePlatforms";
import AIEnhancedQueryInput from "../components/AIEnhancedQueryInput";
import {
    FiMessageCircle,
    FiSearch,
    FiSettings,
    FiPlusCircle,
    FiZap
} from "react-icons/fi";
import {
    FaTelegram,
    FaSlack,
    FaDiscord,
    FaTwitter,
    FaFacebook,
    FaWhatsapp
} from "react-icons/fa";

const Dashboard = () => {
    const { isAuthenticated } = useAuth();
    const { connectedPlatforms } = usePlatforms();

    // Platform data for the connection cards
    const platforms = [
        { id: "telegram", name: "Telegram", icon: FaTelegram, color: "blue" },
        { id: "slack", name: "Slack", icon: FaSlack, color: "purple" },
        { id: "discord", name: "Discord", icon: FaDiscord, color: "indigo" },
        { id: "twitter", name: "Twitter", icon: FaTwitter, color: "blue" },
        { id: "facebook", name: "Facebook", icon: FaFacebook, color: "blue" },
        { id: "whatsapp", name: "WhatsApp", icon: FaWhatsapp, color: "green" },
    ];

    if (!isAuthenticated) {
        return (
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div className="text-center">
                    <h1 className="text-4xl font-extrabold text-gray-900 dark:text-white sm:text-5xl sm:tracking-tight lg:text-6xl">
                        All your messages in one place
                    </h1>
                    <p className="mt-5 max-w-xl mx-auto text-xl text-gray-500 dark:text-gray-300">
                        Connect and query your conversations from Telegram, Slack, Discord, Twitter, Facebook, and WhatsApp.
                    </p>
                    <div className="mt-8 flex justify-center">
                        <div className="inline-flex rounded-md shadow">
                            <button
                                onClick={() => { }}
                                className="btn btn-primary px-5 py-3 text-base font-medium"
                            >
                                Get Started
                            </button>
                        </div>
                        <div className="ml-3 inline-flex">
                            <a
                                href="https://github.com/yourusername/messagr"
                                className="btn btn-secondary px-5 py-3 text-base font-medium"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                Learn More
                            </a>
                        </div>
                    </div>
                </div>

                <div className="mt-16">
                    <h2 className="text-2xl font-bold text-center mb-8">
                        Supported Platforms
                    </h2>
                    <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
                        {platforms.map((platform) => (
                            <div
                                key={platform.id}
                                className="bg-white dark:bg-gray-800 rounded-lg shadow p-6 text-center"
                            >
                                <platform.icon className={`mx-auto h-12 w-12 text-${platform.color}-500`} />
                                <h3 className="mt-4 text-lg font-medium text-gray-900 dark:text-white">
                                    {platform.name}
                                </h3>
                            </div>
                        ))}
                    </div>
                </div>

                <div className="mt-16">
                    <h2 className="text-2xl font-bold text-center mb-8">
                        How It Works
                    </h2>
                    <div className="grid md:grid-cols-3 gap-8">
                        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                            <div className="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white mx-auto">
                                <FiPlusCircle className="h-6 w-6" />
                            </div>
                            <h3 className="mt-4 text-lg font-medium text-center text-gray-900 dark:text-white">
                                Connect Your Platforms
                            </h3>
                            <p className="mt-2 text-base text-gray-500 dark:text-gray-300 text-center">
                                Link your messaging accounts from Telegram, Slack, Discord, Twitter, Facebook, and WhatsApp.
                            </p>
                        </div>
                        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                            <div className="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white mx-auto">
                                <FiMessageCircle className="h-6 w-6" />
                            </div>
                            <h3 className="mt-4 text-lg font-medium text-center text-gray-900 dark:text-white">
                                Sync Your Messages
                            </h3>
                            <p className="mt-2 text-base text-gray-500 dark:text-gray-300 text-center">
                                We securely store your conversations in your own personal canister on the Internet Computer.
                            </p>
                        </div>
                        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                            <div className="flex items-center justify-center h-12 w-12 rounded-md bg-indigo-500 text-white mx-auto">
                                <FiSearch className="h-6 w-6" />
                            </div>
                            <h3 className="mt-4 text-lg font-medium text-center text-gray-900 dark:text-white">
                                Search Across Platforms
                            </h3>
                            <p className="mt-2 text-base text-gray-500 dark:text-gray-300 text-center">
                                Ask questions and find messages across all your platforms with our powerful AI-powered search.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        );
    }

    return (
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white">Dashboard</h1>

            {/* Query Input */}
            <div className="mt-6">
                <AIEnhancedQueryInput useAIQuery={true} />
            </div>

            {/* Platform Status */}
            <div className="mt-8">
                <h2 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                    Connected Platforms
                </h2>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {platforms.map((platform) => {
                        const isConnected = connectedPlatforms.includes(platform.name);

                        return (
                            <div
                                key={platform.id}
                                className={`card ${isConnected
                                        ? `border-l-4 border-${platform.color}-500`
                                        : ""
                                    }`}
                            >
                                <div className="p-4 flex items-center justify-between">
                                    <div className="flex items-center">
                                        <platform.icon className={`h-8 w-8 text-${platform.color}-500`} />
                                        <div className="ml-3">
                                            <h3 className="text-md font-medium text-gray-900 dark:text-white">
                                                {platform.name}
                                            </h3>
                                            <p className="text-sm text-gray-500 dark:text-gray-400">
                                                {isConnected ? "Connected" : "Not connected"}
                                            </p>
                                        </div>
                                    </div>
                                    <div>
                                        {isConnected ? (
                                            <Link
                                                to={`/conversations?platform=${platform.id}`}
                                                className="btn btn-secondary btn-sm"
                                            >
                                                View
                                            </Link>
                                        ) : (
                                            <Link
                                                to={`/connect/${platform.id}`}
                                                className="btn btn-primary btn-sm"
                                            >
                                                Connect
                                            </Link>
                                        )}
                                    </div>
                                </div>
                            </div>
                        );
                    })}
                </div>
            </div>

            {/* Recent Activity */}
            <div className="mt-8">
                <h2 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                    Recent Activity
                </h2>
                <div className="card">
                    <div className="p-6">
                        {connectedPlatforms.length > 0 ? (
                            <div className="text-center py-8">
                                <FiSearch className="mx-auto h-12 w-12 text-gray-400" />
                                <h3 className="mt-2 text-md font-medium text-gray-900 dark:text-white">
                                    Try asking a question
                                </h3>
                                <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                                    Use the search bar above to query your messages across platforms.
                                </p>
                                <div className="mt-4">
                                    <p className="text-sm text-gray-700 dark:text-gray-300">
                                        Example: "Find messages about project deadlines from last week"
                                    </p>
                                </div>
                            </div>
                        ) : (
                            <div className="text-center py-8">
                                <FiSettings className="mx-auto h-12 w-12 text-gray-400" />
                                <h3 className="mt-2 text-md font-medium text-gray-900 dark:text-white">
                                    Connect your first platform
                                </h3>
                                <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                                    Start by connecting at least one messaging platform.
                                </p>
                                <div className="mt-6">
                                    <Link to="/settings" className="btn btn-primary">
                                        Go to Settings
                                    </Link>
                                </div>
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Dashboard;