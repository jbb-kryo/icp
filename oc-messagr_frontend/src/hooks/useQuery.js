import { useState, useCallback } from "react";
import { useApi } from "./useApi";

export const useQuery = () => {
    const { actor } = useApi();
    const [isQuerying, setIsQuerying] = useState(false);
    const [error, setError] = useState(null);

    // Execute a natural language query across platforms
    const queryMessages = useCallback(
        async (queryText, limit = 50) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setIsQuerying(true);
                setError(null);

                const result = await actor.query_conversations(queryText);

                if ("Ok" in result) {
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }

                return {
                    messages: [],
                    context: "No results found"
                };
            } catch (err) {
                console.error("Error performing query:", err);
                setError(err.message || "Failed to perform query");
                throw err;
            } finally {
                setIsQuerying(false);
            }
        },
        [actor]
    );

    // Execute advanced search with structured filters
    const advancedSearch = useCallback(
        async ({
            query,
            platform,
            startTime,
            endTime,
            conversationId,
            senderId,
            hasAttachments,
            attachmentType,
            isReply,
            inThread,
            isEdited,
            sortBy = "relevance",
            sortDirection = "desc",
            limit = 50,
            offset = 0
        }) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setIsQuerying(true);
                setError(null);

                // Convert platform string to enum if provided
                let platformEnum = null;
                if (platform) {
                    platformEnum = { [platform.charAt(0).toUpperCase() + platform.slice(1)]: null };
                }

                const result = await actor.advanced_search(
                    query,
                    platformEnum,
                    startTime ? [startTime] : [],
                    endTime ? [endTime] : [],
                    conversationId ? [conversationId] : [],
                    senderId ? [senderId] : [],
                    hasAttachments !== undefined ? [hasAttachments] : [],
                    attachmentType ? [attachmentType] : [],
                    isReply !== undefined ? [isReply] : [],
                    inThread !== undefined ? [inThread] : [],
                    isEdited !== undefined ? [isEdited] : [],
                    sortBy,
                    sortDirection,
                    [limit],
                    [offset]
                );

                if ("Ok" in result) {
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }

                return {
                    messages: [],
                    context: "No results found"
                };
            } catch (err) {
                console.error("Error performing advanced search:", err);
                setError(err.message || "Failed to perform search");
                throw err;
            } finally {
                setIsQuerying(false);
            }
        },
        [actor]
    );

    // Get information about the search indices
    const getIndexStats = useCallback(async () => {
        if (!actor) throw new Error("Actor not initialized");

        try {
            const result = await actor.get_index_stats();

            if ("Ok" in result) {
                return result.Ok;
            } else if ("Err" in result) {
                throw new Error(formatError(result.Err));
            }

            return null;
        } catch (err) {
            console.error("Error getting index stats:", err);
            setError(err.message || "Failed to get index stats");
            throw err;
        }
    }, [actor]);

    // Optimize the search indices for better performance
    const optimizeIndices = useCallback(async () => {
        if (!actor) throw new Error("Actor not initialized");

        try {
            const result = await actor.optimize_indices();

            if ("Ok" in result) {
                return result.Ok;
            } else if ("Err" in result) {
                throw new Error(formatError(result.Err));
            }

            return false;
        } catch (err) {
            console.error("Error optimizing indices:", err);
            setError(err.message || "Failed to optimize indices");
            throw err;
        }
    }, [actor]);

    // Rebuild all search indices
    const rebuildIndices = useCallback(async () => {
        if (!actor) throw new Error("Actor not initialized");

        try {
            const result = await actor.rebuild_indices();

            if ("Ok" in result) {
                return result.Ok;
            } else if ("Err" in result) {
                throw new Error(formatError(result.Err));
            }

            return false;
        } catch (err) {
            console.error("Error rebuilding indices:", err);
            setError(err.message || "Failed to rebuild indices");
            throw err;
        }
    }, [actor]);

    // Helper function to format error messages
    const formatError = (error) => {
        if (typeof error === "object") {
            if ("NotAuthenticated" in error) {
                return "Authentication required";
            } else if ("PlatformError" in error) {
                return `Platform error: ${error.PlatformError}`;
            } else if ("QueryError" in error) {
                return `Query error: ${error.QueryError}`;
            } else if ("InternalError" in error) {
                return `Internal error: ${error.InternalError}`;
            } else if ("InvalidParameters" in error) {
                return `Invalid parameters: ${error.InvalidParameters}`;
            }
        }
        return "Unknown error";
    };

    // AI-enhanced query using OpenChat SDK
    const aiEnhancedQuery = useCallback(
        async (queryText, limit = 50) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setIsQuerying(true);
                setError(null);

                const result = await actor.ai_enhanced_query(queryText);

                if ("Ok" in result) {
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }

                return {
                    messages: [],
                    context: "No results found"
                };
            } catch (err) {
                console.error("Error performing AI-enhanced query:", err);
                setError(err.message || "Failed to perform AI-enhanced query");
                throw err;
            } finally {
                setIsQuerying(false);
            }
        },
        [actor]
    );

    // Platform-specific AI query
    const aiQueryPlatform = useCallback(
        async (queryText, platform, limit = 50) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setIsQuerying(true);
                setError(null);

                // Convert platform string to enum
                const platformEnum = { [platform.charAt(0).toUpperCase() + platform.slice(1)]: null };

                const result = await actor.ai_query_platform(queryText, platformEnum);

                if ("Ok" in result) {
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }

                return {
                    messages: [],
                    context: `No results found for ${platform}`
                };
            } catch (err) {
                console.error(`Error performing AI query for ${platform}:`, err);
                setError(err.message || `Failed to perform AI query for ${platform}`);
                throw err;
            } finally {
                setIsQuerying(false);
            }
        },
        [actor]
    );

    // Analyze a specific topic across messages
    const analyzeTopic = useCallback(
        async (topic) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setIsQuerying(true);
                setError(null);

                const result = await actor.analyze_topic(topic);

                if ("Ok" in result) {
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }

                return "No insights available for this topic";
            } catch (err) {
                console.error(`Error analyzing topic ${topic}:`, err);
                setError(err.message || `Failed to analyze topic ${topic}`);
                throw err;
            } finally {
                setIsQuerying(false);
            }
        },
        [actor]
    );

    // Generate insights for a conversation
    const generateConversationInsights = useCallback(
        async (conversationId) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setIsQuerying(true);
                setError(null);

                const result = await actor.generate_conversation_insights(conversationId);

                if ("Ok" in result) {
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }

                return null;
            } catch (err) {
                console.error(`Error generating insights for conversation ${conversationId}:`, err);
                setError(err.message || `Failed to generate insights for conversation ${conversationId}`);
                throw err;
            } finally {
                setIsQuerying(false);
            }
        },
        [actor]
    );

    return {
        queryMessages,
        advancedSearch,
        getIndexStats,
        optimizeIndices,
        rebuildIndices,
        aiEnhancedQuery,
        aiQueryPlatform,
        analyzeTopic,
        generateConversationInsights,
        isQuerying,
        error,
    };
};