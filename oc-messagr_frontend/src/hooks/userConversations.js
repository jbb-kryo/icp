import { useState, useEffect, useCallback } from "react";
import { useApi } from "./useApi";

export const useConversations = () => {
    const { actor } = useApi();
    const [conversations, setConversations] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    const [isLoadingConversation, setIsLoadingConversation] = useState(false);
    const [error, setError] = useState(null);

    // Fetch all conversations
    useEffect(() => {
        const fetchConversations = async () => {
            if (!actor) return;

            try {
                setIsLoading(true);
                setError(null);

                const allConversations = [];

                // Fetch conversations for each platform
                // This would be converted to use the Candid enum format
                const platforms = ["Telegram", "Slack", "Discord", "Twitter", "Facebook", "WhatsApp"];

                for (const platform of platforms) {
                    try {
                        const platformEnum = { [platform]: null };
                        const result = await actor.get_conversations(platformEnum);

                        if ("Ok" in result) {
                            allConversations.push(...result.Ok);
                        }
                    } catch (err) {
                        // Skip this platform if there's an error
                        console.warn(`Error fetching ${platform} conversations:`, err);
                    }
                }

                setConversations(allConversations);
            } catch (err) {
                console.error("Error fetching conversations:", err);
                setError(err.message || "Failed to fetch conversations");
            } finally {
                setIsLoading(false);
            }
        };

        fetchConversations();
    }, [actor]);

    // Get a single conversation by ID
    const getConversation = useCallback(
        async (conversationId) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setIsLoadingConversation(true);
                setError(null);

                // In the real implementation, we'd have a specific API for this
                // For now, we're searching through our cached conversations
                const conversation = conversations.find(
                    (conv) => conv.id === conversationId
                );

                if (conversation) {
                    return conversation;
                }

                throw new Error("Conversation not found");
            } catch (err) {
                console.error("Error fetching conversation:", err);
                setError(err.message || "Failed to fetch conversation");
                throw err;
            } finally {
                setIsLoadingConversation(false);
            }
        },
        [actor, conversations]
    );

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

    return {
        conversations,
        getConversation,
        isLoading,
        isLoadingConversation,
        error,
    };
};