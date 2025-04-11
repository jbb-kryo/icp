import React, { useState, useEffect } from "react";
import { useQuery } from "../hooks/useQuery";
import { FiDatabase, FiRefreshCw, FiAlertTriangle, FiCheckCircle } from "react-icons/fi";

const AdminPanel = () => {
    const { getIndexStats, optimizeIndices, rebuildIndices } = useQuery();
    const [stats, setStats] = useState(null);
    const [isLoading, setIsLoading] = useState(true);
    const [isOptimizing, setIsOptimizing] = useState(false);
    const [isRebuilding, setIsRebuilding] = useState(false);
    const [error, setError] = useState(null);
    const [success, setSuccess] = useState(null);

    useEffect(() => {
        fetchStats();
    }, []);

    const fetchStats = async () => {
        try {
            setIsLoading(true);
            setError(null);
            const indexStats = await getIndexStats();
            setStats(indexStats);
        } catch (err) {
            setError("Failed to load index statistics");
            console.error(err);
        } finally {
            setIsLoading(false);
        }
    };

    const handleOptimize = async () => {
        try {
            setIsOptimizing(true);
            setError(null);
            setSuccess(null);

            await optimizeIndices();
            setSuccess("Indices optimized successfully");

            // Refresh stats
            await fetchStats();
        } catch (err) {
            setError("Failed to optimize indices: " + err.message);
            console.error(err);
        } finally {
            setIsOptimizing(false);
        }
    };

    const handleRebuild = async () => {
        try {
            setIsRebuilding(true);
            setError(null);
            setSuccess(null);

            await rebuildIndices();
            setSuccess("Indices rebuilt successfully");

            // Refresh stats
            await fetchStats();
        } catch (err) {
            setError("Failed to rebuild indices: " + err.message);
            console.error(err);
        } finally {
            setIsRebuilding(false);
        }
    };

    const formatBytes = (bytes) => {
        if (bytes === 0) return "0 Bytes";

        const k = 1024;
        const sizes = ["Bytes", "KB", "MB", "GB"];
        const i = Math.floor(Math.log(bytes) / Math.log(k));

        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
    };

    const formatTimestamp = (timestamp) => {
        if (!timestamp) return "Never";
        return new Date(timestamp).toLocaleString();
    };

    return (
        <div className="card">
            <div className="card-header flex justify-between items-center">
                <h2 className="text-lg font-medium">Search Index Management</h2>
                <button
                    onClick={fetchStats}
                    className="btn btn-secondary btn-sm"
                    disabled={isLoading}
                >
                    <FiRefreshCw className={`mr-2 ${isLoading ? "animate-spin" : ""}`} />
                    <span>Refresh</span>
                </button>
            </div>

            <div className="card-body">
                {error && (
                    <div className="mb-4 p-3 bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200 rounded-md flex items-center">
                        <FiAlertTriangle className="mr-2 flex-shrink-0" />
                        <span>{error}</span>
                    </div>
                )}

                {success && (
                    <div className="mb-4 p-3 bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-200 rounded-md flex items-center">
                        <FiCheckCircle className="mr-2 flex-shrink-0" />
                        <span>{success}</span>
                    </div>
                )}

                {isLoading ? (
                    <div className="py-8 flex justify-center">
                        <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-indigo-600"></div>
                    </div>
                ) : stats ? (
                    <div>
                        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
                            <div className="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg border border-gray-200 dark:border-gray-700">
                                <div className="text-sm text-gray-500 dark:text-gray-400">Total Messages</div>
                                <div className="text-2xl font-semibold mt-1">{stats.message_count.toLocaleString()}</div>
                            </div>

                            <div className="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg border border-gray-200 dark:border-gray-700">
                                <div className="text-sm text-gray-500 dark:text-gray-400">Indexed Messages</div>
                                <div className="text-2xl font-semibold mt-1">{stats.indexed_count.toLocaleString()}</div>
                            </div>

                            <div className="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg border border-gray-200 dark:border-gray-700">
                                <div className="text-sm text-gray-500 dark:text-gray-400">Index Size</div>
                                <div className="text-2xl font-semibold mt-1">{formatBytes(stats.index_size_bytes)}</div>
                            </div>

                            <div className="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg border border-gray-200 dark:border-gray-700">
                                <div className="text-sm text-gray-500 dark:text-gray-400">Last Optimization</div>
                                <div className="text-lg font-semibold mt-1">{formatTimestamp(stats.last_optimization)}</div>
                            </div>
                        </div>

                        <div className="space-y-4">
                            <div>
                                <h3 className="text-md font-medium mb-2">Index Health</h3>
                                <div className="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg border border-gray-200 dark:border-gray-700">
                                    <div className="mb-2 flex items-center">
                                        <span className="text-sm text-gray-700 dark:text-gray-300 mr-2">Indexing Status:</span>
                                        {stats.indexed_count === stats.message_count ? (
                                            <span className="text-green-600 dark:text-green-400 flex items-center">
                                                <FiCheckCircle className="mr-1" />
                                                All messages indexed
                                            </span>
                                        ) : (
                                            <span className="text-yellow-600 dark:text-yellow-400 flex items-center">
                                                <FiAlertTriangle className="mr-1" />
                                                {stats.message_count - stats.indexed_count} messages need indexing
                                            </span>
                                        )}
                                    </div>

                                    <div className="mb-2 flex items-center">
                                        <span className="text-sm text-gray-700 dark:text-gray-300 mr-2">Optimization:</span>
                                        {stats.last_optimization ? (
                                            <span className="text-green-600 dark:text-green-400 flex items-center">
                                                <FiCheckCircle className="mr-1" />
                                                Last optimized: {formatTimestamp(stats.last_optimization)}
                                            </span>
                                        ) : (
                                            <span className="text-yellow-600 dark:text-yellow-400 flex items-center">
                                                <FiAlertTriangle className="mr-1" />
                                                Never optimized
                                            </span>
                                        )}
                                    </div>
                                </div>
                            </div>

                            <div>
                                <h3 className="text-md font-medium mb-2">Maintenance Actions</h3>
                                <div className="bg-gray-50 dark:bg-gray-800 p-4 rounded-lg border border-gray-200 dark:border-gray-700">
                                    <div className="flex flex-col md:flex-row gap-4">
                                        <div className="flex-1">
                                            <h4 className="text-sm font-medium mb-1">Optimize Indices</h4>
                                            <p className="text-xs text-gray-500 dark:text-gray-400 mb-2">
                                                Optimize indices for better search performance. This doesn't change search results, but makes queries faster.
                                            </p>
                                            <button
                                                onClick={handleOptimize}
                                                disabled={isOptimizing}
                                                className="btn btn-primary btn-sm w-full"
                                            >
                                                {isOptimizing ? (
                                                    <div className="flex items-center justify-center">
                                                        <div className="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full mr-2"></div>
                                                        <span>Optimizing...</span>
                                                    </div>
                                                ) : (
                                                    <span>Optimize Now</span>
                                                )}
                                            </button>
                                        </div>

                                        <div className="flex-1">
                                            <h4 className="text-sm font-medium mb-1">Rebuild Indices</h4>
                                            <p className="text-xs text-gray-500 dark:text-gray-400 mb-2">
                                                Rebuild all search indices from scratch. Use this if search results seem incorrect or incomplete.
                                            </p>
                                            <button
                                                onClick={handleRebuild}
                                                disabled={isRebuilding}
                                                className="btn btn-danger btn-sm w-full"
                                            >
                                                {isRebuilding ? (
                                                    <div className="flex items-center justify-center">
                                                        <div className="animate-spin h-4 w-4 border-2 border-white border-t-transparent rounded-full mr-2"></div>
                                                        <span>Rebuilding...</span>
                                                    </div>
                                                ) : (
                                                    <span>Rebuild All</span>
                                                )}
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                ) : (
                    <div className="py-8 text-center text-gray-500 dark:text-gray-400">
                        <FiDatabase className="mx-auto h-12 w-12 mb-4" />
                        <p>Index statistics unavailable</p>
                    </div>
                )}
            </div>
        </div>
    );
};

export default AdminPanel;