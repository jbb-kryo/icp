import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { usePlatforms } from "../hooks/usePlatforms";
import { useQuery } from "../hooks/useQuery";
import { FiSearch, FiFilter, FiX, FiChevronDown, FiChevronUp } from "react-icons/fi";
import {
    FaTelegram,
    FaSlack,
    FaDiscord,
    FaTwitter,
    FaFacebook,
    FaWhatsapp,
} from "react-icons/fa";

const AdvancedSearch = () => {
    const navigate = useNavigate();
    const { connectedPlatforms } = usePlatforms();
    const { advancedSearch, isQuerying } = useQuery();

    const [searchTerm, setSearchTerm] = useState("");
    const [showFilters, setShowFilters] = useState(false);

    // Filter states
    const [platform, setPlatform] = useState("");
    const [timeRange, setTimeRange] = useState("any");
    const [hasAttachments, setHasAttachments] = useState(false);
    const [attachmentType, setAttachmentType] = useState("");
    const [isReply, setIsReply] = useState(false);
    const [isEdited, setIsEdited] = useState(false);
    const [sortBy, setSortBy] = useState("relevance");
    const [sortDirection, setSortDirection] = useState("desc");

    // Custom time range
    const [startDate, setStartDate] = useState("");
    const [endDate, setEndDate] = useState("");

    const getPlatformIcon = (platformId) => {
        switch (platformId.toLowerCase()) {
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
                return null;
        }
    };

    // Calculate time range in milliseconds
    const getTimeRangeValues = () => {
        const now = Date.now();
        const HOUR = 60 * 60 * 1000;
        const DAY = 24 * HOUR;
        const WEEK = 7 * DAY;
        const MONTH = 30 * DAY;

        switch (timeRange) {
            case "hour":
                return { start: now - HOUR, end: now };
            case "day":
                return { start: now - DAY, end: now };
            case "week":
                return { start: now - WEEK, end: now };
            case "month":
                return { start: now - MONTH, end: now };
            case "custom":
                return {
                    start: startDate ? new Date(startDate).getTime() : undefined,
                    end: endDate ? new Date(endDate).getTime() : undefined,
                };
            default:
                return { start: undefined, end: undefined };
        }
    };

    const handleSearch = async (e) => {
        e.preventDefault();

        if (!searchTerm.trim()) return;

        const timeRangeValues = getTimeRangeValues();

        try {
            const results = await advancedSearch({
                query: searchTerm,
                platform: platform || undefined,
                startTime: timeRangeValues.start,
                endTime: timeRangeValues.end,
                hasAttachments,
                attachmentType: attachmentType || undefined,
                isReply,
                isEdited,
                sortBy,
                sortDirection,
            });

            navigate("/conversations", {
                state: { queryResults: results, query: searchTerm, isAdvancedSearch: true },
            });
        } catch (error) {
            console.error("Error performing advanced search:", error);
        }
    };

    const resetFilters = () => {
        setPlatform("");
        setTimeRange("any");
        setHasAttachments(false);
        setAttachmentType("");
        setIsReply(false);
        setIsEdited(false);
        setSortBy("relevance");
        setSortDirection("desc");
        setStartDate("");
        setEndDate("");
    };

    return (
        <div className="w-full">
            <form onSubmit={handleSearch}>
                <div className="relative">
                    <div className="flex">
                        <div className="relative flex-grow">
                            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                <FiSearch className="text-gray-400" size={20} />
                            </div>
                            <input
                                type="text"
                                className="input pl-10 py-3 text-lg w-full rounded-lg border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
                                placeholder="Search across all platforms..."
                                value={searchTerm}
                                onChange={(e) => setSearchTerm(e.target.value)}
                            />
                            {searchTerm && (
                                <button
                                    type="button"
                                    onClick={() => setSearchTerm("")}
                                    className="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
                                >
                                    <FiX size={18} />
                                </button>
                            )}
                        </div>
                        <button
                            type="button"
                            onClick={() => setShowFilters(!showFilters)}
                            className="ml-2 btn btn-secondary flex items-center"
                        >
                            <FiFilter className="mr-2" />
                            <span>Filters</span>
                            {showFilters ? (
                                <FiChevronUp className="ml-2" />
                            ) : (
                                <FiChevronDown className="ml-2" />
                            )}
                        </button>
                        <button
                            type="submit"
                            disabled={isQuerying || !searchTerm.trim()}
                            className={`ml-2 btn btn-primary ${isQuerying || !searchTerm.trim()
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
                </div>

                {/* Advanced filters */}
                {showFilters && (
                    <div className="mt-4 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
                        <div className="flex flex-wrap -mx-2">
                            {/* Platform filter */}
                            <div className="px-2 w-full md:w-1/3 lg:w-1/4 mb-4">
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Platform
                                </label>
                                <select
                                    className="input"
                                    value={platform}
                                    onChange={(e) => setPlatform(e.target.value)}
                                >
                                    <option value="">All Platforms</option>
                                    {connectedPlatforms.map((p) => (
                                        <option key={p.toLowerCase()} value={p.toLowerCase()}>
                                            {p}
                                        </option>
                                    ))}
                                </select>
                            </div>

                            {/* Time range filter */}
                            <div className="px-2 w-full md:w-1/3 lg:w-1/4 mb-4">
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Time Range
                                </label>
                                <select
                                    className="input"
                                    value={timeRange}
                                    onChange={(e) => setTimeRange(e.target.value)}
                                >
                                    <option value="any">Any Time</option>
                                    <option value="hour">Past Hour</option>
                                    <option value="day">Past 24 Hours</option>
                                    <option value="week">Past Week</option>
                                    <option value="month">Past Month</option>
                                    <option value="custom">Custom Range</option>
                                </select>
                            </div>

                            {/* Custom time range */}
                            {timeRange === "custom" && (
                                <div className="px-2 w-full md:w-2/3 lg:w-1/2 mb-4">
                                    <div className="flex space-x-2">
                                        <div className="flex-1">
                                            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                                Start Date
                                            </label>
                                            <input
                                                type="date"
                                                className="input"
                                                value={startDate}
                                                onChange={(e) => setStartDate(e.target.value)}
                                            />
                                        </div>
                                        <div className="flex-1">
                                            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                                End Date
                                            </label>
                                            <input
                                                type="date"
                                                className="input"
                                                value={endDate}
                                                onChange={(e) => setEndDate(e.target.value)}
                                            />
                                        </div>
                                    </div>
                                </div>
                            )}

                            {/* Content filters */}
                            <div className="px-2 w-full md:w-1/3 lg:w-1/4 mb-4">
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Content Type
                                </label>
                                <div className="space-y-2">
                                    <div className="flex items-center">
                                        <input
                                            id="has-attachments"
                                            type="checkbox"
                                            className="h-4 w-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-500"
                                            checked={hasAttachments}
                                            onChange={(e) => setHasAttachments(e.target.checked)}
                                        />
                                        <label
                                            htmlFor="has-attachments"
                                            className="ml-2 text-sm text-gray-700 dark:text-gray-300"
                                        >
                                            Has Attachments
                                        </label>
                                    </div>

                                    {hasAttachments && (
                                        <div className="ml-6">
                                            <select
                                                className="input"
                                                value={attachmentType}
                                                onChange={(e) => setAttachmentType(e.target.value)}
                                            >
                                                <option value="">Any Type</option>
                                                <option value="image">Images</option>
                                                <option value="file">Files/Documents</option>
                                                <option value="audio">Audio</option>
                                                <option value="video">Video</option>
                                                <option value="link">Links</option>
                                            </select>
                                        </div>
                                    )}
                                </div>
                            </div>

                            {/* Message type filters */}
                            <div className="px-2 w-full md:w-1/3 lg:w-1/4 mb-4">
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Message Type
                                </label>
                                <div className="space-y-2">
                                    <div className="flex items-center">
                                        <input
                                            id="is-reply"
                                            type="checkbox"
                                            className="h-4 w-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-500"
                                            checked={isReply}
                                            onChange={(e) => setIsReply(e.target.checked)}
                                        />
                                        <label
                                            htmlFor="is-reply"
                                            className="ml-2 text-sm text-gray-700 dark:text-gray-300"
                                        >
                                            Only Replies
                                        </label>
                                    </div>
                                    <div className="flex items-center">
                                        <input
                                            id="is-edited"
                                            type="checkbox"
                                            className="h-4 w-4 text-indigo-600 rounded border-gray-300 focus:ring-indigo-500"
                                            checked={isEdited}
                                            onChange={(e) => setIsEdited(e.target.checked)}
                                        />
                                        <label
                                            htmlFor="is-edited"
                                            className="ml-2 text-sm text-gray-700 dark:text-gray-300"
                                        >
                                            Edited Messages
                                        </label>
                                    </div>
                                </div>
                            </div>

                            {/* Sort options */}
                            <div className="px-2 w-full md:w-1/3 lg:w-1/4 mb-4">
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Sort By
                                </label>
                                <div className="flex space-x-2">
                                    <select
                                        className="input"
                                        value={sortBy}
                                        onChange={(e) => setSortBy(e.target.value)}
                                    >
                                        <option value="relevance">Relevance</option>
                                        <option value="time">Time</option>
                                        <option value="platform">Platform</option>
                                    </select>
                                    <select
                                        className="input"
                                        value={sortDirection}
                                        onChange={(e) => setSortDirection(e.target.value)}
                                    >
                                        <option value="desc">Descending</option>
                                        <option value="asc">Ascending</option>
                                    </select>
                                </div>
                            </div>
                        </div>

                        {/* Reset filters button */}
                        <div className="mt-2 flex justify-end">
                            <button
                                type="button"
                                onClick={resetFilters}
                                className="btn btn-secondary text-sm"
                            >
                                Reset Filters
                            </button>
                        </div>
                    </div>
                )}
            </form>

            {/* Search hints */}
            {!showFilters && (
                <div className="mt-2 text-xs text-gray-500 dark:text-gray-400">
                    <p>
                        Pro tip: Try natural language queries like "messages from John about project deadline in Slack last week"
                    </p>
                </div>
            )}
        </div>
    );
};

export default AdvancedSearch;