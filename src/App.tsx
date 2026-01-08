import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import BrowserChrome from './components/BrowserChrome';
import NewTabPage from './components/NewTabPage';
import SettingsPanel from './components/SettingsPanel';
import ITToolsPanel from './components/ITToolsPanel';
import VirtualKeyboard from './components/VirtualKeyboard';
import DownloadManager from './components/DownloadManager';
import { Tab, PrivacyStatus, FakeFingerprint, FakeGeolocation, FakeUserAgent } from './lib/types';

interface PrivacyDetails {
  fingerprint: FakeFingerprint | null;
  geolocation: FakeGeolocation | null;
  userAgent: FakeUserAgent | null;
}

function App() {
  // Tab management
  const [tabs, setTabs] = useState<Tab[]>([
    { id: '1', title: 'New Tab', url: '', isLoading: false, isSecure: true }
  ]);
  const [activeTabId, setActiveTabId] = useState('1');

  // Panel states
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [toolsOpen, setToolsOpen] = useState(false);
  const [keyboardOpen, setKeyboardOpen] = useState(false);
  const [downloadsOpen, setDownloadsOpen] = useState(false);

  // Privacy status
  const [privacyStatus, setPrivacyStatus] = useState<PrivacyStatus>({
    identityId: '',
    isProtected: true,
    trackersBlocked: 0,
    fingerprintsBlocked: 0,
  });

  // Detailed privacy info
  const [privacyDetails, setPrivacyDetails] = useState<PrivacyDetails>({
    fingerprint: null,
    geolocation: null,
    userAgent: null,
  });

  // Settings state
  const [isSettingsLocked, setIsSettingsLocked] = useState(true);

  // Initialize privacy engine and load all privacy details
  useEffect(() => {
    const initPrivacy = async () => {
      try {
        const [fingerprint, geolocation, userAgent] = await Promise.all([
          invoke<FakeFingerprint>('get_fake_fingerprint'),
          invoke<FakeGeolocation>('get_fake_geolocation'),
          invoke<FakeUserAgent>('get_fake_user_agent'),
        ]);

        setPrivacyDetails({ fingerprint, geolocation, userAgent });
        setPrivacyStatus((prev: PrivacyStatus) => ({
          ...prev,
          identityId: fingerprint.sessionId,
        }));
      } catch (error) {
        console.error('Failed to initialize privacy engine:', error);
      }
    };

    initPrivacy();
  }, []);

  // Tab management functions
  const addTab = useCallback(() => {
    const newTab: Tab = {
      id: Date.now().toString(),
      title: 'New Tab',
      url: '',
      isLoading: false,
      isSecure: true,
    };
    setTabs((prev: Tab[]) => [...prev, newTab]);
    setActiveTabId(newTab.id);
  }, []);

  const closeTab = useCallback((tabId: string) => {
    invoke('close_browser_tab', { tabId }).catch(() => { });

    setTabs((prev: Tab[]) => {
      const newTabs = prev.filter(t => t.id !== tabId);
      if (newTabs.length === 0) {
        return [{ id: Date.now().toString(), title: 'New Tab', url: '', isLoading: false, isSecure: true }];
      }
      return newTabs;
    });

    if (activeTabId === tabId) {
      setActiveTabId((prev: string) => {
        const currentIndex = tabs.findIndex(t => t.id === prev);
        const newTabs = tabs.filter(t => t.id !== tabId);
        if (newTabs.length === 0) return '';
        const newIndex = Math.min(currentIndex, newTabs.length - 1);
        return newTabs[newIndex]?.id || newTabs[0].id;
      });
    }
  }, [activeTabId, tabs]);

  const updateTab = useCallback((tabId: string, updates: Partial<Tab>) => {
    setTabs((prev: Tab[]) => prev.map(tab =>
      tab.id === tabId ? { ...tab, ...updates } : tab
    ));
  }, []);

  const navigateTo = useCallback(async (url: string) => {
    let finalUrl = url;

    if (!url.includes('.') || url.includes(' ')) {
      finalUrl = `https://duckduckgo.com/?q=${encodeURIComponent(url)}`;
    } else if (!url.startsWith('http://') && !url.startsWith('https://')) {
      finalUrl = `https://${url}`;
    }

    updateTab(activeTabId, {
      url: finalUrl,
      isLoading: true,
      isSecure: finalUrl.startsWith('https://'),
    });

    try {
      await invoke('create_browser_tab', {
        tabId: activeTabId,
        url: finalUrl,
      });

      setTimeout(() => {
        updateTab(activeTabId, {
          isLoading: false,
          title: new URL(finalUrl).hostname
        });
      }, 2000);
    } catch (error) {
      console.error('Failed to create browser tab:', error);
      updateTab(activeTabId, { isLoading: false });
    }
  }, [activeTabId, updateTab]);

  const regenerateIdentity = useCallback(async () => {
    try {
      const identity = await invoke<any>('regenerate_identity');
      setPrivacyStatus((prev: PrivacyStatus) => ({
        ...prev,
        identityId: identity.fingerprint.sessionId,
      }));

      // Reload privacy details
      const [fingerprint, geolocation, userAgent] = await Promise.all([
        invoke<FakeFingerprint>('get_fake_fingerprint'),
        invoke<FakeGeolocation>('get_fake_geolocation'),
        invoke<FakeUserAgent>('get_fake_user_agent'),
      ]);
      setPrivacyDetails({ fingerprint, geolocation, userAgent });
    } catch (error) {
      console.error('Failed to regenerate identity:', error);
    }
  }, []);

  const activeTab = tabs.find(t => t.id === activeTabId) || tabs[0];
  const { fingerprint, geolocation, userAgent } = privacyDetails;

  return (
    <div className="app">
      <BrowserChrome
        tabs={tabs}
        activeTabId={activeTabId}
        onTabSelect={setActiveTabId}
        onTabClose={closeTab}
        onNewTab={addTab}
        currentUrl={activeTab?.url || ''}
        onNavigate={navigateTo}
        isLoading={activeTab?.isLoading || false}
        isSecure={activeTab?.isSecure || false}
        privacyStatus={privacyStatus}
        onSettingsClick={() => setSettingsOpen(true)}
        onToolsClick={() => setToolsOpen(true)}
        onDownloadsClick={() => setDownloadsOpen(true)}
        onRegenerateIdentity={regenerateIdentity}
      />

      <div className="main-content">
        {activeTab?.url ? (
          <div className="privacy-dashboard">
            <div className="dashboard-header">
              <div className="shield-icon-large">🛡️</div>
              <div>
                <h1>Privacy Protection Active</h1>
                <p className="current-url">Browsing: <code>{activeTab.url}</code></p>
              </div>
            </div>

            <div className="protection-grid">
              {/* Fingerprint Protection */}
              <div className="protection-card">
                <div className="card-header">
                  <span className="card-icon">🖥️</span>
                  <h3>Fingerprint Spoofing</h3>
                  <span className="status-badge active">ACTIVE</span>
                </div>
                <div className="card-content">
                  {fingerprint && (
                    <ul className="detail-list">
                      <li><span>Session ID:</span> <code>{fingerprint.sessionId.slice(0, 8)}...</code></li>
                      <li><span>Canvas Noise:</span> <code>{fingerprint.canvasNoiseSeed}</code></li>
                      <li><span>WebGL Vendor:</span> <code>{fingerprint.webglVendor}</code></li>
                      <li><span>WebGL Renderer:</span> <code>{fingerprint.webglRenderer?.slice(0, 30)}...</code></li>
                      <li><span>Audio Noise:</span> <code>{fingerprint.audioNoiseSeed}</code></li>
                      <li><span>CPU Cores (Fake):</span> <code>{fingerprint.hardwareConcurrency}</code></li>
                      <li><span>Memory (Fake):</span> <code>{fingerprint.deviceMemory} GB</code></li>
                    </ul>
                  )}
                </div>
              </div>

              {/* User Agent Spoofing */}
              <div className="protection-card">
                <div className="card-header">
                  <span className="card-icon">🌐</span>
                  <h3>User Agent Spoofing</h3>
                  <span className="status-badge active">ACTIVE</span>
                </div>
                <div className="card-content">
                  {userAgent && (
                    <ul className="detail-list">
                      <li><span>Browser:</span> <code>{userAgent.browserName} {userAgent.browserVersion}</code></li>
                      <li><span>OS:</span> <code>{userAgent.osName} {userAgent.osVersion}</code></li>
                      <li><span>Platform:</span> <code>{userAgent.platform}</code></li>
                      <li><span>Vendor:</span> <code>{userAgent.vendor}</code></li>
                      <li className="full-width"><span>Full UA:</span><br /><code className="small">{userAgent.full?.slice(0, 80)}...</code></li>
                    </ul>
                  )}
                </div>
              </div>

              {/* Geolocation Faking */}
              <div className="protection-card">
                <div className="card-header">
                  <span className="card-icon">📍</span>
                  <h3>Geolocation Faking</h3>
                  <span className="status-badge active">ACTIVE</span>
                </div>
                <div className="card-content">
                  {geolocation && (
                    <ul className="detail-list">
                      <li><span>City:</span> <code>{geolocation.city}</code></li>
                      <li><span>Country:</span> <code>{geolocation.country}</code></li>
                      <li><span>Latitude:</span> <code>{geolocation.latitude.toFixed(6)}</code></li>
                      <li><span>Longitude:</span> <code>{geolocation.longitude.toFixed(6)}</code></li>
                      <li><span>Accuracy:</span> <code>{geolocation.accuracy}m</code></li>
                    </ul>
                  )}
                </div>
              </div>

              {/* Additional Protections */}
              <div className="protection-card">
                <div className="card-header">
                  <span className="card-icon">🔒</span>
                  <h3>Additional Protections</h3>
                  <span className="status-badge active">ALL ON</span>
                </div>
                <div className="card-content">
                  <ul className="protection-list">
                    <li className="protected">✓ WebRTC IP Leak Prevention</li>
                    <li className="protected">✓ Canvas Fingerprint Noise</li>
                    <li className="protected">✓ Audio Fingerprint Noise</li>
                    <li className="protected">✓ WebGL Parameters Spoofed</li>
                    <li className="protected">✓ Timezone Spoofing</li>
                    <li className="protected">✓ Battery API Blocked</li>
                    <li className="protected">✓ Do-Not-Track Enabled</li>
                    <li className="protected">✓ Language Spoofing</li>
                  </ul>
                </div>
              </div>
            </div>

            <div className="dashboard-footer">
              <button className="btn btn-primary" onClick={regenerateIdentity}>
                🔄 Regenerate Identity
              </button>
              <p className="footer-note">
                Website is open in a separate protected window. All listed protections are injected into that browsing session.
              </p>
            </div>
          </div>
        ) : (
          <NewTabPage
            onNavigate={navigateTo}
            privacyStatus={privacyStatus}
          />
        )}
      </div>

      <SettingsPanel
        isOpen={settingsOpen}
        onClose={() => setSettingsOpen(false)}
        isLocked={isSettingsLocked}
        onUnlock={() => setIsSettingsLocked(false)}
        onShowKeyboard={() => setKeyboardOpen(true)}
      />

      <ITToolsPanel
        isOpen={toolsOpen}
        onClose={() => setToolsOpen(false)}
      />

      <DownloadManager
        isOpen={downloadsOpen}
        onClose={() => setDownloadsOpen(false)}
      />

      <VirtualKeyboard
        isOpen={keyboardOpen}
        onClose={() => setKeyboardOpen(false)}
        onInput={(char: string) => {
          console.log('Virtual key:', char);
        }}
      />
    </div>
  );
}

export default App;
