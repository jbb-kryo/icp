import React, { useState, useEffect } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";
import { useConversations } from "../hooks/useConversations";
import { usePlatforms } from "../hooks/usePlatforms";
import { format } from "date-fns";
import {
    FiSearch,
    FiFilter,
    FiChevronDown,
    FiChevronUp,
    FiMessageCircle,
    FiUsers,
    FiCalendar,
    FiZap
} from "react-icons/fi";
import {
    FaTelegram,
    FaSlack,
    FaDiscord,
    FaTwitter,
    FaFacebook,
    FaWhatsapp,
} from "react-icons/fa";
import AIEnhancedQueryInput from "../components/AIEnhancedQueryInput";

const Conversations = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const { conversations, isLoading } = useConversations();
    const { connectedPlatforms } = usePlatforms();
    const [searchTerm, setSearchTerm] = useState("");
    const [selectedPlatform, setSelectedPlatform] = useState("");
    const [sortBy, setSortBy] = useState("lastActivity");
    const [sortDirection, setSortDirection] = useState("desc");
    const [filterMenuOpen, setFilterMenuOpen] = useState(false);
    const [queryResults, setQueryResults] = useState(null);

    useEffect(() => {
        // Check if there are query results in location state
        if (location.state?.queryResults) {
            setQueryResults(location.state.queryResults);
        }

        // Check for platform filter in URL params
        const params = new URLSearchParams(location.search);
        const platformParam = params.get("platform");
        if (platformParam) {
            setSelectedPlatform(platformParam);
        }
    }, [location]);

    const getPlatformIcon = (platform) => {
        switch (platform.toLowerCase()) {
            case "telegram":
                return <FaTelegram className="text-blue-500" />;
            case "slack":
                return <FaSlack className="text-purple-700" />;
            case "discord":
                return <FaDiscord className="text-indigo-500" />;
            case "twitter":
                return <FaTwitter className="text-blue-400" />;
            case "facebook":
                return <FaFacebook className="text-blue-600" />;
            case "whatsapp":
                return <FaWhatsapp className="text-green-500" />;
            default:
                return <FiMessageCircle />;
        }
    };

    const formatDate = (timestamp) => {
        if (!timestamp) return "No activity";
        return format(new Date(timestamp), "MMM d, yyyy h:mm a");
    };

    const toggleSortDirection = () => {
        setSortDirection(sortDirection === "asc" ? "desc" : "asc");
    };

    const handleSortChange = (value) => {
        if (sortBy === value) {
            toggleSortDirection();
        } else {
            setSortBy(value);
            setSortDirection("desc");
        }
    };

    const filteredConversations = conversations.filter((conversation) => {
        const matchesPlatform =
            selectedPlatform === "" ||
            conversation.platform.toLowerCase() === selectedPlatform.toLowerCase();
        const matchesSearch =
            searchTerm === "" ||
            conversation.name.toLowerCase().includes(searchTerm.toLowerCase());
        return matchesPlatform && matchesSearch;
    });

    const sortedConversations = [...filteredConversations].sort((a, b) => {
        const multiplier = sortDirection === "asc" ? 1 : -1;

        switch (sortBy) {
            case "name":
                return multiplier * a.name.localeCompare(b.name);
            case "platform":
                return multiplier * a.platform.localeCompare(b.platform);
            case "participants":
                return multiplier * (a.participants.length - b.participants.length);
            case "lastActivity":
            default:
                const aTime = a.last_message_at || 0;
                const bTime = b.last_message_at || 0;
                return multiplier * (aTime - bTime);
        }
    });

    const clearQueryResults = () => {
        setQueryResults(null);
        navigate("/conversations", { replace: true });
    };

    return (
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <div className="flex items-center justify-between mb-6">
                <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
                    {queryResults ? "Search Results" : "Conversations"}
                </h1>
                <div className="flex">
                    {queryResults && (
                        <button
                            onClick={clearQueryResults}
                            className="btn btn-secondary mr-2"
                        >
                            Clear Results
                        </button>
                    )}
                    <Link to="/settings" className="btn btn-primary">
                        Connect Platform
                    </Link>
                </div>
            </div>

            {/* Query/Search Input */}
            <div className="mb-6">
                <AIEnhancedQueryInput useAIQuery={!!queryResults?.isAIEnhanced} />
            </div>

            {/* Query Results Summary */}
            {queryResults && (
                <div className="mb-6 p-4 bg-indigo-50 dark:bg-indigo-900 rounded-lg">
                    <div className="flex items-center justify-between">
                        <h2 className="text-lg font-medium text-indigo-700 dark:text-indigo-200 mb-2">
                            Results for: "{location.state?.query || "Your query"}"
                        </h2>
                        {location.state?.isAIEnhanced && (
                            <div className="flex items-center text-indigo-700 dark:text-indigo-200 bg-indigo-100 dark:bg-indigo-800 px-2 py-1 rounded-md">
                                <FiZap className="mr-1" />
                                <span className="text-sm">AI-Enhanced</span>
                            </div>
                        )}
                    </div>
                    <p className="text-sm text-indigo-600 dark:text-indigo-300">
                        {queryResults.context}
                    </p>
                </div>
            )}

            {/* Filter and Sort Controls */}
            <div className="mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4">
                <div className="flex items-center space-x-2">
                    <div className="relative">
                        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <FiSearch className="text-gray-400" />
                        </div>
                        <input
                            type="text"
                            placeholder="Search conversations..."
                            className="input pl-10"
                            value={searchTerm}
                            onChange={(e) => setSearchTerm(e.target.value)}
                        />
                    </div>

                    <div className="relative">
                        <button
                            onClick={() => setFilterMenuOpen(!filterMenuOpen)}
                            className="btn btn-secondary flex items-center"
                        >
                            <FiFilter className="mr-2" />
                            <span>Filter</span>
                            {filterMenuOpen ? (
                                <FiChevronUp className="ml-2" />
                            ) : (
                                <FiChevronDown className="ml-2" />
                            )}
                        </button>

                        {filterMenuOpen && (
                            <div className="absolute right-0 mt-2 w-56 rounded-md shadow-lg bg-white dark:bg-gray-800 ring-1 ring-black ring-opacity-5 z-10">
                                <div className="py-1">
                                    <button
                                        onClick={() => {
                                            setSelectedPlatform("");
                                            setFilterMenuOpen(false);
                                        }}
                                        className={`${selectedPlatform === ""
                                                ? "bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white"
                                                : "text-gray-700 dark:text-gray-300"
                                            } w-full text-left block px-4 py-2 text-sm`}
                                    >
                                        All Platforms
                                    </button>

                                    {connectedPlatforms.map((platform) => (
                                        <button
                                            key={platform}
                                            onClick={() => {
                                                setSelectedPlatform(platform.toLowerCase());
                                                setFilterMenuOpen(false);
                                            }}
                                            className={`${selectedPlatform === platform.toLowerCase()
                                                    ? "bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white"
                                                    : "text-gray-700 dark:text-gray-300"
                                                } w-full text-left flex items-center px-4 py-2 text-sm`}
                                        >
                                            <span className="mr-2">
                                                {getPlatformIcon(platform.toLowerCase())}
                                            </span>
                                            <span>{platform}</span>
                                        </button>
                                    ))}
                                </div>
                            </div>
                        )}
                    </div>
                </div>

                <div className="flex items-center space-x-4">
                    <span className="text-sm text-gray-500 dark:text-gray-400">Sort by:</span>
                    <button
                        onClick={() => handleSortChange("lastActivity")}
                        className={`text-sm flex items-center ${sortBy === "lastActivity"
                                ? "text-indigo-600 dark:text-indigo-400 font-medium"
                                : "text-gray-500 dark:text-gray-400"
                            }`}
                    >
                        <FiCalendar className="mr-1" />
                        Last Activity
                        {sortBy === "lastActivity" && (
                            <span className="ml-1">
                                {sortDirection === "asc" ? (
                                    <FiChevronUp />
                                ) : (
                                    <FiChevronDown />
                                )}
                            </span>
                        )}
                    </button>
                    <button
                        onClick={() => handleSortChange("name")}
                        className={`text-sm flex items-center ${sortBy === "name"
                                ? "text-indigo-600 dark:text-indigo-400 font-medium"
                                : "text-gray-500 dark:text-gray-400"
                            }`}
                    >
                        <FiMessageCircle className="mr-1" />
                        Name
                        {sortBy === "name" && (
                            <span className="ml-1">
                                {sortDirection === "asc" ? (
                                    <FiChevronUp />
                                ) : (
                                    <FiChevronDown />
                                )}
                            </span>
                        )}
                    </button>
                    <button
                        onClick={() => handleSortChange("participants")}
                        className={`text-sm flex items-center ${sortBy === "participants"
                                ? "text-indigo-600 dark:text-indigo-400 font-medium"
                                : "text-gray-500 dark:text-gray-400"
                            }`}
                    >
                        <FiUsers className="mr-1" />
                        Participants
                        {sortBy === "participants" && (
                            <span className="ml-1">
                                {sortDirection === "asc" ? (
                                    <FiChevronUp />
                                ) : (
                                    <FiChevronDown />
                                )}
                            </span>
                        )}
                    </button>
                </div>
            </div>

            {/* Conversation List or Query Results */}
            <div className="space-y-4">
                {isLoading ? (
                    <div className="flex justify-center py-12">
                        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-600"></div>
                    </div>
                ) : queryResults ? (
                    // Show query results
                    queryResults.messages.length > 0 ? (
                        queryResults.messages.map((message) => (
                            <Link
                                key={message.id}
                                to={`/conversations/${message.conversation_id}`}
                                className="block"
                            >
                                <div className="card hover:bg-gray-50 dark:hover:bg-gray-750 transition">
                                    <div className="p-4">
                                        <div className="flex items-start">
                                            <div className="flex-shrink-0 pt-1">
                                                {getPlatformIcon(message.platform)}
                                            </div>
                                            <div className="ml-4 flex-1">
                                                <div className="flex items-center justify-between">
                                                    <h3 className="text-md font-medium text-gray-900 dark:text-white">
                                                        {message.sender.name}
                                                        <span className="ml-2 text-sm font-normal text-gray-500 dark:text-gray-400">
                                                            in conversation
                                                        </span>
                                                    </h3>
                                                    <span className="text-sm text-gray-500 dark:text-gray-400">
                                                        {formatDate(message.timestamp)}
                                                    </span>
                                                </div>
                                                <p className="mt-1 text-sm text-gray-600 dark:text-gray-300">
                                                    {message.content.text}
                                                </p>
                                                <div className="mt-2 flex items-center">
                                                    <span className={`platform-badge platform-${message.platform.toLowerCase()}`}>
                                                        {message.platform}
                                                    </span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </Link>
                        ))
                    ) : (
                        <div className="text-center py-12">
                            <FiSearch className="mx-auto h-12 w-12 text-gray-400" />
                            <h3 className="mt-2 text-lg font-medium text-gray-900 dark:text-white">
                                No results found
                            </h3>
                            <p className="mt-1 text-gray-500 dark:text-gray-400">
                                Try adjusting your search query or connect more platforms.
                            </p>
                        </div>
                    )
                ) : sortedConversations.length > 0 ? (
                    // Show regular conversation list
                    sortedConversations.map((conversation) => (
                        <Link
                            key={conversation.id}
                            to={`/conversations/${conversation.id}`}
                            className="block"
                        >
                            <div className="card hover:bg-gray-50 dark:hover:bg-gray-750 transition">
                                <div className="p-4">
                                    <div className="flex items-start">
                                        <div className="flex-shrink-0 pt-1">
                                            {getPlatformIcon(conversation.platform)}
                                        </div>
                                        <div className="ml-4 flex-1">
                                            <div className="flex items-center justify-between">
                                                <h3 className="text-md font-medium text-gray-900 dark:text-white">
                                                    {conversation.name}
                                                </h3>
                                                <span className="text-sm text-gray-500 dark:text-gray-400">
                                                    {formatDate(conversation.last_message_at)}
                                                </span>
                                            </div>
                                            <div className="mt-1 flex items-center text-sm text-gray-500 dark:text-gray-400">
                                                <FiUsers className="mr-1" />
                                                <span>{conversation.participants.length} participants</span>
                                            </div>
                                            <div className="mt-2 flex items-center">
                                                <span className={`platform-badge platform-${conversation.platform.toLowerCase()}`}>
                                                    {conversation.platform}
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </Link>
                    ))
                ) : selectedPlatform ? (
                    <div className="text-center py-12">
                        <FiSearch className="mx-auto h-12 w-12 text-gray-400" />
                        <h3 className="mt-2 text-lg font-medium text-gray-900 dark:text-white">
                            No conversations found for {selectedPlatform}
                        </h3>
                        <p className="mt-1 text-gray-500 dark:text-gray-400">
                            Try syncing your messages or selecting a different platform.
                        </p>
                    </div>
                ) : (
                    <div className="text-center py-12">
                        <FiMessageCircle className="mx-auto h-12 w-12 text-gray-400" />
                        <h3 className="mt-2 text-lg font-medium text-gray-900 dark:text-white">
                            No conversations yet
                        </h3>
                        <p className="mt-1 text-gray-500 dark:text-gray-400">
                            Connect a platform to start seeing your conversations.
                        </p>
                        <div className="mt-6">
                            <Link to="/settings" className="btn btn-primary">
                                Connect Platform
                            </Link>
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default Conversations;