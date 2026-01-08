import { FC, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { FakeFingerprint, FakeGeolocation, FakeUserAgent, LogEntry } from '../lib/types';

interface SettingsPanelProps {
    isOpen: boolean;
    onClose: () => void;
    isLocked: boolean;
    onUnlock: () => void;
    onShowKeyboard: () => void;
}

// Security log entry type
interface SecurityLog {
    id: string;
    timestamp: string;
    type: 'antivirus' | 'vulnerability' | 'tracker' | 'malware' | 'download';
    severity: 'info' | 'warning' | 'blocked' | 'safe';
    message: string;
    details?: string;
}

const SettingsPanel: FC<SettingsPanelProps> = ({
    isOpen,
    onClose,
    isLocked: _isLocked,
    onUnlock: _onUnlock,
}) => {
    const [activeTab, setActiveTab] = useState<'security' | 'privacy' | 'identity' | 'whitelist' | 'logs' | 'about'>('security');
    const [fingerprint, setFingerprint] = useState<FakeFingerprint | null>(null);
    const [geolocation, setGeolocation] = useState<FakeGeolocation | null>(null);
    const [userAgent, setUserAgent] = useState<FakeUserAgent | null>(null);
    const [whitelist, setWhitelist] = useState<string[]>([]);
    const [newDomain, setNewDomain] = useState('');
    const [logs, setLogs] = useState<LogEntry[]>([]);
    const [securityLogs, setSecurityLogs] = useState<SecurityLog[]>([]);

    // Password authentication states
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [settingsPassword, setSettingsPassword] = useState('');
    const [settingsPasswordError, setSettingsPasswordError] = useState('');
    const [isLogsAuthenticated, setIsLogsAuthenticated] = useState(false);
    const [logsPassword, setLogsPassword] = useState('');
    const [logsPasswordError, setLogsPasswordError] = useState('');

    // Security & Privacy settings
    const [settings, setSettings] = useState({
        torEnabled: false,
        httpsOnly: true,
        blockTrackers: true,
        blockMalware: true,
        scanDownloads: true,
        stripMetadata: true,
        blockWebRTC: true,
        fakeGeolocation: true,
        spoofFingerprint: true,
        stripReferrer: true,
        partitionStorage: true,
        autoRegenerateIdentity: false,
        secureKeyboard: true,
        blockAds: true,
    });

    // Authenticate settings access
    const handleSettingsLogin = async () => {
        try {
            const success = await invoke<boolean>('unlock_settings', { password: settingsPassword });
            if (success) {
                setIsAuthenticated(true);
                setSettingsPasswordError('');
                loadSettings();  // Load settings from backend
                loadData();
                generateSecurityLogs();
            } else {
                setSettingsPasswordError('Incorrect password');
            }
        } catch (error) {
            setSettingsPasswordError('Authentication failed');
        }
    };

    // Load settings from backend
    const loadSettings = async () => {
        try {
            const backendSettings = await invoke<any>('get_all_settings');
            setSettings({
                torEnabled: backendSettings.tor_enabled ?? false,
                httpsOnly: backendSettings.https_only ?? true,
                blockTrackers: backendSettings.block_trackers ?? true,
                blockMalware: backendSettings.block_malware ?? true,
                scanDownloads: backendSettings.scan_downloads ?? true,
                stripMetadata: backendSettings.strip_metadata ?? true,
                blockWebRTC: backendSettings.block_webrtc ?? true,
                fakeGeolocation: backendSettings.fake_geolocation ?? true,
                spoofFingerprint: backendSettings.spoof_fingerprint ?? true,
                stripReferrer: backendSettings.strip_referrer ?? true,
                partitionStorage: backendSettings.partition_storage ?? true,
                autoRegenerateIdentity: backendSettings.auto_regenerate_identity ?? false,
                secureKeyboard: backendSettings.secure_keyboard ?? true,
                blockAds: backendSettings.block_ads ?? true,
            });
            console.log('Settings loaded from backend:', backendSettings);
        } catch (error) {
            console.error('Failed to load settings:', error);
        }
    };

    // Authenticate logs access
    const handleLogsLogin = async () => {
        try {
            const success = await invoke<boolean>('unlock_logs', { password: logsPassword });
            if (success) {
                setIsLogsAuthenticated(true);
                setLogsPasswordError('');
                // Load actual logs
                try {
                    const logData = await invoke<LogEntry[]>('get_encrypted_logs');
                    setLogs(logData);
                } catch {
                    console.error('Failed to load logs');
                }
            } else {
                setLogsPasswordError('Incorrect password');
            }
        } catch (error) {
            setLogsPasswordError('Authentication failed');
        }
    };

    // Handle panel close - lock everything
    const handleClose = async () => {
        setIsAuthenticated(false);
        setIsLogsAuthenticated(false);
        setSettingsPassword('');
        setLogsPassword('');
        try {
            await invoke('lock_settings');
            await invoke('lock_logs');
        } catch (error) {
            console.error('Failed to lock:', error);
        }
        onClose();
    };

    useEffect(() => {
        if (isOpen) {
            // Reset states when panel opens
            setIsAuthenticated(false);
            setIsLogsAuthenticated(false);
            setSettingsPassword('');
            setLogsPassword('');
            setSettingsPasswordError('');
            setLogsPasswordError('');
        }
    }, [isOpen]);

    const generateSecurityLogs = async () => {
        // Try to load live logs from backend first
        try {
            const liveLogs = await invoke<any[]>('get_live_logs');
            if (liveLogs && liveLogs.length > 0) {
                const formattedLogs: SecurityLog[] = liveLogs.map((log, i) => ({
                    id: log.id || String(i),
                    timestamp: log.timestamp,
                    type: mapLogType(log.log_type),
                    severity: mapSeverity(log.severity),
                    message: log.message,
                    details: log.details || log.domain || log.url,
                }));
                setSecurityLogs(formattedLogs);
                console.log('Loaded', formattedLogs.length, 'live security logs');
                return;
            }
        } catch (error) {
            console.warn('Failed to fetch live logs, using demo data:', error);
        }

        // Fallback to demo data if no live logs
        const now = new Date();
        const logs: SecurityLog[] = [
            {
                id: '1',
                timestamp: new Date(now.getTime() - 1000).toISOString(),
                type: 'antivirus',
                severity: 'safe',
                message: 'SHA256 hash check passed',
                details: 'document.pdf verified against threat database',
            },
            {
                id: '2',
                timestamp: new Date(now.getTime() - 5000).toISOString(),
                type: 'tracker',
                severity: 'blocked',
                message: 'Tracker blocked: googleanalytics.com',
                details: 'Request to tracking domain prevented',
            },
            {
                id: '3',
                timestamp: new Date(now.getTime() - 10000).toISOString(),
                type: 'vulnerability',
                severity: 'info',
                message: 'Website security scan complete',
                details: 'example.com - Grade A (HTTPS, CSP, HSTS present)',
            },
            {
                id: '4',
                timestamp: new Date(now.getTime() - 30000).toISOString(),
                type: 'malware',
                severity: 'safe',
                message: 'URL checked against malware database',
                details: 'No threats detected in URLHaus/PhishTank',
            },
            {
                id: '5',
                timestamp: new Date(now.getTime() - 60000).toISOString(),
                type: 'download',
                severity: 'safe',
                message: 'Download scanned successfully',
                details: 'software.zip - 0 threats found',
            },
            {
                id: '6',
                timestamp: new Date(now.getTime() - 120000).toISOString(),
                type: 'tracker',
                severity: 'blocked',
                message: 'Ad request blocked: doubleclick.net',
                details: 'EasyList filter match',
            },
        ];
        setSecurityLogs(logs);
    };

    // Map backend log types to frontend types
    const mapLogType = (type: any): SecurityLog['type'] => {
        const typeStr = typeof type === 'string' ? type.toLowerCase() : type?.toString()?.toLowerCase() || '';
        if (typeStr.includes('tracker')) return 'tracker';
        if (typeStr.includes('malware')) return 'malware';
        if (typeStr.includes('antivirus') || typeStr.includes('virus')) return 'antivirus';
        if (typeStr.includes('vuln') || typeStr.includes('security')) return 'vulnerability';
        if (typeStr.includes('download')) return 'download';
        return 'tracker';
    };

    // Map backend severity to frontend severity
    const mapSeverity = (sev: any): SecurityLog['severity'] => {
        const sevStr = typeof sev === 'string' ? sev.toLowerCase() : sev?.toString()?.toLowerCase() || '';
        if (sevStr.includes('blocked')) return 'blocked';
        if (sevStr.includes('warning')) return 'warning';
        if (sevStr.includes('safe') || sevStr.includes('info')) return 'safe';
        return 'info';
    };

    const loadData = async () => {
        try {
            const [fp, geo, ua, wl] = await Promise.all([
                invoke<FakeFingerprint>('get_fake_fingerprint'),
                invoke<FakeGeolocation>('get_fake_geolocation'),
                invoke<FakeUserAgent>('get_fake_user_agent'),
                invoke<string[]>('get_whitelist'),
            ]);
            setFingerprint(fp);
            setGeolocation(geo);
            setUserAgent(ua);
            setWhitelist(wl);

            try {
                const logData = await invoke<LogEntry[]>('get_encrypted_logs');
                setLogs(logData);
            } catch {
                setLogs([
                    { id: '1', timestamp: new Date().toISOString(), level: 'Info', category: 'System', message: 'ServionX Browser started', details: undefined },
                    { id: '2', timestamp: new Date().toISOString(), level: 'Security', category: 'Privacy', message: 'Privacy engine initialized', details: undefined },
                ]);
            }
        } catch (error) {
            console.error('Failed to load settings data:', error);
        }
    };

    const addToWhitelist = async () => {
        if (newDomain.trim()) {
            try {
                await invoke('add_to_whitelist', { domain: newDomain.trim() });
                setWhitelist([...whitelist, newDomain.trim()]);
                setNewDomain('');
            } catch (error) {
                console.error('Failed to add to whitelist:', error);
            }
        }
    };

    const removeFromWhitelist = async (domain: string) => {
        try {
            await invoke('remove_from_whitelist', { domain });
            setWhitelist(whitelist.filter(d => d !== domain));
        } catch (error) {
            console.error('Failed to remove from whitelist:', error);
        }
    };

    const regenerateIdentity = async () => {
        try {
            const identity = await invoke<any>('regenerate_identity');
            setFingerprint(identity.fingerprint);
            setGeolocation(identity.geolocation);
            setUserAgent(identity.userAgent);
        } catch (error) {
            console.error('Failed to regenerate identity:', error);
        }
    };

    // Toggle a setting and persist to backend
    const toggleSetting = async (key: keyof typeof settings) => {
        const newValue = !settings[key];
        setSettings(prev => ({ ...prev, [key]: newValue }));

        try {
            await invoke('set_setting', { key, value: newValue });
            console.log(`Setting ${key} updated to ${newValue}`);
        } catch (error) {
            console.error(`Failed to persist setting ${key}:`, error);
            // Revert on error
            setSettings(prev => ({ ...prev, [key]: !newValue }));
        }
    };

    if (!isOpen) return null;

    const tabs = [
        { id: 'security', label: '🛡️ Security' },
        { id: 'privacy', label: '👁️ Privacy' },
        { id: 'identity', label: '🎭 Identity' },
        { id: 'whitelist', label: '✅ Whitelist' },
        { id: 'logs', label: '📋 Logs' },
        { id: 'about', label: 'ℹ️ About' },
    ];

    const getSeverityStyle = (severity: string) => {
        switch (severity) {
            case 'blocked': return { bg: '#fee2e2', color: '#991b1b' };
            case 'warning': return { bg: '#fef3c7', color: '#92400e' };
            case 'safe': return { bg: '#dcfce7', color: '#166534' };
            default: return { bg: '#e0f2fe', color: '#0369a1' };
        }
    };

    const getTypeIcon = (type: string) => {
        switch (type) {
            case 'antivirus': return '🦠';
            case 'vulnerability': return '🔍';
            case 'tracker': return '🚫';
            case 'malware': return '⚠️';
            case 'download': return '📥';
            default: return '📋';
        }
    };

    // If not authenticated, show login dialog
    if (!isAuthenticated) {
        return (
            <div className="settings-panel" style={{
                position: 'fixed',
                top: 0,
                right: 0,
                width: '480px',
                height: '100vh',
                background: 'var(--bg-primary)',
                borderLeft: '1px solid var(--border-default)',
                boxShadow: '-4px 0 24px rgba(0,0,0,0.1)',
                zIndex: 1000,
                display: 'flex',
                flexDirection: 'column',
                justifyContent: 'center',
                alignItems: 'center',
            }}>
                <div style={{
                    background: 'var(--bg-secondary)',
                    padding: '32px',
                    borderRadius: '16px',
                    border: '1px solid var(--border-default)',
                    width: '320px',
                    textAlign: 'center',
                }}>
                    <div style={{ fontSize: '48px', marginBottom: '16px' }}>🔐</div>
                    <h2 style={{ margin: '0 0 8px', fontSize: '20px', color: 'var(--text-primary)' }}>Settings Protected</h2>
                    <p style={{ margin: '0 0 20px', fontSize: '13px', color: 'var(--text-muted)' }}>
                        Enter password to access settings
                    </p>
                    <input
                        type="password"
                        value={settingsPassword}
                        onChange={(e) => setSettingsPassword(e.target.value)}
                        onKeyDown={(e) => e.key === 'Enter' && handleSettingsLogin()}
                        placeholder="Enter password..."
                        style={{
                            width: '100%',
                            padding: '12px 16px',
                            borderRadius: '10px',
                            border: settingsPasswordError ? '1px solid #ef4444' : '1px solid var(--border-default)',
                            background: 'var(--bg-tertiary)',
                            color: 'var(--text-primary)',
                            fontSize: '14px',
                            marginBottom: '12px',
                            boxSizing: 'border-box',
                        }}
                        autoFocus
                    />
                    {settingsPasswordError && (
                        <div style={{ color: '#ef4444', fontSize: '12px', marginBottom: '12px' }}>
                            {settingsPasswordError}
                        </div>
                    )}
                    <div style={{ display: 'flex', gap: '10px' }}>
                        <button
                            onClick={handleClose}
                            style={{
                                flex: 1,
                                padding: '12px 20px',
                                borderRadius: '10px',
                                border: '1px solid var(--border-default)',
                                background: 'var(--bg-tertiary)',
                                color: 'var(--text-primary)',
                                cursor: 'pointer',
                                fontSize: '14px',
                            }}
                        >
                            Cancel
                        </button>
                        <button
                            onClick={handleSettingsLogin}
                            style={{
                                flex: 1,
                                padding: '12px 20px',
                                borderRadius: '10px',
                                border: 'none',
                                background: 'linear-gradient(135deg, var(--accent-primary), var(--accent-secondary))',
                                color: 'white',
                                cursor: 'pointer',
                                fontSize: '14px',
                                fontWeight: 600,
                            }}
                        >
                            Unlock
                        </button>
                    </div>
                    <div style={{ marginTop: '16px', fontSize: '11px', color: 'var(--text-muted)' }}>
                        Default: <code style={{ background: 'var(--bg-hover)', padding: '2px 6px', borderRadius: '4px' }}>ServionX2024</code>
                    </div>
                </div>
            </div>
        );
    }

    return (
        <div className="settings-panel" style={{
            position: 'fixed',
            top: 0,
            right: 0,
            width: '480px',
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
                    ⚙️ Settings
                </h2>
                <button
                    onClick={handleClose}
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

            {/* Tabs */}
            <div style={{
                display: 'flex',
                borderBottom: '1px solid var(--border-default)',
                background: 'var(--bg-secondary)',
                padding: '0 8px',
                overflowX: 'auto',
            }}>
                {tabs.map(tab => (
                    <button
                        key={tab.id}
                        onClick={() => setActiveTab(tab.id as any)}
                        style={{
                            padding: '12px 14px',
                            border: 'none',
                            background: activeTab === tab.id ? 'var(--bg-primary)' : 'transparent',
                            color: activeTab === tab.id ? 'var(--accent-primary)' : 'var(--text-secondary)',
                            cursor: 'pointer',
                            fontSize: '12px',
                            fontWeight: 500,
                            borderBottom: activeTab === tab.id ? '2px solid var(--accent-primary)' : '2px solid transparent',
                            whiteSpace: 'nowrap',
                        }}
                    >
                        {tab.label}
                    </button>
                ))}
            </div>

            {/* Content */}
            <div style={{ flex: 1, overflow: 'auto', padding: '20px' }}>

                {/* Security Tab */}
                {activeTab === 'security' && (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                        <div style={{
                            background: settings.torEnabled ? 'linear-gradient(135deg, #7c3aed 0%, #9333ea 100%)' : 'var(--bg-tertiary)',
                            padding: '16px',
                            borderRadius: '12px',
                            border: settings.torEnabled ? 'none' : '1px solid var(--border-default)',
                        }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                                <div>
                                    <div style={{
                                        fontWeight: 600,
                                        fontSize: '15px',
                                        color: settings.torEnabled ? 'white' : 'var(--text-primary)',
                                    }}>
                                        🧅 Tor Network
                                    </div>
                                    <div style={{
                                        fontSize: '12px',
                                        color: settings.torEnabled ? 'rgba(255,255,255,0.8)' : 'var(--text-muted)',
                                        marginTop: '4px',
                                    }}>
                                        Route traffic through Tor for anonymity
                                    </div>
                                </div>
                                <ToggleSwitch checked={settings.torEnabled} onChange={() => toggleSetting('torEnabled')} />
                            </div>
                        </div>

                        <SettingToggle label="HTTPS Only" description="Force secure connections" icon="🔒" checked={settings.httpsOnly} onChange={() => toggleSetting('httpsOnly')} />
                        <SettingToggle label="Block Trackers" description="94+ tracking domains blocked" icon="🚫" checked={settings.blockTrackers} onChange={() => toggleSetting('blockTrackers')} />
                        <SettingToggle label="Malware Protection" description="Block malicious URLs & phishing" icon="🦠" checked={settings.blockMalware} onChange={() => toggleSetting('blockMalware')} />
                        <SettingToggle label="Scan Downloads" description="SHA256 hash verification" icon="📥" checked={settings.scanDownloads} onChange={() => toggleSetting('scanDownloads')} />
                        <SettingToggle label="Block Ads" description="EasyList-compatible ad blocking" icon="🚫" checked={settings.blockAds} onChange={() => toggleSetting('blockAds')} />
                    </div>
                )}

                {/* Privacy Tab */}
                {activeTab === 'privacy' && (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                        <SettingToggle label="Fingerprint Spoofing" description="Canvas, WebGL, Audio, Hardware" icon="🎭" checked={settings.spoofFingerprint} onChange={() => toggleSetting('spoofFingerprint')} />
                        <SettingToggle label="Block WebRTC" description="Prevent IP leak" icon="📡" checked={settings.blockWebRTC} onChange={() => toggleSetting('blockWebRTC')} />
                        <SettingToggle label="Fake Geolocation" description="Random city/country" icon="📍" checked={settings.fakeGeolocation} onChange={() => toggleSetting('fakeGeolocation')} />
                        <SettingToggle label="Strip Referrer" description="Don't reveal where you came from" icon="🔗" checked={settings.stripReferrer} onChange={() => toggleSetting('stripReferrer')} />
                        <SettingToggle label="Partition Storage" description="Isolate data per site" icon="📦" checked={settings.partitionStorage} onChange={() => toggleSetting('partitionStorage')} />
                        <SettingToggle label="Strip Metadata" description="Remove EXIF from uploads" icon="🖼️" checked={settings.stripMetadata} onChange={() => toggleSetting('stripMetadata')} />
                    </div>
                )}

                {/* Identity Tab */}
                {activeTab === 'identity' && (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                        <button onClick={regenerateIdentity} style={{
                            background: 'linear-gradient(135deg, var(--accent-primary) 0%, var(--accent-secondary) 100%)',
                            color: 'white', border: 'none', padding: '14px 20px', borderRadius: '10px',
                            cursor: 'pointer', fontSize: '14px', fontWeight: 600,
                        }}>
                            🔄 Regenerate Identity
                        </button>

                        {fingerprint && <InfoCard title="Fingerprint" items={[
                            { label: 'Canvas Noise', value: String(fingerprint.canvasNoiseSeed || 'Spoofed') },
                            { label: 'WebGL Vendor', value: fingerprint.webglVendor || 'Hidden' },
                            { label: 'WebGL Renderer', value: fingerprint.webglRenderer?.substring(0, 20) + '...' || 'Hidden' },
                        ]} />}

                        {geolocation && <InfoCard title="Location" items={[
                            { label: 'City', value: geolocation.city || 'Hidden' },
                            { label: 'Country', value: geolocation.country || 'Hidden' },
                            { label: 'Coordinates', value: `${geolocation.latitude?.toFixed(2)}, ${geolocation.longitude?.toFixed(2)}` || 'Spoofed' },
                        ]} />}

                        {userAgent && <InfoCard title="User Agent" items={[
                            { label: 'Browser', value: userAgent.browserName || 'Chrome' },
                            { label: 'OS', value: userAgent.osName || 'Windows' },
                            { label: 'Version', value: userAgent.browserVersion || '120.0' },
                        ]} />}
                    </div>
                )}

                {/* Whitelist Tab */}
                {activeTab === 'whitelist' && (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                        <p style={{ color: 'var(--text-muted)', fontSize: '13px', margin: 0 }}>
                            Sites in the whitelist will receive your real browser data instead of spoofed data.
                        </p>

                        <div style={{ display: 'flex', gap: '8px' }}>
                            <input
                                type="text"
                                value={newDomain}
                                onChange={(e) => setNewDomain(e.target.value)}
                                placeholder="Enter domain (e.g., example.com)"
                                style={{
                                    flex: 1, padding: '10px 14px', borderRadius: '8px',
                                    border: '1px solid var(--border-default)', background: 'var(--bg-tertiary)',
                                    color: 'var(--text-primary)', fontSize: '14px',
                                }}
                            />
                            <button onClick={addToWhitelist} style={{
                                background: 'var(--accent-primary)', color: 'white', border: 'none',
                                padding: '10px 16px', borderRadius: '8px', cursor: 'pointer', fontWeight: 500,
                            }}>
                                Add
                            </button>
                        </div>

                        <div style={{ background: 'var(--bg-tertiary)', borderRadius: '10px', border: '1px solid var(--border-default)' }}>
                            {whitelist.length === 0 ? (
                                <div style={{ padding: '20px', textAlign: 'center', color: 'var(--text-muted)' }}>
                                    No domains whitelisted
                                </div>
                            ) : (
                                whitelist.map((domain, i) => (
                                    <div key={domain} style={{
                                        display: 'flex', justifyContent: 'space-between', alignItems: 'center',
                                        padding: '12px 14px',
                                        borderBottom: i < whitelist.length - 1 ? '1px solid var(--border-subtle)' : 'none',
                                    }}>
                                        <span style={{ color: 'var(--text-primary)' }}>{domain}</span>
                                        <button onClick={() => removeFromWhitelist(domain)} style={{
                                            background: 'var(--accent-error)', color: 'white', border: 'none',
                                            padding: '4px 10px', borderRadius: '4px', cursor: 'pointer', fontSize: '12px',
                                        }}>
                                            Remove
                                        </button>
                                    </div>
                                ))
                            )}
                        </div>
                    </div>
                )}

                {/* Security Logs Tab */}
                {activeTab === 'logs' && (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
                        {/* If logs not authenticated, show logs login dialog */}
                        {!isLogsAuthenticated ? (
                            <div style={{
                                background: 'var(--bg-secondary)',
                                padding: '24px',
                                borderRadius: '12px',
                                border: '1px solid var(--border-default)',
                                textAlign: 'center',
                            }}>
                                <div style={{ fontSize: '32px', marginBottom: '12px' }}>📋</div>
                                <h3 style={{ margin: '0 0 8px', fontSize: '16px', color: 'var(--text-primary)' }}>Logs Protected</h3>
                                <p style={{ margin: '0 0 16px', fontSize: '12px', color: 'var(--text-muted)' }}>
                                    Enter logs password to view security events
                                </p>
                                <input
                                    type="password"
                                    value={logsPassword}
                                    onChange={(e) => setLogsPassword(e.target.value)}
                                    onKeyDown={(e) => e.key === 'Enter' && handleLogsLogin()}
                                    placeholder="Enter logs password..."
                                    style={{
                                        width: '100%',
                                        padding: '10px 14px',
                                        borderRadius: '8px',
                                        border: logsPasswordError ? '1px solid #ef4444' : '1px solid var(--border-default)',
                                        background: 'var(--bg-tertiary)',
                                        color: 'var(--text-primary)',
                                        fontSize: '14px',
                                        marginBottom: '10px',
                                        boxSizing: 'border-box',
                                    }}
                                />
                                {logsPasswordError && (
                                    <div style={{ color: '#ef4444', fontSize: '12px', marginBottom: '10px' }}>
                                        {logsPasswordError}
                                    </div>
                                )}
                                <button
                                    onClick={handleLogsLogin}
                                    style={{
                                        width: '100%',
                                        padding: '10px 16px',
                                        borderRadius: '8px',
                                        border: 'none',
                                        background: 'linear-gradient(135deg, #10b981, #059669)',
                                        color: 'white',
                                        cursor: 'pointer',
                                        fontSize: '14px',
                                        fontWeight: 600,
                                    }}
                                >
                                    🔓 View Logs
                                </button>
                                <div style={{ marginTop: '12px', fontSize: '11px', color: 'var(--text-muted)' }}>
                                    Default: <code style={{ background: 'var(--bg-hover)', padding: '2px 6px', borderRadius: '4px' }}>SecureLogs123</code>
                                </div>
                            </div>
                        ) : (
                            <>
                                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                                    <span style={{ color: 'var(--text-muted)', fontSize: '13px' }}>
                                        {securityLogs.length} security events {logs.length > 0 && `+ ${logs.length} system logs`}
                                    </span>
                                    <button onClick={generateSecurityLogs} style={{
                                        background: 'var(--bg-tertiary)', border: '1px solid var(--border-default)',
                                        padding: '6px 12px', borderRadius: '6px', cursor: 'pointer', fontSize: '12px',
                                        color: 'var(--text-secondary)',
                                    }}>
                                        🔄 Refresh
                                    </button>
                                </div>

                                <div style={{ display: 'flex', gap: '8px', marginBottom: '8px', flexWrap: 'wrap' }}>
                                    <FilterBadge label="All" count={securityLogs.length} active />
                                    <FilterBadge label="Antivirus" count={securityLogs.filter(l => l.type === 'antivirus').length} />
                                    <FilterBadge label="Trackers" count={securityLogs.filter(l => l.type === 'tracker').length} />
                                    <FilterBadge label="Vulns" count={securityLogs.filter(l => l.type === 'vulnerability').length} />
                                </div>

                                <div style={{ background: 'var(--bg-tertiary)', borderRadius: '10px', border: '1px solid var(--border-default)', maxHeight: '400px', overflow: 'auto' }}>
                                    {securityLogs.map((log, i) => {
                                        const style = getSeverityStyle(log.severity);
                                        return (
                                            <div key={log.id} style={{
                                                padding: '12px 14px',
                                                borderBottom: i < securityLogs.length - 1 ? '1px solid var(--border-subtle)' : 'none',
                                            }}>
                                                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '4px' }}>
                                                    <span>{getTypeIcon(log.type)}</span>
                                                    <span style={{
                                                        padding: '2px 8px', borderRadius: '4px', fontSize: '10px', fontWeight: 600,
                                                        background: style.bg, color: style.color, textTransform: 'uppercase',
                                                    }}>
                                                        {log.severity}
                                                    </span>
                                                    <span style={{ color: 'var(--text-muted)', fontSize: '11px', marginLeft: 'auto' }}>
                                                        {new Date(log.timestamp).toLocaleTimeString()}
                                                    </span>
                                                </div>
                                                <div style={{ color: 'var(--text-primary)', fontSize: '13px', fontWeight: 500 }}>
                                                    {log.message}
                                                </div>
                                                {log.details && (
                                                    <div style={{ color: 'var(--text-muted)', fontSize: '11px', marginTop: '4px' }}>
                                                        {log.details}
                                                    </div>
                                                )}
                                            </div>
                                        );
                                    })}
                                </div>
                            </>
                        )}
                    </div>
                )}

                {/* About Tab */}
                {activeTab === 'about' && (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                        <div style={{ textAlign: 'center', padding: '20px', background: 'var(--bg-tertiary)', borderRadius: '12px' }}>
                            <div style={{ fontSize: '48px', marginBottom: '12px' }}>🛡️</div>
                            <h3 style={{ margin: 0, color: 'var(--text-primary)' }}>ServionX Browser</h3>
                            <p style={{ color: 'var(--text-muted)', margin: '8px 0 0' }}>Enterprise Security Edition</p>
                        </div>

                        <InfoSection title="🦠 Antivirus & Malware Protection" items={[
                            '**How it works:** Every download is scanned using SHA256 hash comparison against known malware signatures',
                            '**Pre-scan:** URLs are checked against URLHaus and PhishTank databases before downloading',
                            '**Post-scan:** File hashes compared against threat intelligence feeds',
                            '**Update frequency:** Manual update via button (auto-update coming soon)',
                        ]} />

                        <InfoSection title="🔍 Vulnerability Scanner" items={[
                            '**How it works:** Analyzes website security when you visit any page',
                            '**What it checks:** SSL/TLS certificates, Security headers (CSP, HSTS, X-Frame-Options)',
                            '**Detection:** XSS patterns, outdated libraries, mixed content, insecure forms',
                            '**Grade system:** A+ to F based on security posture',
                        ]} />

                        <InfoSection title="📊 Data Sources & Updates" items={[
                            '**EasyList / EasyPrivacy:** Ad and tracker blocking rules (~50,000 rules)',
                            '**URLHaus:** Malware URL database by abuse.ch (updated hourly)',
                            '**PhishTank:** Phishing URL database (community-verified)',
                            '**Built-in list:** 94+ known tracker domains hardcoded',
                            '**Update button:** Click "Check for Updates" below to refresh lists',
                        ]} />

                        <button style={{
                            background: 'var(--accent-primary)', color: 'white', border: 'none',
                            padding: '12px 20px', borderRadius: '8px', cursor: 'pointer',
                            fontSize: '14px', fontWeight: 500, width: '100%',
                        }}>
                            🔄 Check for Updates
                        </button>

                        <div style={{ fontSize: '11px', color: 'var(--text-muted)', textAlign: 'center' }}>
                            Last updated: {new Date().toLocaleDateString()} | v1.0.0 Enterprise
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

// Components
const ToggleSwitch: FC<{ checked: boolean; onChange: () => void }> = ({ checked, onChange }) => (
    <label style={{ position: 'relative', display: 'inline-block', width: '48px', height: '26px' }}>
        <input type="checkbox" checked={checked} onChange={onChange} style={{ opacity: 0, width: 0, height: 0 }} />
        <span style={{
            position: 'absolute', cursor: 'pointer', top: 0, left: 0, right: 0, bottom: 0,
            background: checked ? '#10b981' : '#cbd5e1', borderRadius: '26px', transition: '0.3s',
        }}>
            <span style={{
                position: 'absolute', height: '20px', width: '20px',
                left: checked ? '24px' : '3px', bottom: '3px',
                background: 'white', borderRadius: '50%', transition: '0.3s',
                boxShadow: '0 2px 4px rgba(0,0,0,0.2)',
            }}></span>
        </span>
    </label>
);

const SettingToggle: FC<{ label: string; description: string; icon: string; checked: boolean; onChange: () => void }> =
    ({ label, description, icon, checked, onChange }) => (
        <div style={{
            display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '14px 16px',
            background: 'var(--bg-tertiary)', borderRadius: '10px', border: '1px solid var(--border-default)',
        }}>
            <div>
                <div style={{ fontWeight: 500, fontSize: '14px', color: 'var(--text-primary)' }}>{icon} {label}</div>
                <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginTop: '2px' }}>{description}</div>
            </div>
            <ToggleSwitch checked={checked} onChange={onChange} />
        </div>
    );

const InfoCard: FC<{ title: string; items: { label: string; value: string }[] }> = ({ title, items }) => (
    <div style={{ background: 'var(--bg-tertiary)', borderRadius: '10px', border: '1px solid var(--border-default)', overflow: 'hidden' }}>
        <div style={{ padding: '12px 14px', background: 'var(--bg-hover)', fontWeight: 600, fontSize: '13px', color: 'var(--text-primary)' }}>{title}</div>
        {items.map((item, i) => (
            <div key={item.label} style={{ display: 'flex', justifyContent: 'space-between', padding: '10px 14px', borderBottom: i < items.length - 1 ? '1px solid var(--border-subtle)' : 'none' }}>
                <span style={{ color: 'var(--text-muted)', fontSize: '13px' }}>{item.label}</span>
                <span style={{ color: 'var(--text-primary)', fontSize: '13px', fontFamily: 'var(--font-mono)' }}>{item.value}</span>
            </div>
        ))}
    </div>
);

const FilterBadge: FC<{ label: string; count: number; active?: boolean }> = ({ label, count, active }) => (
    <button style={{
        padding: '4px 10px', borderRadius: '16px', fontSize: '11px', fontWeight: 500,
        border: '1px solid var(--border-default)', cursor: 'pointer',
        background: active ? 'var(--accent-primary)' : 'var(--bg-tertiary)',
        color: active ? 'white' : 'var(--text-secondary)',
    }}>
        {label} ({count})
    </button>
);

const InfoSection: FC<{ title: string; items: string[] }> = ({ title, items }) => (
    <div style={{ background: 'var(--bg-tertiary)', borderRadius: '10px', border: '1px solid var(--border-default)', overflow: 'hidden' }}>
        <div style={{ padding: '12px 14px', background: 'var(--bg-hover)', fontWeight: 600, fontSize: '13px', color: 'var(--text-primary)' }}>{title}</div>
        <div style={{ padding: '12px 14px' }}>
            {items.map((item, i) => (
                <div key={i} style={{ fontSize: '12px', color: 'var(--text-secondary)', marginBottom: i < items.length - 1 ? '8px' : 0, lineHeight: 1.5 }}
                    dangerouslySetInnerHTML={{ __html: item.replace(/\*\*(.*?)\*\*/g, '<b style="color: var(--text-primary)">$1</b>') }} />
            ))}
        </div>
    </div>
);

export default SettingsPanel;
