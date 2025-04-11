import React, { useState, useEffect } from "react";
import { useQuery } from "../hooks/useQuery";
import {
    FiZap,
    FiUsers,
    FiCalendar,
    FiMessageSquare,
    FiTrendingUp,
    FiCheck,
    FiList,
    FiAlertTriangle,
    FiLoader,
    FiChevronDown,
    FiChevronUp
} from "react-icons/fi";

const ConversationInsights = ({ conversationId }) => {
    const { generateConversationInsights } = useQuery();
    const [insights, setInsights] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [expandedSections, setExpandedSections] = useState({
        entities: true,
        topics: true,
        timeline: false,
        sentiment: false,
        flow: false
    });

    useEffect(() => {
        if (!conversationId) return;

        const fetchInsights = async () => {
            try {
                setLoading(true);
                const result = await generateConversationInsights(conversationId);
                setInsights(result);
            } catch (err) {
                console.error("Error fetching insights:", err);
                setError(err.message || "Failed to generate insights");
            } finally {
                setLoading(false);
            }
        };

        fetchInsights();
    }, [conversationId, generateConversationInsights]);

    const toggleSection = (section) => {
        setExpandedSections(prev => ({
            ...prev,
            [section]: !prev[section]
        }));
    };

    if (loading) {
        return (
            <div className="card p-6 flex flex-col items-center justify-center min-h-40">
                <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-indigo-600 mb-4"></div>
                <p className="text-gray-500 dark:text-gray-400">
                    Analyzing conversation with AI...
                </p>
            </div>
        );
    }

    if (error) {
        return (
            <div className="card p-6">
                <div className="flex items-center text-red-600 dark:text-red-400 mb-4">
                    <FiAlertTriangle className="mr-2" size={20} />
                    <h3 className="text-lg font-medium">Error Generating Insights</h3>
                </div>
                <p className="text-gray-600 dark:text-gray-300">{error}</p>
            </div>
        );
    }

    if (!insights) {
        return (
            <div className="card p-6">
                <p className="text-gray-500 dark:text-gray-400">
                    No insights available for this conversation.
                </p>
            </div>
        );
    }

    return (
        <div className="card">
            <div className="card-header flex items-center">
                <FiZap className="mr-2 text-indigo-500" />
                <h2 className="text-lg font-medium">AI-Generated Conversation Insights</h2>
            </div>

            <div className="card-body">
                {/* Entities Section */}
                <div className="mb-6">
                    <div
                        className="flex items-center justify-between cursor-pointer"
                        onClick={() => toggleSection("entities")}
                    >
                        <div className="flex items-center">
                            <FiUsers className="mr-2 text-indigo-500" />
                            <h3 className="text-md font-medium">Key Entities</h3>
                        </div>
                        {expandedSections.entities ? <FiChevronUp /> : <FiChevronDown />}
                    </div>

                    {expandedSections.entities && (
                        <div className="mt-3 pl-6">
                            {insights.entities.length > 0 ? (
                                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    {insights.entities.map((entity, index) => (
                                        <div
                                            key={index}
                                            className="bg-gray-50 dark:bg-gray-800 p-3 rounded-md border border-gray-200 dark:border-gray-700"
                                        >
                                            <div className="font-medium">{entity.name}</div>
                                            <div className="text-sm text-gray-500 dark:text-gray-400">
                                                <span className="mr-3">Type: {formatEntityType(entity.entity_type)}</span>
                                                <span>Mentions: {entity.mentions}</span>
                                            </div>
                                            {entity.sentiment_score !== null && (
                                                <div className="text-sm mt-1">
                                                    Sentiment: {formatSentiment(entity.sentiment_score)}
                                                </div>
                                            )}
                                        </div>
                                    ))}
                                </div>
                            ) : (
                                <p className="text-gray-500 dark:text-gray-400">No key entities identified.</p>
                            )}
                        </div>
                    )}
                </div>

                {/* Topics Section */}
                <div className="mb-6">
                    <div
                        className="flex items-center justify-between cursor-pointer"
                        onClick={() => toggleSection("topics")}
                    >
                        <div className="flex items-center">
                            <FiMessageSquare className="mr-2 text-indigo-500" />
                            <h3 className="text-md font-medium">Main Topics</h3>
                        </div>
                        {expandedSections.topics ? <FiChevronUp /> : <FiChevronDown />}
                    </div>

                    {expandedSections.topics && (
                        <div className="mt-3 pl-6">
                            {insights.topics.length > 0 ? (
                                <div className="space-y-4">
                                    {insights.topics.map((topic, index) => (
                                        <div
                                            key={index}
                                            className="bg-gray-50 dark:bg-gray-800 p-3 rounded-md border border-gray-200 dark:border-gray-700"
                                        >
                                            <div className="font-medium">{topic.name}</div>
                                            <div className="text-sm text-gray-500 dark:text-gray-400 flex items-center">
                                                <span className="mr-2">Relevance:</span>
                                                <div className="bg-gray-200 dark:bg-gray-700 h-2 w-24 rounded-full overflow-hidden">
                                                    <div
                                                        className="bg-indigo-500 h-full"
                                                        style={{ width: `${topic.relevance_score * 100}%` }}
                                                    ></div>
                                                </div>
                                                <span className="ml-2">{Math.round(topic.relevance_score * 100)}%</span>
                                            </div>
                                            <div className="text-sm mt-2">{topic.summary}</div>
                                        </div>
                                    ))}
                                </div>
                            ) : (
                                <p className="text-gray-500 dark:text-gray-400">No main topics identified.</p>
                            )}
                        </div>
                    )}
                </div>

                {/* Timeline Section */}
                {insights.timeline && (
                    <div className="mb-6">
                        <div
                            className="flex items-center justify-between cursor-pointer"
                            onClick={() => toggleSection("timeline")}
                        >
                            <div className="flex items-center">
                                <FiCalendar className="mr-2 text-indigo-500" />
                                <h3 className="text-md font-medium">Conversation Timeline</h3>
                            </div>
                            {expandedSections.timeline ? <FiChevronUp /> : <FiChevronDown />}
                        </div>

                        {expandedSections.timeline && (
                            <div className="mt-3 pl-6">
                                {insights.timeline.length > 0 ? (
                                    <div className="relative">
                                        {/* Timeline line */}
                                        <div className="absolute left-0 top-0 bottom-0 w-0.5 bg-gray-200 dark:bg-gray-700"></div>

                                        {/* Timeline events */}
                                        <div className="space-y-6 pl-6">
                                            {insights.timeline.map((event, index) => (
                                                <div key={index} className="relative">
                                                    {/* Timeline dot */}
                                                    <div className="absolute -left-6 top-0 w-3 h-3 bg-indigo-500 rounded-full"></div>

                                                    <div className="text-sm text-gray-500 dark:text-gray-400">
                                                        {formatEventTimestamp(event.timestamp)}
                                                    </div>
                                                    <div className="mt-1">{event.description}</div>
                                                </div>
                                            ))}
                                        </div>
                                    </div>
                                ) : (
                                    <p className="text-gray-500 dark:text-gray-400">No timeline available.</p>
                                )}
                            </div>
                        )}
                    </div>
                )}

                {/* Sentiment Analysis */}
                {insights.sentiment && (
                    <div className="mb-6">
                        <div
                            className="flex items-center justify-between cursor-pointer"
                            onClick={() => toggleSection("sentiment")}
                        >
                            <div className="flex items-center">
                                <FiTrendingUp className="mr-2 text-indigo-500" />
                                <h3 className="text-md font-medium">Sentiment Analysis</h3>
                            </div>
                            {expandedSections.sentiment ? <FiChevronUp /> : <FiChevronDown />}
                        </div>

                        {expandedSections.sentiment && (
                            <div className="mt-3 pl-6">
                                <div className="bg-gray-50 dark:bg-gray-800 p-4 rounded-md border border-gray-200 dark:border-gray-700">
                                    <div className="mb-3">
                                        <div className="text-sm text-gray-500 dark:text-gray-400 mb-1">Overall Sentiment</div>
                                        <div className="relative pt-1">
                                            <div className="flex mb-2 items-center justify-between">
                                                <div className="text-xs text-red-600 dark:text-red-400">Negative</div>
                                                <div className="text-xs text-green-600 dark:text-green-400">Positive</div>
                                            </div>
                                            <div className="overflow-hidden h-2 text-xs flex rounded bg-gray-200 dark:bg-gray-700">
                                                <div
                                                    className={`shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center ${insights.sentiment.overall_sentiment > 0
                                                            ? 'bg-green-500'
                                                            : 'bg-red-500'
                                                        }`}
                                                    style={{
                                                        width: `${Math.abs(insights.sentiment.overall_sentiment) * 100}%`,
                                                        marginLeft: insights.sentiment.overall_sentiment > 0 ? '50%' : undefined,
                                                        marginRight: insights.sentiment.overall_sentiment < 0 ? '50%' : undefined,
                                                    }}
                                                ></div>
                                            </div>
                                        </div>
                                    </div>

                                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
                                        <div>
                                            <h4 className="text-sm font-medium mb-2">Key Positive Points</h4>
                                            <ul className="list-disc pl-5 text-sm text-gray-600 dark:text-gray-300">
                                                {insights.sentiment.key_positive_points.map((point, i) => (
                                                    <li key={i}>{point}</li>
                                                ))}
                                            </ul>
                                        </div>
                                        <div>
                                            <h4 className="text-sm font-medium mb-2">Key Negative Points</h4>
                                            <ul className="list-disc pl-5 text-sm text-gray-600 dark:text-gray-300">
                                                {insights.sentiment.key_negative_points.map((point, i) => (
                                                    <li key={i}>{point}</li>
                                                ))}
                                            </ul>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        )}
                    </div>
                )}

                {/* Conversation Flow */}
                {insights.conversation_flow && (
                    <div className="mb-6">
                        <div
                            className="flex items-center justify-between cursor-pointer"
                            onClick={() => toggleSection("flow")}
                        >
                            <div className="flex items-center">
                                <FiList className="mr-2 text-indigo-500" />
                                <h3 className="text-md font-medium">Conversation Flow</h3>
                            </div>
                            {expandedSections.flow ? <FiChevronUp /> : <FiChevronDown />}
                        </div>

                        {expandedSections.flow && (
                            <div className="mt-3 pl-6">
                                {insights.conversation_flow.main_threads.length > 0 && (
                                    <div>
                                        <h4 className="text-sm font-medium mb-2">Discussion Threads</h4>
                                        <div className="space-y-3">
                                            {insights.conversation_flow.main_threads.map((thread, i) => (
                                                <div
                                                    key={i}
                                                    className="bg-gray-50 dark:bg-gray-800 p-3 rounded-md border border-gray-200 dark:border-gray-700"
                                                >
                                                    <div className="flex items-center justify-between">
                                                        <span className="font-medium">{thread.topic}</span>
                                                        {thread.resolved && (
                                                            <span className="text-green-500 dark:text-green-400 flex items-center text-sm">
                                                                <FiCheck className="mr-1" />
                                                                Resolved
                                                            </span>
                                                        )}
                                                    </div>
                                                    <div className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                                        Participants: {thread.participants.join(", ")}
                                                    </div>
                                                </div>
                                            ))}
                                        </div>
                                    </div>
                                )}

                                {insights.conversation_flow.key_decisions.length > 0 && (
                                    <div className="mt-4">
                                        <h4 className="text-sm font-medium mb-2">Key Decisions</h4>
                                        <ul className="list-disc pl-5 text-sm text-gray-600 dark:text-gray-300">
                                            {insights.conversation_flow.key_decisions.map((decision, i) => (
                                                <li key={i}>{decision}</li>
                                            ))}
                                        </ul>
                                    </div>
                                )}

                                {insights.conversation_flow.action_items.length > 0 && (
                                    <div className="mt-4">
                                        <h4 className="text-sm font-medium mb-2">Action Items</h4>
                                        <ul className="list-disc pl-5 text-sm text-gray-600 dark:text-gray-300">
                                            {insights.conversation_flow.action_items.map((item, i) => (
                                                <li key={i}>{item}</li>
                                            ))}
                                        </ul>
                                    </div>
                                )}
                            </div>
                        )}
                    </div>
                )}
            </div>

            <div className="card-footer text-xs text-gray-500 dark:text-gray-400 italic">
                Insights generated by AI based on conversation content. These insights are for reference only and may not be 100% accurate.
            </div>
        </div>
    );
};

// Helper functions for formatting
const formatEntityType = (entityType) => {
    if (typeof entityType === 'string') {
        return entityType;
    }

    if (entityType.Other) {
        return entityType.Other;
    }

    return Object.keys(entityType)[0];
};

const formatSentiment = (score) => {
    if (score > 0.3) return "Positive";
    if (score < -0.3) return "Negative";
    return "Neutral";
};

const formatEventTimestamp = (timestamp) => {
    const date = new Date(timestamp);
    return date.toLocaleString();
};

export default ConversationInsights;