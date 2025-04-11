import React, { useState } from "react";
import { Link, useLocation } from "react-router-dom";
import { usePlatforms } from "../hooks/usePlatforms";
import { useConversations } from "../hooks/useConversations";
import {
    FiMessageSquare,
    FiSearch,
    FiPlus,
    FiChevronDown,
    FiChevronRight
} from "react-icons/fi";
import {
    FaTelegram,
    FaSlack,
    FaDiscord,
    FaTwitter,
    FaFacebook,
    FaWhatsapp
} from "react-icons/fa";

const Sidebar = () => {
    const location = useLocation();
    const { connectedPlatforms } = usePlatforms();
    const { conversations, isLoading } = useConversations();
    const [expandedPlatforms, setExpandedPlatforms] = useState({});
    const [searchQuery, setSearchQuery] = useState("");

    const togglePlatform = (platform) => {
        setExpandedPlatforms(prev => ({
            ...prev,
            [platform]: !prev[platform]
        }));
    };

    const getPlatformIcon = (platform) => {
        switch (platform.toLowerCase()) {
            case "telegram": return <FaTelegram className="text-blue-500" />;
            case "slack": return <FaSlack className="text-purple-700" />;
            case "discord": return <FaDiscord className="text-indigo-500" />;
            case "twitter": return <FaTwitter className="text-blue-400" />;
            case "facebook": return <FaFacebook className="text-blue-600" />;
            case "whatsapp": return <FaWhatsapp className="text-green-500" />;
            default: return <FiMessageSquare />;
        }
    };

    const filteredConversations = conversations.filter(conversation => {
        if (!searchQuery.trim()) return true;
        return conversation.name.toLowerCase().includes(searchQuery.toLowerCase());
    });

    // Group conversations by platform
    const groupedConversations = filteredConversations.reduce((acc, conversation) => {
        const platform = conversation.platform.toLowerCase();
        if (!acc[platform]) {
            acc[platform] = [];
        }
        acc[platform].push(conversation);
        return acc;
    }, {});

    return (
        <aside className="w-64 bg-gray-100 dark:bg-gray-800 overflow-y-auto flex-shrink-0 border-r border-gray-200 dark:border-gray-700">
            {/* Search bar */}
            <div className="p-4 border-b border-gray-200 dark:border-gray-700">
                <div className="relative">
                    <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <FiSearch className="text-gray-400" />
                    </div>
                    <input
                        type="text"
                        placeholder="Search conversations..."
                        className="input pl-10"
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                    />
                </div>
            </div>

            {/* Platform sections */}
            <div className="p-2">
                {isLoading ? (
                    <div className="flex justify-center p-4">
                        <div className="animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-indigo-600"></div>
                    </div>
                ) : (
                    <>
                        {/* Connected platforms */}
                        {connectedPlatforms.length > 0 ? (
                            <>
                                {connectedPlatforms.map((platform) => (
                                    <div key={platform.toLowerCase()} className="mb-2">
                                        <button
                                            onClick={() => togglePlatform(platform.toLowerCase())}
                                            className="flex items-center justify-between w-full p-2 text-left rounded-md hover:bg-gray-200 dark:hover:bg-gray-700"
                                        >
                                            <div className="flex items-center">
                                                {getPlatformIcon(platform)}
                                                <span className="ml-2 font-medium">{platform}</span>
                                            </div>
                                            {expandedPlatforms[platform.toLowerCase()] ? (
                                                <FiChevronDown className="text-gray-500" />
                                            ) : (
                                                <FiChevronRight className="text-gray-500" />
                                            )}
                                        </button>

                                        {/* Conversations for this platform */}
                                        {expandedPlatforms[platform.toLowerCase()] && groupedConversations[platform.toLowerCase()] && (
                                            <div className="ml-6 mt-1 space-y-1">
                                                {groupedConversations[platform.toLowerCase()].map((conversation) => (
                                                    <Link
                                                        key={conversation.id}
                                                        to={`/conversations/${conversation.id}`}
                                                        className={`block px-2 py-1.5 text-sm rounded-md ${location.pathname === `/conversations/${conversation.id}`
                                                                ? "bg-indigo-100 text-indigo-700 dark:bg-indigo-900 dark:text-indigo-200"
                                                                : "text-gray-700 hover:bg-gray-200 dark:text-gray-300 dark:hover:bg-gray-700"
                                                            }`}
                                                    >
                                                        {conversation.name}
                                                    </Link>
                                                ))}
                                            </div>
                                        )}
                                    </div>
                                ))}
                            </>
                        ) : (
                            <div className="text-center py-4 text-gray-500 dark:text-gray-400">
                                <p>No platforms connected yet</p>
                            </div>
                        )}

                        {/* Connect new platform button */}
                        <Link
                            to="/settings"
                            className="flex items-center justify-center w-full mt-4 p-2 bg-indigo-50 text-indigo-600 rounded-md hover:bg-indigo-100 dark:bg-indigo-900 dark:text-indigo-300 dark:hover:bg-indigo-800"
                        >
                            <FiPlus className="mr-2" />
                            <span>Connect Platform</span>
                        </Link>
                    </>
                )}
            </div>
        </aside>
    );
};

export default Sidebar;