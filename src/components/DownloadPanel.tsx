import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Download {
    id: string;
    url: string;
    filename: string;
    save_path: string;
    size_total: number | null;
    size_downloaded: number;
    state: string;
    speed_bps: number;
    error: string | null;
}

interface DetectedMedia {
    id: string;
    title: string | null;
    url: string;
    page_url: string;
    media_type: string;
    duration: number | null;
}

interface DownloadPanelProps {
    isOpen: boolean;
    onClose: () => void;
}

export function DownloadPanel({ isOpen, onClose }: DownloadPanelProps) {
    const [downloads, setDownloads] = useState<Download[]>([]);
    const [detectedVideos, setDetectedVideos] = useState<DetectedMedia[]>([]);
    const [downloadDir, setDownloadDir] = useState('');
    const [activeTab, setActiveTab] = useState<'downloads' | 'videos'>('downloads');
    const [urlInput, setUrlInput] = useState('');

    useEffect(() => {
        if (isOpen) {
            loadDownloads();
            loadDetectedVideos();
            invoke<string>('get_download_directory').then(setDownloadDir);

            // Poll for updates every 2 seconds
            const interval = setInterval(() => {
                loadDownloads();
                loadDetectedVideos();
            }, 2000);

            return () => clearInterval(interval);
        }
    }, [isOpen]);

    const loadDownloads = async () => {
        try {
            const list = await invoke<Download[]>('get_downloads');
            setDownloads(list);
        } catch (error) {
            console.error('Failed to load downloads:', error);
        }
    };

    const loadDetectedVideos = async () => {
        try {
            const videos = await invoke<DetectedMedia[]>('get_all_detected_videos');
            setDetectedVideos(videos);
        } catch (error) {
            console.error('Failed to load detected videos:', error);
        }
    };

    const startDownload = async (url: string, filename?: string) => {
        try {
            const download = await invoke<Download>('start_download', { url, filename });
            // Execute the download
            invoke('execute_download', { id: download.id }).catch(console.error);
            loadDownloads();
        } catch (error) {
            console.error('Failed to start download:', error);
        }
    };

    const handleUrlDownload = () => {
        if (urlInput.trim()) {
            startDownload(urlInput.trim());
            setUrlInput('');
        }
    };

    const pauseDownload = (id: string) => invoke('pause_download', { id }).then(loadDownloads);
    const resumeDownload = (id: string) => invoke('resume_download', { id }).then(loadDownloads);
    const cancelDownload = (id: string) => invoke('cancel_download', { id }).then(loadDownloads);
    const clearCompleted = () => invoke('clear_completed_downloads').then(loadDownloads);

    const downloadVideo = async (video: DetectedMedia) => {
        const filename = video.title || 'video';
        try {
            const download = await invoke<Download>('download_video', { url: video.url, filename });
            invoke('execute_download', { id: download.id }).catch(console.error);
            setActiveTab('downloads');
            loadDownloads();
        } catch (error) {
            console.error('Failed to start video download:', error);
        }
    };

    const formatBytes = (bytes: number): string => {
        if (bytes >= 1073741824) return `${(bytes / 1073741824).toFixed(2)} GB`;
        if (bytes >= 1048576) return `${(bytes / 1048576).toFixed(2)} MB`;
        if (bytes >= 1024) return `${(bytes / 1024).toFixed(2)} KB`;
        return `${bytes} B`;
    };

    const getStateColor = (state: string): string => {
        switch (state) {
            case 'Completed': return '#22c55e';
            case 'Downloading': return '#3b82f6';
            case 'Failed': return '#ef4444';
            case 'Paused': return '#f59e0b';
            default: return '#6b7280';
        }
    };

    const getProgress = (d: Download): number => {
        if (!d.size_total || d.size_total === 0) return 0;
        return (d.size_downloaded / d.size_total) * 100;
    };

    if (!isOpen) return null;

    return (
        <div style={{
            position: 'fixed', top: 0, left: 0, right: 0, bottom: 0,
            background: 'rgba(0,0,0,0.8)', zIndex: 9999,
            display: 'flex', alignItems: 'center', justifyContent: 'center',
        }}>
            <div style={{
                width: '700px', maxHeight: '80vh', background: 'var(--bg-secondary)',
                borderRadius: '16px', overflow: 'hidden', display: 'flex', flexDirection: 'column',
            }}>
                {/* Header */}
                <div style={{
                    padding: '20px 24px', borderBottom: '1px solid var(--border-default)',
                    display: 'flex', justifyContent: 'space-between', alignItems: 'center',
                }}>
                    <div>
                        <h2 style={{ margin: 0, fontSize: '20px', color: 'var(--text-primary)' }}>
                            📥 Downloads
                        </h2>
                        <p style={{ margin: '4px 0 0 0', fontSize: '12px', color: 'var(--text-muted)' }}>
                            {downloadDir}
                        </p>
                    </div>
                    <button onClick={onClose} style={{
                        background: 'transparent', border: 'none', fontSize: '24px',
                        cursor: 'pointer', color: 'var(--text-secondary)',
                    }}>×</button>
                </div>

                {/* Tabs */}
                <div style={{ display: 'flex', borderBottom: '1px solid var(--border-default)' }}>
                    <button
                        onClick={() => setActiveTab('downloads')}
                        style={{
                            flex: 1, padding: '12px', border: 'none', cursor: 'pointer',
                            background: activeTab === 'downloads' ? 'var(--bg-tertiary)' : 'transparent',
                            color: activeTab === 'downloads' ? 'var(--accent-primary)' : 'var(--text-secondary)',
                            fontWeight: 600,
                        }}
                    >
                        📥 Downloads ({downloads.length})
                    </button>
                    <button
                        onClick={() => setActiveTab('videos')}
                        style={{
                            flex: 1, padding: '12px', border: 'none', cursor: 'pointer',
                            background: activeTab === 'videos' ? 'var(--bg-tertiary)' : 'transparent',
                            color: activeTab === 'videos' ? 'var(--accent-primary)' : 'var(--text-secondary)',
                            fontWeight: 600,
                        }}
                    >
                        🎬 Detected Videos ({detectedVideos.length})
                    </button>
                </div>

                {/* Content */}
                <div style={{ flex: 1, overflow: 'auto', padding: '16px' }}>
                    {activeTab === 'downloads' && (
                        <>
                            {/* URL Input */}
                            <div style={{ display: 'flex', gap: '8px', marginBottom: '16px' }}>
                                <input
                                    type="text"
                                    value={urlInput}
                                    onChange={(e) => setUrlInput(e.target.value)}
                                    placeholder="Paste URL to download..."
                                    onKeyPress={(e) => e.key === 'Enter' && handleUrlDownload()}
                                    style={{
                                        flex: 1, padding: '10px 12px', borderRadius: '8px',
                                        border: '1px solid var(--border-default)',
                                        background: 'var(--bg-tertiary)', color: 'var(--text-primary)',
                                    }}
                                />
                                <button
                                    onClick={handleUrlDownload}
                                    style={{
                                        padding: '10px 20px', borderRadius: '8px', border: 'none',
                                        background: 'linear-gradient(135deg, #3b82f6, #2563eb)',
                                        color: 'white', fontWeight: 600, cursor: 'pointer',
                                    }}
                                >
                                    Download
                                </button>
                            </div>

                            {downloads.length === 0 ? (
                                <div style={{ textAlign: 'center', padding: '40px', color: 'var(--text-muted)' }}>
                                    <div style={{ fontSize: '48px', marginBottom: '16px' }}>📥</div>
                                    <p>No downloads yet</p>
                                    <p style={{ fontSize: '12px' }}>Paste a URL above or click download on detected videos</p>
                                </div>
                            ) : (
                                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                    {downloads.map((d) => (
                                        <div key={d.id} style={{
                                            padding: '12px', borderRadius: '10px',
                                            background: 'var(--bg-tertiary)', border: '1px solid var(--border-subtle)',
                                        }}>
                                            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                                                <span style={{ fontWeight: 600, color: 'var(--text-primary)', fontSize: '14px' }}>
                                                    {d.filename}
                                                </span>
                                                <span style={{
                                                    color: getStateColor(d.state), fontSize: '12px', fontWeight: 600
                                                }}>
                                                    {d.state}
                                                </span>
                                            </div>

                                            {/* Progress bar */}
                                            <div style={{
                                                height: '6px', borderRadius: '3px',
                                                background: 'var(--bg-primary)', marginBottom: '8px',
                                            }}>
                                                <div style={{
                                                    height: '100%', borderRadius: '3px',
                                                    background: getStateColor(d.state),
                                                    width: `${getProgress(d)}%`,
                                                    transition: 'width 0.3s ease',
                                                }} />
                                            </div>

                                            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                                                <span style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
                                                    {formatBytes(d.size_downloaded)}
                                                    {d.size_total && ` / ${formatBytes(d.size_total)}`}
                                                    {d.speed_bps > 0 && ` • ${formatBytes(d.speed_bps)}/s`}
                                                </span>
                                                <div style={{ display: 'flex', gap: '4px' }}>
                                                    {d.state === 'Downloading' && (
                                                        <button onClick={() => pauseDownload(d.id)} style={btnStyle}>⏸️</button>
                                                    )}
                                                    {d.state === 'Paused' && (
                                                        <button onClick={() => resumeDownload(d.id)} style={btnStyle}>▶️</button>
                                                    )}
                                                    {d.state !== 'Completed' && d.state !== 'Failed' && (
                                                        <button onClick={() => cancelDownload(d.id)} style={btnStyle}>❌</button>
                                                    )}
                                                </div>
                                            </div>

                                            {d.error && (
                                                <p style={{ margin: '8px 0 0 0', fontSize: '12px', color: '#ef4444' }}>
                                                    Error: {d.error}
                                                </p>
                                            )}
                                        </div>
                                    ))}

                                    {downloads.some(d => d.state === 'Completed') && (
                                        <button onClick={clearCompleted} style={{
                                            padding: '8px 16px', borderRadius: '6px', border: 'none',
                                            background: 'var(--bg-primary)', color: 'var(--text-secondary)',
                                            cursor: 'pointer', fontSize: '12px',
                                        }}>
                                            Clear Completed
                                        </button>
                                    )}
                                </div>
                            )}
                        </>
                    )}

                    {activeTab === 'videos' && (
                        detectedVideos.length === 0 ? (
                            <div style={{ textAlign: 'center', padding: '40px', color: 'var(--text-muted)' }}>
                                <div style={{ fontSize: '48px', marginBottom: '16px' }}>🎬</div>
                                <p>No videos detected yet</p>
                                <p style={{ fontSize: '12px' }}>Browse websites with videos and they will appear here</p>
                            </div>
                        ) : (
                            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                {detectedVideos.map((v) => (
                                    <div key={v.id} style={{
                                        padding: '12px', borderRadius: '10px',
                                        background: 'var(--bg-tertiary)', border: '1px solid var(--border-subtle)',
                                        display: 'flex', justifyContent: 'space-between', alignItems: 'center',
                                    }}>
                                        <div style={{ flex: 1 }}>
                                            <div style={{ fontWeight: 600, color: 'var(--text-primary)', fontSize: '14px' }}>
                                                {v.title || 'Untitled Video'}
                                            </div>
                                            <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginTop: '4px' }}>
                                                {v.media_type.toUpperCase()} • {v.url.substring(0, 50)}...
                                            </div>
                                        </div>
                                        <button
                                            onClick={() => downloadVideo(v)}
                                            style={{
                                                padding: '8px 16px', borderRadius: '6px', border: 'none',
                                                background: 'linear-gradient(135deg, #22c55e, #16a34a)',
                                                color: 'white', fontWeight: 600, cursor: 'pointer',
                                            }}
                                        >
                                            📥 Download
                                        </button>
                                    </div>
                                ))}
                            </div>
                        )
                    )}
                </div>
            </div>
        </div>
    );
}

const btnStyle: React.CSSProperties = {
    padding: '4px 8px', borderRadius: '4px', border: 'none',
    background: 'var(--bg-primary)', cursor: 'pointer', fontSize: '12px',
};

export default DownloadPanel;
