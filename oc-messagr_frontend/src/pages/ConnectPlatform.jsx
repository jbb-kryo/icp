import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { usePlatforms } from "../hooks/usePlatforms";
import {
    FaArrowLeft,
    FaTelegram,
    FaSlack,
    FaDiscord,
    FaTwitter,
    FaFacebook,
    FaWhatsapp,
} from "react-icons/fa";
import { FiAlertCircle, FiCheck } from "react-icons/fi";

const ConnectPlatform = () => {
    const { platform } = useParams();
    const navigate = useNavigate();
    const { connectPlatform, isConnectingPlatform } = usePlatforms();
    const [formData, setFormData] = useState({});
    const [error, setError] = useState(null);
    const [success, setSuccess] = useState(false);

    useEffect(() => {
        // Reset form when platform changes
        setFormData({});
        setError(null);
        setSuccess(false);
    }, [platform]);

    const getPlatformIcon = () => {
        switch (platform.toLowerCase()) {
            case "telegram":
                return <FaTelegram className="text-blue-500" size={40} />;
            case "slack":
                return <FaSlack className="text-purple-700" size={40} />;
            case "discord":
                return <FaDiscord className="text-indigo-500" size={40} />;
            case "twitter":
                return <FaTwitter className="text-blue-400" size={40} />;
            case "facebook":
                return <FaFacebook className="text-blue-600" size={40} />;
            case "whatsapp":
                return <FaWhatsapp className="text-green-500" size={40} />;
            default:
                return null;
        }
    };

    const getPlatformName = () => {
        return platform.charAt(0).toUpperCase() + platform.slice(1);
    };

    const getFormFields = () => {
        switch (platform.toLowerCase()) {
            case "telegram":
                return [
                    {
                        id: "token",
                        label: "Bot Token",
                        placeholder: "Enter your Telegram Bot Token",
                        type: "password",
                        required: true,
                        helperText: "You can get this from BotFather",
                    },
                ];
            case "slack":
                return [
                    {
                        id: "token",
                        label: "OAuth Access Token",
                        placeholder: "Enter your Slack OAuth Token",
                        type: "password",
                        required: true,
                        helperText: "OAuth token from Slack App credentials",
                    },
                    {
                        id: "api_key",
                        label: "Client ID",
                        placeholder: "Enter your Slack Client ID",
                        type: "text",
                        required: true,
                    },
                    {
                        id: "api_secret",
                        label: "Client Secret",
                        placeholder: "Enter your Slack Client Secret",
                        type: "password",
                        required: true,
                    },
                    {
                        id: "redirect_uri",
                        label: "Redirect URI",
                        placeholder: "Enter your redirect URI",
                        type: "text",
                        required: true,
                        defaultValue: window.location.origin + "/slack/callback",
                    },
                ];
            case "discord":
                return [
                    {
                        id: "token",
                        label: "Bot Token",
                        placeholder: "Enter your Discord Bot Token",
                        type: "password",
                        required: true,
                        helperText: "From Discord Developer Portal",
                    },
                    {
                        id: "api_key",
                        label: "Client ID",
                        placeholder: "Enter your Discord Client ID",
                        type: "text",
                        required: true,
                    },
                    {
                        id: "api_secret",
                        label: "Client Secret",
                        placeholder: "Enter your Discord Client Secret",
                        type: "password",
                        required: true,
                    },
                ];
            case "twitter":
                return [
                    {
                        id: "token",
                        label: "OAuth Token",
                        placeholder: "Enter your Twitter OAuth Token and Secret",
                        type: "password",
                        required: true,
                        helperText: "Format: token:secret",
                    },
                    {
                        id: "api_key",
                        label: "API Key (Consumer Key)",
                        placeholder: "Enter your Twitter API Key",
                        type: "text",
                        required: true,
                    },
                    {
                        id: "api_secret",
                        label: "API Secret (Consumer Secret)",
                        placeholder: "Enter your Twitter API Secret",
                        type: "password",
                        required: true,
                    },
                ];
            case "facebook":
                return [
                    {
                        id: "token",
                        label: "Page Access Token",
                        placeholder: "Enter your Facebook Page Access Token",
                        type: "password",
                        required: true,
                        helperText: "Long-lived token from Facebook Developer Console",
                    },
                    {
                        id: "api_key",
                        label: "App ID",
                        placeholder: "Enter your Facebook App ID",
                        type: "text",
                        required: true,
                    },
                    {
                        id: "api_secret",
                        label: "App Secret",
                        placeholder: "Enter your Facebook App Secret",
                        type: "password",
                        required: true,
                    },
                    {
                        id: "redirect_uri",
                        label: "Redirect URI",
                        placeholder: "Enter your redirect URI",
                        type: "text",
                        required: true,
                        defaultValue: window.location.origin + "/facebook/callback",
                    },
                ];
            case "whatsapp":
                return [
                    {
                        id: "token",
                        label: "Access Token",
                        placeholder: "Enter your WhatsApp Business API Token",
                        type: "password",
                        required: true,
                        helperText: "From WhatsApp Business Platform",
                    },
                    {
                        id: "api_key",
                        label: "Phone Number ID",
                        placeholder: "Enter your WhatsApp Phone Number ID",
                        type: "text",
                        required: true,
                    },
                    {
                        id: "api_secret",
                        label: "App Secret",
                        placeholder: "Enter your WhatsApp App Secret",
                        type: "password",
                        required: true,
                    },
                ];
            default:
                return [];
        }
    };

    const handleChange = (e) => {
        setFormData({
            ...formData,
            [e.target.id]: e.target.value,
        });
        setError(null);
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
        setError(null);

        try {
            const platformEnum = getPlatformName();

            // Format data for the API
            const authConfig = {
                platform: platformEnum,
                token: formData.token || "",
                api_key: formData.api_key || null,
                api_secret: formData.api_secret || null,
                redirect_uri: formData.redirect_uri || null,
            };

            await connectPlatform(authConfig);
            setSuccess(true);

            // Redirect after a delay
            setTimeout(() => {
                navigate("/settings");
            }, 2000);
        } catch (error) {
            console.error("Error connecting platform:", error);
            setError(error.message || "Failed to connect platform. Please check your credentials.");
        }
    };

    return (
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <div className="mb-6">
                <button
                    onClick={() => navigate("/settings")}
                    className="inline-flex items-center text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300"
                >
                    <FaArrowLeft className="mr-2" />
                    <span>Back to Settings</span>
                </button>
            </div>

            <div className="card max-w-2xl mx-auto">
                <div className="p-6">
                    <div className="flex flex-col items-center mb-6">
                        {getPlatformIcon()}
                        <h1 className="text-2xl font-bold text-gray-900 dark:text-white mt-4">
                            Connect to {getPlatformName()}
                        </h1>
                        <p className="text-gray-500 dark:text-gray-400 mt-2 text-center">
                            Enter your credentials to connect to {getPlatformName()}
                        </p>
                    </div>

                    {error && (
                        <div className="bg-red-100 dark:bg-red-900 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-200 px-4 py-3 rounded-md mb-6 flex items-start">
                            <FiAlertCircle className="mr-2 mt-0.5 flex-shrink-0" />
                            <span>{error}</span>
                        </div>
                    )}

                    {success && (
                        <div className="bg-green-100 dark:bg-green-900 border border-green-200 dark:border-green-800 text-green-700 dark:text-green-200 px-4 py-3 rounded-md mb-6 flex items-center">
                            <FiCheck className="mr-2 flex-shrink-0" />
                            <span>Successfully connected to {getPlatformName()}! Redirecting...</span>
                        </div>
                    )}

                    <form onSubmit={handleSubmit}>
                        <div className="space-y-4">
                            {getFormFields().map((field) => (
                                <div key={field.id}>
                                    <label
                                        htmlFor={field.id}
                                        className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
                                    >
                                        {field.label} {field.required && <span className="text-red-500">*</span>}
                                    </label>
                                    <input
                                        type={field.type}
                                        id={field.id}
                                        placeholder={field.placeholder}
                                        className="input"
                                        required={field.required}
                                        defaultValue={field.defaultValue}
                                        onChange={handleChange}
                                    />
                                    {field.helperText && (
                                        <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                                            {field.helperText}
                                        </p>
                                    )}
                                </div>
                            ))}
                        </div>

                        <div className="mt-8">
                            <button
                                type="submit"
                                disabled={isConnectingPlatform || success}
                                className={`w-full btn btn-primary ${(isConnectingPlatform || success) ? "opacity-70 cursor-not-allowed" : ""
                                    }`}
                            >
                                {isConnectingPlatform ? (
                                    <div className="flex items-center justify-center">
                                        <div className="animate-spin h-5 w-5 border-2 border-white border-t-transparent rounded-full mr-2"></div>
                                        <span>Connecting...</span>
                                    </div>
                                ) : success ? (
                                    <div className="flex items-center justify-center">
                                        <FiCheck className="mr-2" />
                                        <span>Connected!</span>
                                    </div>
                                ) : (
                                    `Connect to ${getPlatformName()}`
                                )}
                            </button>
                        </div>
                    </form>

                    <div className="mt-6 text-sm text-gray-500 dark:text-gray-400">
                        <h3 className="font-medium mb-2">How to get {getPlatformName()} credentials:</h3>
                        <ul className="list-disc pl-5 space-y-1">
                            {platform.toLowerCase() === "telegram" && (
                                <>
                                    <li>Talk to @BotFather on Telegram</li>
                                    <li>Create a new bot using /newbot command</li>
                                    <li>Copy the API token provided by BotFather</li>
                                </>
                            )}
                            {platform.toLowerCase() === "slack" && (
                                <>
                                    <li>Go to the Slack API website and create a new app</li>
                                    <li>Add OAuth scopes: channels:history, chat:write, users:read</li>
                                    <li>Install the app to your workspace</li>
                                    <li>Copy the OAuth token, Client ID, and Client Secret</li>
                                </>
                            )}
                            {platform.toLowerCase() === "discord" && (
                                <>
                                    <li>Go to the Discord Developer Portal</li>
                                    <li>Create a new application</li>
                                    <li>Go to the Bot section and create a bot</li>
                                    <li>Copy the token, client ID, and client secret</li>
                                </>
                            )}
                            {platform.toLowerCase() === "twitter" && (
                                <>
                                    <li>Go to the Twitter Developer Portal</li>
                                    <li>Create a new project and app</li>
                                    <li>Generate consumer keys and access tokens</li>
                                    <li>Copy the API key, API secret, and access tokens</li>
                                </>
                            )}
                            {platform.toLowerCase() === "facebook" && (
                                <>
                                    <li>Go to the Facebook Developer Portal</li>
                                    <li>Create a new app</li>
                                    <li>Add the Messenger product</li>
                                    <li>Generate a page access token</li>
                                    <li>Copy the app ID, app secret, and page access token</li>
                                </>
                            )}
                            {platform.toLowerCase() === "whatsapp" && (
                                <>
                                    <li>Go to the WhatsApp Business Platform</li>
                                    <li>Set up a business account</li>
                                    <li>Go to the API setup section</li>
                                    <li>Copy the phone number ID, access token, and app secret</li>
                                </>
                            )}
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default ConnectPlatform;