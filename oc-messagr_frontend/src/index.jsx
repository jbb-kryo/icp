import React from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter as Router } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "react-query";
import App from "./components/App";
import { AuthProvider } from "./context/AuthContext";
import { PlatformProvider } from "./context/PlatformContext";
import { ThemeProvider } from "./context/ThemeContext";
import "./assets/styles/main.css";

// Create a client for React Query
const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            refetchOnWindowFocus: false,
            retry: 1,
            staleTime: 30000,
        },
    },
});

const root = createRoot(document.getElementById("root"));

root.render(
    <React.StrictMode>
        <Router>
            <QueryClientProvider client={queryClient}>
                <AuthProvider>
                    <PlatformProvider>
                        <ThemeProvider>
                            <App />
                        </ThemeProvider>
                    </PlatformProvider>
                </AuthProvider>
            </QueryClientProvider>
        </Router>
    </React.StrictMode>
);