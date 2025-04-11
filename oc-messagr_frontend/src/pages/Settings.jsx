import React, { useState } from "react";
import { Link } from "react-router-dom";
import { usePlatforms } from "../hooks/usePlatforms";
import { useTheme } from "../context/ThemeContext";
import AdminPanel from "../components/AdminPanel";
import {
    FiCheck,
    FiX,
    FiSettings,
    FiRefreshCw,
    FiLogOut,
    FiSun,
    FiMoon,
} from "react-icons/fi";
import {
    FaTelegram,
    FaSlack,
    FaDiscord,
    FaTwitter,
    FaFacebook,
    FaWhatsapp,
} from "react-icons/fa";

const Settings = () => {
    const { connectedPlatforms, disconnectPlatform, syncMessages } = usePlatforms();
    const { theme, toggleTheme } = useTheme();
    const [syncing, setSyncing] = useState({});
    const [activeTab, setActiveTab] = useState("platforms");

    const platforms = [
        {
            id: "telegram",
            name: "Telegram",
            icon: FaTelegram,
            color: "blue",
            description: "Connect to Telegram to retrieve and query your messages.",
        },
        {
            id: "slack",
            name: "Slack",
            icon: FaSlack,
            color: "purple",
            description: "Connect to Slack workspaces and channels.",
        },
        {
            id: "discord",
            name: "Discord",
            icon: FaDiscord,
            color: "indigo",
            description: "Connect to Discord servers and DMs.",
        },
        {
            id: "twitter",
            name: "Twitter",
            icon: FaTwitter,
            color: "blue",
            description: "Connect to Twitter DMs and mentions.",
        },
        {
            id: "facebook",
            name: "Facebook",
            icon: FaFacebook,
            color: "blue",
            description: "Connect to Facebook Messenger conversations.",
        },
        {
            id: "whatsapp",
            name: "WhatsApp",
            icon: FaWhatsapp,
            color: "green",
            description: "Connect to WhatsApp chats via WhatsApp Business API.",
        },
    ];

    const handleSync = async (platform) => {
        setSyncing({ ...syncing, [platform.toLowerCase()]: true });
        try {
            await syncMessages(platform);
        } catch (error) {
            console.error(`Error syncing ${platform} messages:`, error);
        } finally {
            setSyncing({ ...syncing, [platform.toLowerCase()]: false });
        }
    };

    const handleDisconnect = async (platform) => {
        try {
            await disconnectPlatform(platform);
        } catch (error) {
            console.error(`Error disconnecting ${platform}:`, error);
        }
    };

    return (
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white mb-6">Settings</h1>

            {/* Tabs */}
            <div className="border-b border-gray-200 dark:border-gray-700 mb-6">
                <nav className="-mb-px flex space-x-8">
                    <button
                        onClick={() => setActiveTab("platforms")}
                        className={`${activeTab === "platforms"
                                ? "border-indigo-500 text-indigo-600 dark:text-indigo-400"
                                : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300 dark:hover:border-gray-500"
                            } whitespace-nowrap pb-4 px-1 border-b-2 font-medium`}
                    >
                        Platforms
                    </button>
                    <button
                        onClick={() => setActiveTab("indexing")}
                        className={`${activeTab === "indexing"
                                ? "border-indigo-500 text-indigo-600 dark:text-indigo-400"
                                : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300 dark:hover:border-gray-500"
                            } whitespace-nowrap pb-4 px-1 border-b-2 font-medium`}
                    >
                        Indexing
                    </button>
                    <button
                        onClick={() => setActiveTab("preferences")}
                        className={`${activeTab === "preferences"
                                ? "border-indigo-500 text-indigo-600 dark:text-indigo-400"
                                : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300 dark:hover:border-gray-500"
                            } whitespace-nowrap pb-4 px-1 border-b-2 font-medium`}
                    >
                        Preferences
                    </button>
                    <button
                        onClick={() => setActiveTab("account")}
                        className={`${activeTab === "account"
                                ? "border-indigo-500 text-indigo-600 dark:text-indigo-400"
                                : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-400 dark:hover:text-gray-300 dark:hover:border-gray-500"
                            } whitespace-nowrap pb-4 px-1 border-b-2 font-medium`}
                    >
                        Account
                    </button>
                </nav>
            </div>

            {/* Platforms Tab */}
            {activeTab === "platforms" && (
                <div>
                    <h2 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                        Connected Platforms
                    </h2>
                    <div className="space-y-4">
                        {platforms.map((platform) => {
                            const isConnected = connectedPlatforms.includes(platform.name);

                            return (
                                <div
                                    key={platform.id}
                                    className="card"
                                >
                                    <div className="p-4 sm:p-6">
                                        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between">
                                            <div className="flex items-center mb-4 sm:mb-0">
                                                <platform.icon className={`h-10 w-10 text-${platform.color}-500`} />
                                                <div className="ml-4">
                                                    <h3 className="text-lg font-medium text-gray-900 dark:text-white">
                                                        {platform.name}
                                                    </h3>
                                                    <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                                        {platform.description}
                                                    </p>
                                                </div>
                                            </div>
                                            <div className="flex items-center">
                                                {isConnected && (
                                                    <button
                                                        onClick={() => handleSync(platform.name)}
                                                        disabled={syncing[platform.id.toLowerCase()]}
                                                        className="mr-3 btn btn-secondary"
                                                    >
                                                        {syncing[platform.id.toLowerCase()] ? (
                                                            <div className="flex items-center">
                                                                <div className="animate-spin h-4 w-4 border-2 border-indigo-600 border-t-transparent rounded-full mr-2"></div>
                                                                <span>Syncing</span>
                                                            </div>
                                                        ) : (
                                                            <>
                                                                <FiRefreshCw className="mr-2" />
                                                                <span>Sync</span>
                                                            </>
                                                        )}
                                                    </button>
                                                )}

                                                {isConnected ? (
                                                    <button
                                                        onClick={() => handleDisconnect(platform.name)}
                                                        className="btn btn-danger"
                                                    >
                                                        <FiLogOut className="mr-2" />
                                                        <span>Disconnect</span>
                                                    </button>
                                                ) : (
                                                    <Link
                                                        to={`/connect/${platform.id}`}
                                                        className="btn btn-primary"
                                                    >
                                                        <FiSettings className="mr-2" />
                                                        <span>Connect</span>
                                                    </Link>
                                                )}
                                            </div>
                                        </div>

                                        {isConnected && (
                                            <div className="mt-4 flex items-center text-sm text-gray-500 dark:text-gray-400">
                                                <FiCheck className="mr-2 text-green-500" />
                                                <span>Connected</span>
                                            </div>
                                        )}
                                    </div>
                                </div>
                            );
                        })}
                    </div>
                </div>
            )}

            {/* Indexing Tab */}
            {activeTab === "indexing" && (
                <div>
                    <h2 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                        Search Index Management
                    </h2>
                    <div className="card max-w-4xl">
                        <div className="p-0">
                            <AdminPanel />
                        </div>
                    </div>

                    <div className="mt-6">
                        <h2 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                            Advanced Search Settings
                        </h2>
                        <div className="card">
                            <div className="p-6">
                                <div className="mb-6">
                                    <h3 className="text-md font-medium text-gray-900 dark:text-white mb-2">
                                        Natural Language Processing
                                    </h3>
                                    <div className="flex items-center justify-between">
                                        <div>
                                            <p className="text-sm text-gray-500 dark:text-gray-400">
                                                Enable advanced text analysis for better search results
                                            </p>
                                        </div>
                                        <div className="ml-4">
                                            <label className="inline-flex items-center cursor-pointer">
                                                <input type="checkbox" value="" className="sr-only peer" checked />
                                                <div className="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
                                            </label>
                                        </div>
                                    </div>
                                </div>

                                <div className="mb-6">
                                    <h3 className="text-md font-medium text-gray-900 dark:text-white mb-2">
                                        Automatic Indexing
                                    </h3>
                                    <div className="flex items-center justify-between">
                                        <div>
                                            <p className="text-sm text-gray-500 dark:text-gray-400">
                                                Automatically index new messages as they're received
                                            </p>
                                        </div>
                                        <div className="ml-4">
                                            <label className="inline-flex items-center cursor-pointer">
                                                <input type="checkbox" value="" className="sr-only peer" checked />
                                                <div className="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
                                            </label>
                                        </div>
                                    </div>
                                </div>

                                <div>
                                    <h3 className="text-md font-medium text-gray-900 dark:text-white mb-2">
                                        Index Performance
                                    </h3>
                                    <div className="mb-4">
                                        <label className="block text-sm text-gray-700 dark:text-gray-300 mb-1">
                                            Memory usage for indexing
                                        </label>
                                        <select className="input">
                                            <option value="low">Low (256MB)</option>
                                            <option value="medium" selected>Medium (512MB)</option>
                                            <option value="high">High (1GB)</option>
                                            <option value="very-high">Very High (2GB+)</option>
                                        </select>
                                        <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                                            Higher memory usage improves search performance but requires more canister cycles
                                        </p>
                                    </div>

                                    <div>
                                        <label className="block text-sm text-gray-700 dark:text-gray-300 mb-1">
                                            Auto-optimization schedule
                                        </label>
                                        <select className="input">
                                            <option value="never">Never</option>
                                            <option value="daily">Daily</option>
                                            <option value="weekly" selected>Weekly</option>
                                            <option value="monthly">Monthly</option>
                                        </select>
                                        <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                                            How often to automatically optimize indices for best performance
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            )}

            {/* Preferences Tab */}
            {activeTab === "preferences" && (
                <div>
                    <h2 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                        Application Preferences
                    </h2>
                    <div className="card">
                        <div className="p-6">
                            <div className="flex items-center justify-between">
                                <div>
                                    <h3 className="text-md font-medium text-gray-900 dark:text-white">
                                        Theme
                                    </h3>
                                    <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                        Choose between light and dark mode
                                    </p>
                                </div>
                                <div className="ml-4">
                                    <button
                                        onClick={toggleTheme}
                                        className="btn btn-secondary"
                                    >
                                        {theme === "dark" ? (
                                            <>
                                                <FiSun className="mr-2" />
                                                <span>Light Mode</span>
                                            </>
                                        ) : (
                                            <>
                                                <FiMoon className="mr-2" />
                                                <span>Dark Mode</span>
                                            </>
                                        )}
                                    </button>
                                </div>
                            </div>

                            <div className="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700">
                                <div className="flex items-center justify-between">
                                    <div>
                                        <h3 className="text-md font-medium text-gray-900 dark:text-white">
                                            Notifications
                                        </h3>
                                        <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                            Receive notifications for new messages
                                        </p>
                                    </div>
                                    <div className="ml-4">
                                        <label className="inline-flex items-center cursor-pointer">
                                            <input type="checkbox" value="" className="sr-only peer" />
                                            <div className="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
                                        </label>
                                    </div>
                                </div>
                            </div>

                            <div className="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700">
                                <div className="flex items-center justify-between">
                                    <div>
                                        <h3 className="text-md font-medium text-gray-900 dark:text-white">
                                            Auto-sync
                                        </h3>
                                        <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                            Automatically sync messages on login
                                        </p>
                                    </div>
                                    <div className="ml-4">
                                        <label className="inline-flex items-center cursor-pointer">
                                            <input type="checkbox" value="" className="sr-only peer" checked />
                                            <div className="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 dark:peer-focus:ring-indigo-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-indigo-600"></div>
                                        </label>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            )}

            {/* Account Tab */}
            {activeTab === "account" && (
                <div>
                    <h2 className="text-lg font-medium text-gray-900 dark:text-white mb-4">
                        Account Settings
                    </h2>
                    <div className="card">
                        <div className="p-6">
                            <div className="flex items-center justify-between">
                                <div>
                                    <h3 className="text-md font-medium text-gray-900 dark:text-white">
                                        Internet Identity
                                    </h3>
                                    <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                        Your account is authenticated via Internet Identity
                                    </p>
                                </div>
                                <div className="ml-4">
                                    <button
                                        className="btn btn-danger"
                                    >
                                        <FiLogOut className="mr-2" />
                                        <span>Logout</span>
                                    </button>
                                </div>
                            </div>

                            <div className="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700">
                                <div>
                                    <h3 className="text-md font-medium text-gray-900 dark:text-white">
                                        Data Storage
                                    </h3>
                                    <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                        Your data is stored securely in your personal canister on the Internet Computer blockchain
                                    </p>
                                    <div className="mt-4">
                                        <button className="btn btn-secondary">
                                            Export Data
                                        </button>
                                    </div>
                                </div>
                            </div>

                            <div className="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700">
                                <div>
                                    <h3 className="text-md font-medium text-gray-900 dark:text-white text-red-600 dark:text-red-400">
                                        Danger Zone
                                    </h3>
                                    <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                        Delete your account and all associated data
                                    </p>
                                    <div className="mt-4">
                                        <button className="btn btn-danger">
                                            <FiX className="mr-2" />
                                            <span>Delete Account</span>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default Settings;