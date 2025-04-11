import React, { createContext, useContext, useState, useEffect } from "react";

const ThemeContext = createContext(null);

export const useTheme = () => useContext(ThemeContext);

export const ThemeProvider = ({ children }) => {
    // Check for user preference in localStorage or use system preference
    const getInitialTheme = () => {
        const savedTheme = localStorage.getItem("messagrTheme");

        if (savedTheme) {
            return savedTheme;
        }

        // Check system preference
        if (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches) {
            return "dark";
        }

        return "light";
    };

    const [theme, setTheme] = useState(getInitialTheme);

    // Apply theme to document when it changes
    useEffect(() => {
        if (theme === "dark") {
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }

        // Save to localStorage
        localStorage.setItem("messagrTheme", theme);
    }, [theme]);

    // Toggle between light and dark mode
    const toggleTheme = () => {
        setTheme((prevTheme) => (prevTheme === "light" ? "dark" : "light"));
    };

    // Set theme directly
    const setThemeMode = (mode) => {
        if (mode === "light" || mode === "dark") {
            setTheme(mode);
        }
    };

    const value = {
        theme,
        toggleTheme,
        setTheme: setThemeMode,
        isDark: theme === "dark",
        isLight: theme === "light",
    };

    return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
};