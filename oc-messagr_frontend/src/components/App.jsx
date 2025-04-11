import React, { useEffect } from "react";
import { Routes, Route, Navigate } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";
import Header from "./Header";
import Sidebar from "./Sidebar";
import Dashboard from "../pages/Dashboard";
import Settings from "../pages/Settings";
import ConnectPlatform from "../pages/ConnectPlatform";
import Conversations from "../pages/Conversations";
import MessageView from "../pages/MessageView";

const App = () => {
    const { isAuthenticated, isLoading, login } = useAuth();

    useEffect(() => {
        // Check if user is already authenticated on page load
        const checkAuth = async () => {
            try {
                await login();
            } catch (error) {
                console.error("Authentication error:", error);
            }
        };

        checkAuth();
    }, [login]);

    if (isLoading) {
        return (
            <div className="flex-center h-screen">
                <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-600"></div>
            </div>
        );
    }

    return (
        <>
            <Header />
            <div className="flex flex-1 overflow-hidden">
                {isAuthenticated && <Sidebar />}
                <main className="flex-1 overflow-auto">
                    <Routes>
                        {isAuthenticated ? (
                            <>
                                <Route path="/" element={<Dashboard />} />
                                <Route path="/settings" element={<Settings />} />
                                <Route path="/connect/:platform" element={<ConnectPlatform />} />
                                <Route path="/conversations" element={<Conversations />} />
                                <Route path="/conversations/:id" element={<MessageView />} />
                                <Route path="*" element={<Navigate to="/" replace />} />
                            </>
                        ) : (
                            <>
                                <Route path="/" element={<Dashboard />} />
                                <Route path="*" element={<Navigate to="/" replace />} />
                            </>
                        )}
                    </Routes>
                </main>
            </div>
        </>
    );
};

export default App;