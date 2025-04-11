import { useState, useCallback } from "react";
import { useApi } from "./useApi";

export const useMessages = () => {
    const { actor } = useApi();
    const [isLoadingMessages, setIsLoadingMessages] = useState(false);
    const [error, setError] = useState(null);

    // Fetch messages for a conversation
    const getMessages = useCallback(
        async (conversationId, limit = 100, offset = 0) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setIsLoadingMessages(true);
                setError(null);

                const result = await actor.get_messages(conversationId, [limit], [offset]);

                if ("Ok" in result) {
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }

                return [];
            } catch (err) {
                console.error("Error fetching messages:", err);
                setError(err.message || "Failed to fetch messages");
                throw err;
            } finally {
                setIsLoadingMessages(false);
            }
        },
        [actor]
    );

    // Sync messages for a specific conversation
    const syncConversationMessages = useCallback(
        async (platform, conversationId) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setError(null);
                const platformEnum = { [platform]: null };

                // In a real implementation, this would be a specific API call
                // Here we're using the general sync method
                const result = await actor.sync_messages(platformEnum);

                if ("Ok" in result) {
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }
            } catch (err) {
                console.error("Error syncing messages:", err);
                setError(err.message || "Failed to sync messages");
                throw err;
            }
        },
        [actor]
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
        getMessages,
        syncMessages: syncConversationMessages,
        isLoadingMessages,
        error,
    };
};