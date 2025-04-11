import React, { createContext, useContext, useState, useCallback, useEffect } from "react";
import { AuthClient } from "@dfinity/auth-client";
import { Actor, Identity } from "@dfinity/agent";

const AuthContext = createContext(null);

export const useAuth = () => useContext(AuthContext);

export const AuthProvider = ({ children }) => {
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [isLoading, setIsLoading] = useState(true);
    const [authClient, setAuthClient] = useState(null);
    const [identity, setIdentity] = useState(null);
    const [principal, setPrincipal] = useState(null);

    // Initialize auth client when component mounts
    useEffect(() => {
        const initAuth = async () => {
            try {
                const client = await AuthClient.create();
                setAuthClient(client);

                // Check if user is already authenticated
                const isLoggedIn = await client.isAuthenticated();
                if (isLoggedIn) {
                    const identity = client.getIdentity();
                    const principal = identity.getPrincipal();

                    setIdentity(identity);
                    setPrincipal(principal);
                    setIsAuthenticated(true);
                }
            } catch (error) {
                console.error("Error initializing auth client:", error);
            } finally {
                setIsLoading(false);
            }
        };

        initAuth();
    }, []);

    // Login method
    const login = useCallback(async () => {
        if (!authClient) return;

        try {
            setIsLoading(true);
            const daysToLivePerLogin = 30; // 30 days
            const millisToLivePerLogin = daysToLivePerLogin * 24 * 60 * 60 * 1000;

            await new Promise((resolve, reject) => {
                authClient.login({
                    identityProvider: process.env.DFX_NETWORK === "ic"
                        ? "https://identity.ic0.app"
                        : `http://localhost:8000/?canisterId=${process.env.INTERNET_IDENTITY_CANISTER_ID}`,
                    maxTimeToLive: BigInt(millisToLivePerLogin),
                    onSuccess: () => {
                        resolve();
                    },
                    onError: (error) => {
                        reject(new Error(`Login failed: ${error}`));
                    },
                });
            });

            const identity = authClient.getIdentity();
            const principal = identity.getPrincipal();

            setIdentity(identity);
            setPrincipal(principal);
            setIsAuthenticated(true);
            return true;
        } catch (error) {
            console.error("Login error:", error);
            return false;
        } finally {
            setIsLoading(false);
        }
    }, [authClient]);

    // Logout method
    const logout = useCallback(async () => {
        if (!authClient) return;

        try {
            setIsLoading(true);
            await authClient.logout();
            setIsAuthenticated(false);
            setIdentity(null);
            setPrincipal(null);
        } catch (error) {
            console.error("Logout error:", error);
        } finally {
            setIsLoading(false);
        }
    }, [authClient]);

    // Create an authenticated actor with the user's identity
    const createActor = useCallback(
        (canisterId, idlFactory) => {
            if (!identity) return null;

            // Create an actor with the user's identity
            return Actor.createActor(idlFactory, {
                agent: {
                    identity,
                },
                canisterId,
            });
        },
        [identity]
    );

    const value = {
        isAuthenticated,
        isLoading,
        identity,
        principal,
        login,
        logout,
        createActor,
    };

    return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};