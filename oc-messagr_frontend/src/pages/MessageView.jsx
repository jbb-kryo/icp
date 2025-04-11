import React, { useState, useEffect, useRef } from "react";
import { useParams, Link } from "react-router-dom";
import { useMessages } from "../hooks/useMessages";
import { useConversations } from "../hooks/useConversations";
import ConversationInsights from "../components/ConversationInsights";
import { format } from "date-fns";
import {
    FiArrowLeft,
    FiUsers,
    FiMoreVertical,
    FiRefreshCw,
    FiDownload,
    FiClock,
    FiPaperclip,
    FiImage,
    FiFile,
    FiLink,
    FiZap
} from "react-icons/fi";
import {
    FaTelegram,
    FaSlack,
    FaDiscord,
    FaTwitter,
    FaFacebook,
    FaWhatsapp,
} from "react-icons/fa";

const MessageView = () => {
    const { id } = useParams();
    const { getConversation, isLoadingConversation } = useConversations();
    const { getMessages, isLoadingMessages, syncMessages } = useMessages();
    const [conversation, setConversation] = useState(null);
    const [messages, setMessages] = useState([]);
    const [isSyncing, setIsSyncing] = useState(false);
    const [showInfo, setShowInfo] = useState(false);
    const [showAIInsights, setShowAIInsights] = useState(false);
    const messagesEndRef = useRef(null);

    useEffect(() => {
        const fetchConversation = async () => {
            try {
                const conv = await getConversation(id);
                setConversation(conv);
            } catch (error) {
                console.error("Error fetching conversation:", error);
            }
        };

        const fetchMessages = async () => {
            try {
                const msgs = await getMessages(id);
                setMessages(msgs);
            } catch (error) {
                console.error("Error fetching messages:", error);
            }
        };

        fetchConversation();
        fetchMessages();
    }, [id, getConversation, getMessages]);

    useEffect(() => {
        scrollToBottom();
    }, [messages]);

    const scrollToBottom = () => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    };

    const handleSync = async () => {
        setIsSyncing(true);
        try {
            await syncMessages(conversation.platform, id);
            const msgs = await getMessages(id);
            setMessages(msgs);
        } catch (error) {
            console.error("Error syncing messages:", error);
        } finally {
            setIsSyncing(false);
        }
    };

    const formatTimestamp = (timestamp) => {
        return format(new Date(timestamp), "MMM d, yyyy h:mm a");
    };

    const getPlatformIcon = (platform) => {
        switch (platform?.toLowerCase()) {
            case "telegram":
                return <FaTelegram className="text-blue-500" />;
            case "slack":
                return <FaSlack className="text-purple-700" />;
            case "discord":
                return <FaDiscord className="text-indigo-500" />;
            case "twitter":
                return <FaTwitter className="text-blue-400" />;
            case "facebook":
                return <FaFacebook className="text-blue-600" />;
            case "whatsapp":
                return <FaWhatsapp className="text-green-500" />;
            default:
                return null;
        }
    };

    const renderAttachment = (attachment) => {
        switch (attachment.attachment_type.toLowerCase()) {
            case "image":
                return (
                    <div className="mt-2 rounded-lg overflow-hidden border border-gray-200 dark:border-gray-700 inline-block">
                        {attachment.url ? (
                            <a
                                href={attachment.url}
                                target="_blank"
                                rel="noopener noreferrer"
                                className="block"
                            >
                                <img
                                    src={attachment.url}
                                    alt={attachment.name || "Image"}
                                    className="max-h-60 max-w-full object-contain"
                                />
                            </a>
                        ) : (
                            <div className="bg-gray-100 dark:bg-gray-800 p-4 text-center">
                                <FiImage className="h-8 w-8 mx-auto text-gray-400" />
                                <span className="text-sm text-gray-500 dark:text-gray-400 block mt-1">
                                    {attachment.name || "Image"}
                                </span>
                            </div>
                        )}
                    </div>
                );
            case "file":
            case "document":
                return (
                    <div className="mt-2 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 inline-flex items-center">
                        <FiFile className="h-5 w-5 text-gray-400 mr-2" />
                        {attachment.url ? (
                            <a
                                href={attachment.url}
                                target="_blank"
                                rel="noopener noreferrer"
                                className="text-indigo-600 dark:text-indigo-400 hover:underline text-sm"
                            >
                                {attachment.name || "File"}
                            </a>
                        ) : (
                            <span className="text-sm text-gray-500 dark:text-gray-400">
                                {attachment.name || "File"}
                            </span>
                        )}
                    </div>
                );
            case "link":
                return (
                    <div className="mt-2 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 inline-flex items-center">
                        <FiLink className="h-5 w-5 text-gray-400 mr-2" />
                        {attachment.url ? (
                            <a
                                href={attachment.url}
                                target="_blank"
                                rel="noopener noreferrer"
                                className="text-indigo-600 dark:text-indigo-400 hover:underline text-sm"
                            >
                                {attachment.name || attachment.url}
                            </a>
                        ) : (
                            <span className="text-sm text-gray-500 dark:text-gray-400">
                                {attachment.name || "Link"}
                            </span>
                        )}
                    </div>
                );
            default:
                return (
                    <div className="mt-2 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 inline-flex items-center">
                        <FiPaperclip className="h-5 w-5 text-gray-400 mr-2" />
                        <span className="text-sm text-gray-500 dark:text-gray-400">
                            {attachment.name || `${attachment.attachment_type} attachment`}
                        </span>
                    </div>
                );
        }
    };

    if (isLoadingConversation || !conversation) {
        return (
            <div className="flex justify-center items-center h-96">
                <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-600"></div>
            </div>
        );
    }

    return (
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            {/* Header */}
            <div className="flex items-center justify-between mb-6">
                <div className="flex items-center">
                    <Link
                        to="/conversations"
                        className="mr-4 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300"
                    >
                        <FiArrowLeft size={20} />
                    </Link>
                    <div className="flex items-center">
                        <div className="mr-3">
                            {getPlatformIcon(conversation.platform)}
                        </div>
                        <div>
                            <h1 className="text-xl font-bold text-gray-900 dark:text-white">
                                {conversation.name}
                            </h1>
                            <div className="flex items-center text-sm text-gray-500 dark:text-gray-400">
                                <span className={`platform-badge platform-${conversation.platform.toLowerCase()}`}>
                                    {conversation.platform}
                                </span>
                                <span className="mx-2">•</span>
                                <FiUsers className="mr-1" />
                                <span>{conversation.participants.length} participants</span>
                            </div>
                        </div>
                    </div>
                </div>
                <div className="flex items-center">
                    <button
                        onClick={handleSync}
                        disabled={isSyncing}
                        className="mr-3 btn btn-secondary"
                    >
                        {isSyncing ? (
                            <div className="flex items-center">
                                <div className="animate-spin h-4 w-4 border-2 border-indigo-600 border-t-transparent rounded-full mr-2"></div>
                                <span>Syncing</span>
                            </div>
                        ) : (
                            <>
                                <FiRefreshCw className="mr-2" />
                                <span>Sync</span>
                            </>
                        )}
                    </button>
                    <button
                        onClick={() => setShowAIInsights(!showAIInsights)}
                        className={`mr-3 btn ${showAIInsights ? 'btn-primary' : 'btn-secondary'}`}
                    >
                        <FiZap className="mr-2" />
                        <span>AI Insights</span>
                    </button>
                    <button
                        onClick={() => setShowInfo(!showInfo)}
                        className={`btn ${showInfo ? 'btn-primary' : 'btn-secondary'}`}
                    >
                        <FiUsers className="mr-2" />
                        <span>Info</span>
                    </button>
                </div>
            </div>

            <div className="flex flex-col md:flex-row gap-6">
                {/* Main message view */}
                <div className={`flex-1 ${showAIInsights ? 'md:w-7/12' : 'w-full'}`}>
                    <div className="card h-[calc(100vh-200px)] flex flex-col">
                        {/* Messages container */}
                        <div className="flex-1 overflow-y-auto p-4">
                            {isLoadingMessages ? (
                                <div className="flex justify-center items-center h-full">
                                    <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-indigo-600"></div>
                                </div>
                            ) : messages.length > 0 ? (
                                <div className="space-y-4">
                                    {messages.map((message) => {
                                        const isCurrentUser = message.sender.id === "current_user"; // Replace with actual logic

                                        return (
                                            <div
                                                key={message.id}
                                                className={`flex ${isCurrentUser ? "justify-end" : "justify-start"
                                                    }`}
                                            >
                                                <div
                                                    className={`max-w-3/4 ${isCurrentUser
                                                            ? "bg-indigo-100 dark:bg-indigo-900 rounded-tl-lg rounded-tr-lg rounded-bl-lg"
                                                            : "bg-gray-100 dark:bg-gray-800 rounded-tr-lg rounded-tl-lg rounded-br-lg"
                                                        } p-3 shadow-sm`}
                                                >
                                                    <div className="flex items-center mb-1">
                                                        <span className="font-medium text-gray-900 dark:text-white">
                                                            {message.sender.name}
                                                        </span>
                                                        <span className="ml-2 text-xs text-gray-500 dark:text-gray-400">
                                                            {formatTimestamp(message.timestamp)}
                                                        </span>
                                                        {message.edited && (
                                                            <span className="ml-2 text-xs text-gray-500 dark:text-gray-400 italic">
                                                                (edited)
                                                            </span>
                                                        )}
                                                    </div>
                                                    <p className="text-gray-800 dark:text-gray-200">
                                                        {message.content.text}
                                                    </p>

                                                    {message.content.attachments &&
                                                        message.content.attachments.length > 0 && (
                                                            <div className="mt-2 space-y-2">
                                                                {message.content.attachments.map(
                                                                    (attachment, index) => (
                                                                        <div key={index}>
                                                                            {renderAttachment(attachment)}
                                                                        </div>
                                                                    )
                                                                )}
                                                            </div>
                                                        )}

                                                    {message.reply_to && (
                                                        <div className="mt-2 text-xs text-gray-500 dark:text-gray-400 border-l-2 border-gray-300 dark:border-gray-600 pl-2">
                                                            Replying to message
                                                        </div>
                                                    )}
                                                </div>
                                            </div>
                                        );
                                    })}
                                    <div ref={messagesEndRef} />
                                </div>
                            ) : (
                                <div className="flex flex-col items-center justify-center h-full text-center">
                                    <FiClock className="h-12 w-12 text-gray-400 mb-4" />
                                    <h3 className="text-lg font-medium text-gray-900 dark:text-white">
                                        No messages yet
                                    </h3>
                                    <p className="mt-1 text-gray-500 dark:text-gray-400">
                                        Try syncing your messages using the Sync button above.
                                    </p>
                                </div>
                            )}
                        </div>
                    </div>
                </div>

                {/* AI Insights panel */}
                {showAIInsights && (
                    <div className="w-full md:w-5/12 flex-shrink-0">
                        <ConversationInsights conversationId={id} />
                    </div>
                )}

                {/* Info sidebar */}
                {showInfo && (
                    <div className="w-full md:w-80 flex-shrink-0">
                        <div className="card h-[calc(100vh-200px)] overflow-y-auto">
                            <div className="p-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
                                <h2 className="text-lg font-medium text-gray-900 dark:text-white">
                                    Conversation Info
                                </h2>
                                <button
                                    onClick={() => setShowInfo(false)}
                                    className="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300"
                                >
                                    <FiMoreVertical />
                                </button>
                            </div>

                            <div className="p-4">
                                <h3 className="font-medium text-gray-900 dark:text-white mb-2">
                                    Participants
                                </h3>
                                <ul className="space-y-3">
                                    {conversation.participants.map((participant) => (
                                        <li
                                            key={participant.id}
                                            className="flex items-center"
                                        >
                                            <div className="h-8 w-8 rounded-full bg-gray-200 dark:bg-gray-700 flex items-center justify-center overflow-hidden">
                                                {participant.avatar_url ? (
                                                    <img
                                                        src={participant.avatar_url}
                                                        alt={participant.name}
                                                        className="h-full w-full object-cover"
                                                    />
                                                ) : (
                                                    <span className="text-sm font-medium text-gray-500 dark:text-gray-400">
                                                        {participant.name.charAt(0)}
                                                    </span>
                                                )}
                                            </div>
                                            <div className="ml-3">
                                                <p className="text-sm font-medium text-gray-900 dark:text-white">
                                                    {participant.name}
                                                </p>
                                            </div>
                                        </li>
                                    ))}
                                </ul>
                            </div>

                            <div className="p-4 border-t border-gray-200 dark:border-gray-700">
                                <h3 className="font-medium text-gray-900 dark:text-white mb-2">
                                    Details
                                </h3>
                                <dl className="space-y-2">
                                    <div>
                                        <dt className="text-sm text-gray-500 dark:text-gray-400">
                                            Platform
                                        </dt>
                                        <dd className="text-sm text-gray-900 dark:text-white flex items-center mt-1">
                                            {getPlatformIcon(conversation.platform)}
                                            <span className="ml-2">{conversation.platform}</span>
                                        </dd>
                                    </div>
                                    <div>
                                        <dt className="text-sm text-gray-500 dark:text-gray-400">
                                            Created
                                        </dt>
                                        <dd className="text-sm text-gray-900 dark:text-white mt-1">
                                            {formatTimestamp(conversation.created_at)}
                                        </dd>
                                    </div>
                                    <div>
                                        <dt className="text-sm text-gray-500 dark:text-gray-400">
                                            Last Activity
                                        </dt>
                                        <dd className="text-sm text-gray-900 dark:text-white mt-1">
                                            {conversation.last_message_at
                                                ? formatTimestamp(conversation.last_message_at)
                                                : "No activity"}
                                        </dd>
                                    </div>
                                    <div>
                                        <dt className="text-sm text-gray-500 dark:text-gray-400">
                                            Message Count
                                        </dt>
                                        <dd className="text-sm text-gray-900 dark:text-white mt-1">
                                            {messages.length}
                                        </dd>
                                    </div>
                                </dl>
                            </div>

                            <div className="p-4 border-t border-gray-200 dark:border-gray-700">
                                <h3 className="font-medium text-gray-900 dark:text-white mb-2">
                                    Actions
                                </h3>
                                <div className="space-y-2">
                                    <button className="btn btn-secondary w-full justify-center">
                                        <FiDownload className="mr-2" />
                                        <span>Export Messages</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default MessageView;