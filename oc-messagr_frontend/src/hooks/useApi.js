import { useState, useEffect } from "react";
import { useAuth } from "./useAuth";
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../declarations/messagr_app";

// Local canister ID - would be set by environment/webpack
const canisterId = process.env.MESSAGR_APP_CANISTER_ID || "rrkah-fqaaa-aaaaa-aaaaq-cai";

export const useApi = () => {
    const { isAuthenticated, identity } = useAuth();
    const [actor, setActor] = useState(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        const initActor = async () => {
            try {
                setIsLoading(true);
                setError(null);

                if (!isAuthenticated || !identity) {
                    setActor(null);
                    return;
                }

                // Create an agent with the user's identity
                const host = process.env.DFX_NETWORK === "ic"
                    ? "https://ic0.app"
                    : "http://localhost:8000";

                const agent = new HttpAgent({ identity, host });

                // When in development, use dev server's local replica
                if (process.env.NODE_ENV !== "production") {
                    await agent.fetchRootKey();
                }

                // Create actor with the agent
                const newActor = Actor.createActor(idlFactory, {
                    agent,
                    canisterId,
                });

                setActor(newActor);
            } catch (err) {
                console.error("Error initializing API:", err);
                setError(err.message || "Failed to initialize API connection");
                setActor(null);
            } finally {
                setIsLoading(false);
            }
        };

        initActor();
    }, [isAuthenticated, identity]);

    return {
        actor,
        isLoading,
        error,
        canisterId,
    };
};