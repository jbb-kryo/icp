import React, { useState } from "react";
import { Link } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";
import { useTheme } from "../context/ThemeContext";
import {
    FiMenu,
    FiX,
    FiSun,
    FiMoon,
    FiSettings,
    FiLogOut,
    FiLogIn
} from "react-icons/fi";

const Header = () => {
    const { isAuthenticated, login, logout } = useAuth();
    const { theme, toggleTheme } = useTheme();
    const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

    const toggleMobileMenu = () => {
        setMobileMenuOpen(!mobileMenuOpen);
    };

    return (
        <header className="bg-white dark:bg-gray-800 shadow-sm">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div className="flex justify-between h-16">
                    <div className="flex">
                        {/* Logo */}
                        <div className="flex-shrink-0 flex items-center">
                            <Link to="/" className="flex items-center">
                                <img
                                    className="h-8 w-auto"
                                    src="/logo.svg"
                                    alt="Messagr Logo"
                                />
                                <span className="ml-2 text-xl font-bold text-indigo-600 dark:text-indigo-400">
                                    Messagr
                                </span>
                            </Link>
                        </div>

                        {/* Desktop Navigation */}
                        <nav className="hidden sm:ml-6 sm:flex sm:space-x-8">
                            {isAuthenticated && (
                                <>
                                    <Link
                                        to="/"
                                        className="inline-flex items-center px-1 pt-1 border-b-2 border-transparent text-sm font-medium text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-300 dark:hover:text-white dark:hover:border-gray-500"
                                    >
                                        Dashboard
                                    </Link>
                                    <Link
                                        to="/conversations"
                                        className="inline-flex items-center px-1 pt-1 border-b-2 border-transparent text-sm font-medium text-gray-500 hover:text-gray-700 hover:border-gray-300 dark:text-gray-300 dark:hover:text-white dark:hover:border-gray-500"
                                    >
                                        Conversations
                                    </Link>
                                </>
                            )}
                        </nav>
                    </div>

                    {/* Right side actions */}
                    <div className="flex items-center">
                        {/* Theme toggle */}
                        <button
                            onClick={toggleTheme}
                            className="p-2 rounded-md text-gray-500 hover:text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-white dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                        >
                            {theme === "dark" ? <FiSun size={20} /> : <FiMoon size={20} />}
                        </button>

                        {/* Settings (if authenticated) */}
                        {isAuthenticated && (
                            <Link
                                to="/settings"
                                className="ml-3 p-2 rounded-md text-gray-500 hover:text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-white dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                            >
                                <FiSettings size={20} />
                            </Link>
                        )}

                        {/* Auth button */}
                        <div className="ml-3">
                            {isAuthenticated ? (
                                <button
                                    onClick={logout}
                                    className="btn btn-secondary flex items-center"
                                >
                                    <FiLogOut className="mr-2" />
                                    <span>Logout</span>
                                </button>
                            ) : (
                                <button
                                    onClick={login}
                                    className="btn btn-primary flex items-center"
                                >
                                    <FiLogIn className="mr-2" />
                                    <span>Login</span>
                                </button>
                            )}
                        </div>

                        {/* Mobile menu button */}
                        <div className="sm:hidden ml-3">
                            <button
                                onClick={toggleMobileMenu}
                                className="p-2 rounded-md text-gray-500 hover:text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:text-white dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                            >
                                {mobileMenuOpen ? <FiX size={24} /> : <FiMenu size={24} />}
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            {/* Mobile menu */}
            {mobileMenuOpen && (
                <div className="sm:hidden bg-white dark:bg-gray-800 shadow-lg">
                    <div className="pt-2 pb-3 space-y-1">
                        {isAuthenticated ? (
                            <>
                                <Link
                                    to="/"
                                    className="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-gray-900 hover:bg-gray-50 dark:text-gray-200 dark:hover:text-white dark:hover:bg-gray-700"
                                    onClick={toggleMobileMenu}
                                >
                                    Dashboard
                                </Link>
                                <Link
                                    to="/conversations"
                                    className="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-gray-900 hover:bg-gray-50 dark:text-gray-200 dark:hover:text-white dark:hover:bg-gray-700"
                                    onClick={toggleMobileMenu}
                                >
                                    Conversations
                                </Link>
                                <Link
                                    to="/settings"
                                    className="block px-3 py-2 rounded-md text-base font-medium text-gray-700 hover:text-gray-900 hover:bg-gray-50 dark:text-gray-200 dark:hover:text-white dark:hover:bg-gray-700"
                                    onClick={toggleMobileMenu}
                                >
                                    Settings
                                </Link>
                            </>
                        ) : (
                            <div className="px-3 py-2">
                                <button
                                    onClick={() => {
                                        login();
                                        toggleMobileMenu();
                                    }}
                                    className="w-full btn btn-primary"
                                >
                                    Login with Internet Identity
                                </button>
                            </div>
                        )}
                    </div>
                </div>
            )}
        </header>
    );
};

export default Header;