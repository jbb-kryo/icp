import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useQuery } from "../hooks/useQuery";
import { FiSearch, FiX, FiSliders } from "react-icons/fi";
import AdvancedSearch from "./AdvancedSearch";

const QueryInput = () => {
    const [query, setQuery] = useState("");
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [recentQueries, setRecentQueries] = useState([
        "Find messages about project deadlines",
        "Show conversations from Slack last week",
        "Messages from Alice about the budget"
    ]);
    const [showSuggestions, setShowSuggestions] = useState(false);
    const [showAdvanced, setShowAdvanced] = useState(false);
    const navigate = useNavigate();
    const { queryMessages } = useQuery();

    const handleSubmit = async (e) => {
        e.preventDefault();

        if (!query.trim()) return;

        setIsSubmitting(true);

        try {
            const results = await queryMessages(query);

            // Save the query to recent queries list
            if (!recentQueries.includes(query)) {
                setRecentQueries(prev => [query, ...prev].slice(0, 5));
            }

            // Navigate to results page
            navigate("/conversations", { state: { queryResults: results, query } });
        } catch (error) {
            console.error("Error performing query:", error);
            // Could add error handling/notification here
        } finally {
            setIsSubmitting(false);
        }
    };

    const handleQueryChange = (e) => {
        setQuery(e.target.value);
        if (e.target.value) {
            setShowSuggestions(true);
        } else {
            setShowSuggestions(false);
        }
    };

    const handleSuggestionClick = (suggestion) => {
        setQuery(suggestion);
        setShowSuggestions(false);
    };

    const handleClearQuery = () => {
        setQuery("");
        setShowSuggestions(false);
    };

    const toggleAdvancedSearch = () => {
        setShowAdvanced(!showAdvanced);
        setShowSuggestions(false);
    };

    const filteredSuggestions = query
        ? recentQueries.filter(item =>
            item.toLowerCase().includes(query.toLowerCase())
        )
        : [];

    return (
        <div className="relative">
            {!showAdvanced ? (
                <>
                    <form onSubmit={handleSubmit} className="relative">
                        <div className="relative">
                            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                <FiSearch className="text-gray-400" size={20} />
                            </div>
                            <input
                                type="text"
                                className="input pl-10 py-3 text-lg w-full rounded-lg border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
                                placeholder="Ask anything about your messages..."
                                value={query}
                                onChange={handleQueryChange}
                                onFocus={() => setShowSuggestions(true)}
                                onBlur={() => setTimeout(() => setShowSuggestions(false), 200)}
                            />
                            {query && (
                                <div className="absolute inset-y-0 right-12 flex items-center">
                                    <button
                                        type="button"
                                        onClick={handleClearQuery}
                                        className="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
                                    >
                                        <FiX size={18} />
                                    </button>
                                </div>
                            )}
                            <div className="absolute inset-y-0 right-0 flex items-center pr-3">
                                <button
                                    type="submit"
                                    disabled={isSubmitting || !query.trim()}
                                    className={`rounded-md px-3 py-1 text-sm font-medium text-white ${isSubmitting || !query.trim()
                                            ? "bg-indigo-400 cursor-not-allowed"
                                            : "bg-indigo-600 hover:bg-indigo-700"
                                        }`}
                                >
                                    {isSubmitting ? (
                                        <div className="flex items-center">
                                            <div className="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full mr-2"></div>
                                            <span>Searching</span>
                                        </div>
                                    ) : (
                                        "Search"
                                    )}
                                </button>
                            </div>
                        </div>
                    </form>

                    <div className="mt-2 flex justify-end">
                        <button
                            type="button"
                            onClick={toggleAdvancedSearch}
                            className="text-sm text-indigo-600 hover:text-indigo-700 dark:text-indigo-400 dark:hover:text-indigo-300 flex items-center"
                        >
                            <FiSliders className="mr-1" />
                            <span>Advanced Search</span>
                        </button>
                    </div>

                    {/* Example queries and suggestions */}
                    {showSuggestions && (
                        <div className="absolute z-10 mt-1 w-full bg-white dark:bg-gray-800 shadow-lg rounded-md border border-gray-200 dark:border-gray-700">
                            {filteredSuggestions.length > 0 ? (
                                <div className="py-2">
                                    <h3 className="px-4 py-1 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                                        Recent Queries
                                    </h3>
                                    <ul>
                                        {filteredSuggestions.map((suggestion, index) => (
                                            <li
                                                key={index}
                                                className="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-gray-700 dark:text-gray-200"
                                                onClick={() => handleSuggestionClick(suggestion)}
                                            >
                                                <div className="flex items-center">
                                                    <FiSearch className="text-gray-400 mr-2" size={14} />
                                                    <span>{suggestion}</span>
                                                </div>
                                            </li>
                                        ))}
                                    </ul>
                                </div>
                            ) : query ? (
                                <div className="py-3 px-4 text-sm text-gray-500 dark:text-gray-400">
                                    No recent queries match "{query}"
                                </div>
                            ) : (
                                <div className="py-2">
                                    <h3 className="px-4 py-1 text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                                        Example Queries
                                    </h3>
                                    <ul>
                                        <li className="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-gray-700 dark:text-gray-200"
                                            onClick={() => handleSuggestionClick("Find messages from John about the project deadline")}>
                                            Find messages from John about the project deadline
                                        </li>
                                        <li className="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-gray-700 dark:text-gray-200"
                                            onClick={() => handleSuggestionClick("Show all Slack conversations from last week")}>
                                            Show all Slack conversations from last week
                                        </li>
                                        <li className="px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer text-gray-700 dark:text-gray-200"
                                            onClick={() => handleSuggestionClick("Messages with attachments in WhatsApp")}>
                                            Messages with attachments in WhatsApp
                                        </li>
                                    </ul>
                                </div>
                            )}
                        </div>
                    )}
                </>
            ) : (
                <div>
                    <AdvancedSearch />
                    <div className="mt-2 flex justify-end">
                        <button
                            type="button"
                            onClick={toggleAdvancedSearch}
                            className="text-sm text-indigo-600 hover:text-indigo-700 dark:text-indigo-400 dark:hover:text-indigo-300"
                        >
                            Switch to Simple Search
                        </button>
                    </div>
                </div>
            )}
        </div>
    );
};

export default QueryInput;