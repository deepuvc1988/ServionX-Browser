import { FC, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface DownloadItem {
    id: string;
    url: string;
    filename: string;
    save_path?: string;
    size: number | null;
    size_total?: number | null;
    downloaded: number;
    size_downloaded?: number;
    status: 'Pending' | 'Downloading' | 'Scanning' | 'Safe' | 'Dangerous' | 'Failed' | 'Cancelled' | 'Completed';
    state?: string;
    scan_result: { Clean: null } | { Suspicious: { reason: string } } | { Malware: { threat_name: string } } | null;
    file_hash: string | null;
    error?: string | null;
    started_at: number;
    completed_at: number | null;
}

interface DownloadManagerProps {
    isOpen: boolean;
    onClose: () => void;
}

const DownloadManager: FC<DownloadManagerProps> = ({ isOpen, onClose }) => {
    const [downloads, setDownloads] = useState<DownloadItem[]>([]);
    const [scanEnabled, setScanEnabled] = useState(true);
    const [stats, setStats] = useState({
        total_downloads: 0,
        total_scanned: 0,
        threats_blocked: 0,
        pending: 0,
        in_progress: 0,
    });

    useEffect(() => {
        if (isOpen) {
            loadDownloads();
            // Poll for download updates every 2 seconds
            const interval = setInterval(loadDownloads, 2000);
            return () => clearInterval(interval);
        }
    }, [isOpen]);

    const loadDownloads = async () => {
        try {
            // Fetch real downloads from backend
            const data = await invoke<DownloadItem[]>('get_downloads');
            setDownloads(data.map(d => ({
                ...d,
                size: d.size_total || null,
                downloaded: d.size_downloaded || 0,
                status: d.state as any,
            })));

            // Update stats
            setStats({
                total_downloads: data.length,
                total_scanned: data.filter(d => d.state === 'Completed' || d.state === 'Safe').length,
                threats_blocked: data.filter(d => d.state === 'Failed' && d.error?.includes('threat')).length,
                pending: data.filter(d => d.state === 'Pending').length,
                in_progress: data.filter(d => d.state === 'Downloading').length,
            });
        } catch (error) {
            console.error('Failed to load downloads:', error);
        }
    };

    // Format functions

    const formatSize = (bytes: number | null) => {
        if (!bytes) return 'Unknown';
        if (bytes < 1024) return `${bytes} B`;
        if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
        return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    };

    const getStatusColor = (status: string) => {
        switch (status) {
            case 'Safe': return '#10b981';
            case 'Dangerous': return '#ef4444';
            case 'Scanning': return '#f59e0b';
            case 'Downloading': return '#3b82f6';
            case 'Failed': return '#ef4444';
            default: return '#64748b';
        }
    };

    const getStatusIcon = (status: string) => {
        switch (status) {
            case 'Safe': return '✓';
            case 'Dangerous': return '⚠️';
            case 'Scanning': return '🔍';
            case 'Downloading': return '⬇️';
            case 'Failed': return '✕';
            default: return '⏳';
        }
    };

    if (!isOpen) return null;

    return (
        <div style={{
            position: 'fixed',
            top: 0,
            right: 0,
            width: '450px',
            height: '100vh',
            background: 'var(--bg-primary)',
            borderLeft: '1px solid var(--border-default)',
            boxShadow: '-4px 0 24px rgba(0,0,0,0.1)',
            zIndex: 1000,
            display: 'flex',
            flexDirection: 'column',
        }}>
            {/* Header */}
            <div style={{
                padding: '16px 20px',
                borderBottom: '1px solid var(--border-default)',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                background: 'var(--bg-secondary)',
            }}>
                <h2 style={{ margin: 0, fontSize: '18px', fontWeight: 600, color: 'var(--text-primary)' }}>
                    📥 Downloads
                </h2>
                <button
                    onClick={onClose}
                    style={{
                        background: 'var(--bg-hover)',
                        border: 'none',
                        borderRadius: '6px',
                        padding: '6px 12px',
                        cursor: 'pointer',
                        color: 'var(--text-primary)',
                        fontSize: '14px',
                    }}
                >
                    ✕ Close
                </button>
            </div>

            {/* Security Status */}
            <div style={{
                padding: '16px 20px',
                background: 'linear-gradient(135deg, #10b981 0%, #059669 100%)',
                color: 'white',
                margin: '16px',
                borderRadius: '12px',
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '10px', marginBottom: '12px' }}>
                    <span style={{ fontSize: '24px' }}>🛡️</span>
                    <div>
                        <div style={{ fontWeight: 600, fontSize: '15px' }}>Antivirus Scanning Active</div>
                        <div style={{ fontSize: '12px', opacity: 0.9 }}>All downloads are automatically scanned</div>
                    </div>
                </div>
                <div style={{ display: 'flex', gap: '16px', fontSize: '13px' }}>
                    <div>
                        <span style={{ fontWeight: 600 }}>{stats.total_scanned}</span> Scanned
                    </div>
                    <div>
                        <span style={{ fontWeight: 600 }}>{stats.threats_blocked}</span> Threats Blocked
                    </div>
                </div>
            </div>

            {/* Scanning Toggle */}
            <div style={{
                margin: '0 16px 16px',
                padding: '14px 16px',
                background: 'var(--bg-tertiary)',
                borderRadius: '10px',
                border: '1px solid var(--border-default)',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
            }}>
                <div>
                    <div style={{ fontWeight: 500, fontSize: '14px', color: 'var(--text-primary)' }}>
                        🔍 Scan Downloads
                    </div>
                    <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginTop: '2px' }}>
                        SHA256 hash verification
                    </div>
                </div>
                <label style={{ position: 'relative', display: 'inline-block', width: '44px', height: '24px' }}>
                    <input
                        type="checkbox"
                        checked={scanEnabled}
                        onChange={() => setScanEnabled(!scanEnabled)}
                        style={{ opacity: 0, width: 0, height: 0 }}
                    />
                    <span style={{
                        position: 'absolute',
                        cursor: 'pointer',
                        top: 0, left: 0, right: 0, bottom: 0,
                        background: scanEnabled ? '#10b981' : '#cbd5e1',
                        borderRadius: '24px',
                        transition: '0.3s',
                    }}>
                        <span style={{
                            position: 'absolute',
                            height: '18px',
                            width: '18px',
                            left: scanEnabled ? '22px' : '3px',
                            bottom: '3px',
                            background: 'white',
                            borderRadius: '50%',
                            transition: '0.3s',
                            boxShadow: '0 2px 4px rgba(0,0,0,0.2)',
                        }}></span>
                    </span>
                </label>
            </div>

            {/* How It Works */}
            <div style={{
                margin: '0 16px 16px',
                padding: '14px 16px',
                background: 'var(--bg-tertiary)',
                borderRadius: '10px',
                border: '1px solid var(--border-default)',
            }}>
                <div style={{ fontWeight: 600, fontSize: '13px', color: 'var(--text-primary)', marginBottom: '10px' }}>
                    🔒 How Scanning Works
                </div>
                <div style={{ fontSize: '12px', color: 'var(--text-muted)', lineHeight: 1.6 }}>
                    <div style={{ marginBottom: '6px' }}>1. <b>Pre-scan</b>: URL and filename checked against threat databases</div>
                    <div style={{ marginBottom: '6px' }}>2. <b>Download</b>: File downloaded with progress tracking</div>
                    <div style={{ marginBottom: '6px' }}>3. <b>Hash Check</b>: SHA256 hash compared to known malware</div>
                    <div>4. <b>Result</b>: ✓ Safe or ⚠️ Blocked if threat detected</div>
                </div>
            </div>

            {/* Downloads List */}
            <div style={{ flex: 1, overflow: 'auto', padding: '0 16px 16px' }}>
                <div style={{ fontWeight: 600, fontSize: '13px', color: 'var(--text-muted)', marginBottom: '12px' }}>
                    RECENT DOWNLOADS
                </div>

                {downloads.length === 0 ? (
                    <div style={{
                        textAlign: 'center',
                        color: 'var(--text-muted)',
                        padding: '40px 20px',
                        background: 'var(--bg-tertiary)',
                        borderRadius: '10px',
                    }}>
                        <div style={{ fontSize: '32px', marginBottom: '12px' }}>📂</div>
                        <div>No downloads yet</div>
                        <div style={{ fontSize: '12px', marginTop: '4px' }}>Files will appear here when downloaded</div>
                    </div>
                ) : (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                        {downloads.map((download) => (
                            <div key={download.id} style={{
                                padding: '14px',
                                background: 'var(--bg-tertiary)',
                                borderRadius: '10px',
                                border: '1px solid var(--border-default)',
                            }}>
                                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
                                    <div style={{ flex: 1 }}>
                                        <div style={{
                                            fontWeight: 500,
                                            fontSize: '14px',
                                            color: 'var(--text-primary)',
                                            wordBreak: 'break-all',
                                        }}>
                                            {download.filename}
                                        </div>
                                        <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginTop: '4px' }}>
                                            {formatSize(download.size)}
                                        </div>
                                    </div>
                                    <div style={{
                                        display: 'flex',
                                        alignItems: 'center',
                                        gap: '6px',
                                        padding: '4px 10px',
                                        borderRadius: '20px',
                                        background: `${getStatusColor(download.status)}20`,
                                        color: getStatusColor(download.status),
                                        fontSize: '12px',
                                        fontWeight: 600,
                                    }}>
                                        {getStatusIcon(download.status)} {download.status}
                                    </div>
                                </div>

                                {download.scan_result && 'Clean' in download.scan_result && (
                                    <div style={{
                                        marginTop: '10px',
                                        padding: '8px 10px',
                                        background: '#dcfce7',
                                        borderRadius: '6px',
                                        fontSize: '12px',
                                        color: '#166534',
                                    }}>
                                        ✓ Scanned - No threats detected
                                    </div>
                                )}

                                {download.file_hash && (
                                    <div style={{
                                        marginTop: '8px',
                                        fontSize: '11px',
                                        color: 'var(--text-muted)',
                                        fontFamily: 'var(--font-mono)',
                                    }}>
                                        SHA256: {download.file_hash}
                                    </div>
                                )}
                            </div>
                        ))}
                    </div>
                )}
            </div>
        </div>
    );
};

export default DownloadManager;
