import React, { createContext, useContext, useState, useEffect, useCallback } from "react";
import { useAuth } from "../hooks/useAuth";
import { useApi } from "../hooks/useApi";

const PlatformContext = createContext(null);

export const usePlatforms = () => useContext(PlatformContext);

export const PlatformProvider = ({ children }) => {
    const { isAuthenticated } = useAuth();
    const { actor } = useApi();
    const [connectedPlatforms, setConnectedPlatforms] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    const [isConnectingPlatform, setIsConnectingPlatform] = useState(false);
    const [error, setError] = useState(null);

    // Fetch connected platforms when authenticated
    useEffect(() => {
        const fetchPlatforms = async () => {
            if (!isAuthenticated || !actor) return;

            try {
                setIsLoading(true);
                const platforms = await actor.get_connected_platforms();
                setConnectedPlatforms(platforms.map(formatPlatformName));
            } catch (error) {
                console.error("Error fetching platforms:", error);
                setError("Failed to fetch connected platforms");
            } finally {
                setIsLoading(false);
            }
        };

        fetchPlatforms();
    }, [isAuthenticated, actor]);

    // Connect a new platform
    const connectPlatform = useCallback(
        async (authConfig) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setIsConnectingPlatform(true);
                setError(null);

                const result = await actor.connect_platform(authConfig);

                if ("Ok" in result) {
                    // Refresh the list of connected platforms
                    const platforms = await actor.get_connected_platforms();
                    setConnectedPlatforms(platforms.map(formatPlatformName));
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }
            } catch (error) {
                console.error("Error connecting platform:", error);
                setError(error.message || "Failed to connect platform");
                throw error;
            } finally {
                setIsConnectingPlatform(false);
            }
        },
        [actor]
    );

    // Disconnect a platform
    const disconnectPlatform = useCallback(
        async (platform) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setError(null);
                const platformEnum = { [platform]: null };
                const result = await actor.disconnect_platform(platformEnum);

                if ("Ok" in result) {
                    // Refresh the list of connected platforms
                    const platforms = await actor.get_connected_platforms();
                    setConnectedPlatforms(platforms.map(formatPlatformName));
                    return true;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }
            } catch (error) {
                console.error(`Error disconnecting ${platform}:`, error);
                setError(error.message || `Failed to disconnect ${platform}`);
                throw error;
            }
        },
        [actor]
    );

    // Sync messages for a platform
    const syncMessages = useCallback(
        async (platform) => {
            if (!actor) throw new Error("Actor not initialized");

            try {
                setError(null);
                const platformEnum = { [platform]: null };
                const result = await actor.sync_messages(platformEnum);

                if ("Ok" in result) {
                    return result.Ok;
                } else if ("Err" in result) {
                    throw new Error(formatError(result.Err));
                }
            } catch (error) {
                console.error(`Error syncing ${platform} messages:`, error);
                setError(error.message || `Failed to sync ${platform} messages`);
                throw error;
            }
        },
        [actor]
    );

    // Helper functions
    const formatPlatformName = (platform) => {
        // Convert platform enum to string
        if (typeof platform === "object") {
            const key = Object.keys(platform)[0];
            return key.charAt(0).toUpperCase() + key.slice(1);
        }
        return platform;
    };

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

    const value = {
        connectedPlatforms,
        isLoading,
        isConnectingPlatform,
        error,
        connectPlatform,
        disconnectPlatform,
        syncMessages,
    };

    return (
        <PlatformContext.Provider value={value}>
            {children}
        </PlatformContext.Provider>
    );
};