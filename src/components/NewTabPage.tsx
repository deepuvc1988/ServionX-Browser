import { FC, useState } from 'react';
import { PrivacyStatus } from '../lib/types';

interface NewTabPageProps {
    onNavigate: (url: string) => void;
    privacyStatus: PrivacyStatus;
}

const NewTabPage: FC<NewTabPageProps> = ({ onNavigate, privacyStatus }) => {
    const [searchQuery, setSearchQuery] = useState('');

    const handleSearch = () => {
        if (searchQuery.trim()) {
            onNavigate(searchQuery);
        }
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter') {
            handleSearch();
        }
    };

    const shortcuts = [
        { title: 'DuckDuckGo', url: 'https://duckduckgo.com', icon: '🦆' },
        { title: 'GitHub', url: 'https://github.com', icon: '🐙' },
        { title: 'Wikipedia', url: 'https://wikipedia.org', icon: '📚' },
        { title: 'Reddit', url: 'https://reddit.com', icon: '🔴' },
    ];

    return (
        <div className="new-tab-page">
            <div className="ntp-logo">
                <svg width="120" height="120" viewBox="0 0 120 120" fill="none">
                    <defs>
                        <linearGradient id="logoGradient" x1="0%" y1="0%" x2="100%" y2="100%">
                            <stop offset="0%" stopColor="#6366f1" />
                            <stop offset="50%" stopColor="#a855f7" />
                            <stop offset="100%" stopColor="#ec4899" />
                        </linearGradient>
                    </defs>
                    <circle cx="60" cy="60" r="55" stroke="url(#logoGradient)" strokeWidth="4" fill="none" />
                    <path
                        d="M60 25 L60 45 M60 75 L60 95 M25 60 L45 60 M75 60 L95 60"
                        stroke="url(#logoGradient)"
                        strokeWidth="4"
                        strokeLinecap="round"
                    />
                    <circle cx="60" cy="60" r="20" fill="url(#logoGradient)" opacity="0.2" />
                    <path
                        d="M50 55 L55 60 L65 50"
                        stroke="url(#logoGradient)"
                        strokeWidth="3"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        fill="none"
                    />
                    <path
                        d="M35 60 A25 25 0 0 1 60 35"
                        stroke="url(#logoGradient)"
                        strokeWidth="3"
                        strokeLinecap="round"
                        fill="none"
                    />
                    <path
                        d="M85 60 A25 25 0 0 1 60 85"
                        stroke="url(#logoGradient)"
                        strokeWidth="3"
                        strokeLinecap="round"
                        fill="none"
                    />
                </svg>
            </div>

            <h1 className="ntp-title">ServionX Browser</h1>
            <p className="ntp-subtitle">The Most Advanced and Secure Browser</p>

            <div className="ntp-search">
                <svg className="ntp-search-icon" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <circle cx="11" cy="11" r="8" />
                    <line x1="21" y1="21" x2="16.65" y2="16.65" />
                </svg>
                <input
                    className="ntp-search-input"
                    type="text"
                    placeholder="Search the web privately..."
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    onKeyDown={handleKeyDown}
                />
            </div>

            <div className="ntp-shortcuts">
                {shortcuts.map((shortcut) => (
                    <div
                        key={shortcut.url}
                        className="ntp-shortcut"
                        onClick={() => onNavigate(shortcut.url)}
                    >
                        <div className="ntp-shortcut-icon">
                            <span style={{ fontSize: 24 }}>{shortcut.icon}</span>
                        </div>
                        <span className="ntp-shortcut-title">{shortcut.title}</span>
                    </div>
                ))}
            </div>

            <div className="privacy-stats">
                <div className="privacy-stat">
                    <div className="privacy-stat-value">{privacyStatus.trackersBlocked}</div>
                    <div className="privacy-stat-label">Trackers Blocked</div>
                </div>
                <div className="privacy-stat">
                    <div className="privacy-stat-value">{privacyStatus.fingerprintsBlocked}</div>
                    <div className="privacy-stat-label">Fingerprints Blocked</div>
                </div>
                <div className="privacy-stat">
                    <div className="privacy-stat-value" style={{ color: 'var(--accent-primary)' }}>
                        {privacyStatus.identityId ? privacyStatus.identityId.slice(0, 8) : '---'}
                    </div>
                    <div className="privacy-stat-label">Identity ID</div>
                </div>
            </div>
        </div>
    );
};

export default NewTabPage;
