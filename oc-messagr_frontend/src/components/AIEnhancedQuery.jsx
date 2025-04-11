import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { FiSearch, FiLoader, FiZap, FiX, FiCommand } from "react-icons/fi";
import { usePlatforms } from "../hooks/usePlatforms";

const AIEnhancedQueryInput = ({ useAIQuery }) => {
    const [query, setQuery] = useState("");
    const [isQuerying, setIsQuerying] = useState(false);
    const [aiEnabled, setAiEnabled] = useState(useAIQuery);
    const [selectedPlatform, setSelectedPlatform] = useState("");
    const navigate = useNavigate();
    const { connectedPlatforms } = usePlatforms();

    const handleSubmit = async (e) => {
        e.preventDefault();

        if (!query.trim()) return;

        setIsQuerying(true);

        try {
            let results;

            if (aiEnabled) {
                // Use AI-enhanced query
                if (selectedPlatform) {
                    // Platform-specific AI query
                    results = await window.aiQueryPlatform(query, selectedPlatform);
                } else {
                    // General AI query
                    results = await window.aiEnhancedQuery(query);
                }
            } else {
                // Standard query
                results = await window.queryMessages(query);
            }

            // Navigate to results page
            navigate("/conversations", {
                state: {
                    queryResults: results,
                    query,
                    isAIEnhanced: aiEnabled,
                    platform: selectedPlatform || null
                }
            });
        } catch (error) {
            console.error("Error performing query:", error);
        } finally {
            setIsQuerying(false);
        }
    };

    const toggleAI = () => {
        setAiEnabled(!aiEnabled);
    };

    const handleClearQuery = () => {
        setQuery("");
    };

    return (
        <div className="w-full">
            <form onSubmit={handleSubmit} className="relative">
                <div className="flex">
                    <div className="relative flex-grow">
                        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            {aiEnabled ? (
                                <FiZap className="text-indigo-500" size={20} />
                            ) : (
                                <FiSearch className="text-gray-400" size={20} />
                            )}
                        </div>
                        <input
                            type="text"
                            className={`input pl-10 py-3 text-lg w-full rounded-lg ${aiEnabled
                                    ? "border-indigo-300 focus:border-indigo-500 dark:border-indigo-700 dark:focus:border-indigo-500"
                                    : "border-gray-300 dark:border-gray-600"
                                } dark:bg-gray-700 dark:text-white`}
                            placeholder={aiEnabled
                                ? "Ask anything using AI-powered search..."
                                : "Search across your messages..."}
                            value={query}
                            onChange={(e) => setQuery(e.target.value)}
                        />
                        {query && (
                            <button
                                type="button"
                                onClick={handleClearQuery}
                                className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
                            >
                                <FiX size={18} />
                            </button>
                        )}
                    </div>

                    {/* AI Toggle Button */}
                    <button
                        type="button"
                        onClick={toggleAI}
                        className={`ml-2 px-3 rounded-md flex items-center justify-center ${aiEnabled
                                ? "bg-indigo-100 text-indigo-700 dark:bg-indigo-900 dark:text-indigo-300"
                                : "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300"
                            }`}
                        title={aiEnabled ? "Disable AI-powered search" : "Enable AI-powered search"}
                    >
                        <FiZap size={20} />
                    </button>

                    {/* Platform selector (only for AI queries) */}
                    {aiEnabled && (
                        <select
                            className="ml-2 input"
                            value={selectedPlatform}
                            onChange={(e) => setSelectedPlatform(e.target.value)}
                        >
                            <option value="">All Platforms</option>
                            {connectedPlatforms.map((platform) => (
                                <option key={platform.toLowerCase()} value={platform.toLowerCase()}>
                                    {platform}
                                </option>
                            ))}
                        </select>
                    )}

                    <button
                        type="submit"
                        disabled={isQuerying || !query.trim()}
                        className={`ml-2 btn ${aiEnabled ? "btn-primary" : "btn-secondary"
                            } ${isQuerying || !query.trim()
                                ? "opacity-70 cursor-not-allowed"
                                : ""
                            }`}
                    >
                        {isQuerying ? (
                            <div className="flex items-center">
                                <div className="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full mr-2"></div>
                                <span>Searching</span>
                            </div>
                        ) : (
                            <span>Search</span>
                        )}
                    </button>
                </div>
            </form>

            {/* AI search hint */}
            {aiEnabled && (
                <div className="mt-2">
                    <div className="bg-indigo-50 dark:bg-indigo-900/30 p-2 rounded-md">
                        <div className="flex items-center text-xs text-indigo-700 dark:text-indigo-300">
                            <FiCommand className="mr-1" />
                            <span>
                                <span className="font-semibold">AI-powered search</span> {" "}
                                can answer complex questions like "What was that project deadline Alice mentioned last week?"
                            </span>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default AIEnhancedQueryInput;