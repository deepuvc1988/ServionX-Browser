import { FC } from 'react';
import { Tab, PrivacyStatus } from '../lib/types';

interface BrowserChromeProps {
    tabs: Tab[];
    activeTabId: string;
    onTabSelect: (id: string) => void;
    onTabClose: (id: string) => void;
    onNewTab: () => void;
    currentUrl: string;
    onNavigate: (url: string) => void;
    isLoading: boolean;
    isSecure: boolean;
    privacyStatus: PrivacyStatus;
    onSettingsClick: () => void;
    onToolsClick: () => void;
    onDownloadsClick: () => void;
    onRegenerateIdentity: () => void;
}

const BrowserChrome: FC<BrowserChromeProps> = ({
    tabs,
    activeTabId,
    onTabSelect,
    onTabClose,
    onNewTab,
    currentUrl,
    onNavigate,
    isLoading,
    isSecure,
    onSettingsClick,
    onToolsClick,
    onDownloadsClick,
    onRegenerateIdentity,
}) => {
    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            onNavigate((e.target as HTMLInputElement).value);
        }
    };

    return (
        <div className="browser-chrome">
            {/* Tab Bar */}
            <div className="tab-bar">
                {tabs.map(tab => (
                    <div
                        key={tab.id}
                        className={`tab ${tab.id === activeTabId ? 'active' : ''}`}
                        onClick={() => onTabSelect(tab.id)}
                    >
                        {tab.isLoading ? (
                            <div className="loading-spinner" style={{ width: 14, height: 14 }} />
                        ) : (
                            <TabIcon secure={tab.isSecure} />
                        )}
                        <span className="tab-title">{tab.title}</span>
                        <div
                            className="tab-close"
                            onClick={(e) => {
                                e.stopPropagation();
                                onTabClose(tab.id);
                            }}
                        >
                            <CloseIcon />
                        </div>
                    </div>
                ))}
                <div className="new-tab-btn" onClick={onNewTab}>
                    <PlusIcon />
                </div>
            </div>

            {/* Navigation Bar */}
            <div className="nav-bar">
                <div className="nav-buttons">
                    <button className="nav-btn" disabled>
                        <BackIcon />
                    </button>
                    <button className="nav-btn" disabled>
                        <ForwardIcon />
                    </button>
                    <button className="nav-btn" onClick={() => onNavigate(currentUrl)}>
                        {isLoading ? <CloseIcon /> : <RefreshIcon />}
                    </button>
                </div>

                <div className="address-bar">
                    <div className={`security-indicator ${isSecure ? 'secure' : 'insecure'}`}>
                        {isSecure ? <LockIcon /> : <UnlockIcon />}
                    </div>
                    <input
                        className="address-input"
                        type="text"
                        placeholder="Search or enter URL"
                        defaultValue={currentUrl}
                        onKeyDown={handleKeyDown}
                    />
                </div>

                <div
                    className="privacy-status"
                    onClick={onRegenerateIdentity}
                    title="Click to regenerate identity"
                >
                    <ShieldIcon />
                    <span>Protected</span>
                </div>

                <div className="chrome-actions">
                    <button className="chrome-btn tooltip" data-tooltip="Downloads" onClick={onDownloadsClick}>
                        <DownloadIcon />
                    </button>
                    <button className="chrome-btn tooltip" data-tooltip="IT Tools" onClick={onToolsClick}>
                        <TerminalIcon />
                    </button>
                    <button className="chrome-btn tooltip" data-tooltip="Settings" onClick={onSettingsClick}>
                        <SettingsIcon />
                    </button>
                </div>
            </div>
        </div>
    );
};

// Icon Components
const TabIcon: FC<{ secure: boolean }> = ({ secure }) => (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        {secure ? (
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
        ) : (
            <circle cx="12" cy="12" r="10" />
        )}
    </svg>
);

const CloseIcon = () => (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <line x1="18" y1="6" x2="6" y2="18" />
        <line x1="6" y1="6" x2="18" y2="18" />
    </svg>
);

const PlusIcon = () => (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <line x1="12" y1="5" x2="12" y2="19" />
        <line x1="5" y1="12" x2="19" y2="12" />
    </svg>
);

const BackIcon = () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <polyline points="15 18 9 12 15 6" />
    </svg>
);

const ForwardIcon = () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <polyline points="9 18 15 12 9 6" />
    </svg>
);

const RefreshIcon = () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <polyline points="23 4 23 10 17 10" />
        <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
    </svg>
);

const LockIcon = () => (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
        <path d="M7 11V7a5 5 0 0 1 10 0v4" />
    </svg>
);

const UnlockIcon = () => (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
        <path d="M7 11V7a5 5 0 0 1 9.9-1" />
    </svg>
);

const ShieldIcon = () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
        <polyline points="9 12 11 14 15 10" />
    </svg>
);

const DownloadIcon = () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
        <polyline points="7 10 12 15 17 10" />
        <line x1="12" y1="15" x2="12" y2="3" />
    </svg>
);

const TerminalIcon = () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <polyline points="4 17 10 11 4 5" />
        <line x1="12" y1="19" x2="20" y2="19" />
    </svg>
);

const SettingsIcon = () => (
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
        <circle cx="12" cy="12" r="3" />
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
    </svg>
);

export default BrowserChrome;
