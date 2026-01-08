export interface Tab {
    id: string;
    title: string;
    url: string;
    isLoading: boolean;
    isSecure: boolean;
    isWhitelisted?: boolean;
    favicon?: string;
}

export interface PrivacyStatus {
    identityId: string;
    isProtected: boolean;
    trackersBlocked: number;
    fingerprintsBlocked: number;
}

export interface FakeFingerprint {
    sessionId: string;
    hardwareConcurrency: number;
    deviceMemory: number;
    canvasNoiseSeed: number;
    webglVendor: string;
    webglRenderer: string;
    webglVersion: string;
    audioNoiseSeed: number;
    installedFonts: string[];
    plugins: FakePlugin[];
    maxTouchPoints: number;
    touchSupport: boolean;
}

export interface FakePlugin {
    name: string;
    description: string;
    filename: string;
}

export interface FakeGeolocation {
    latitude: number;
    longitude: number;
    accuracy: number;
    city: string;
    country: string;
    countryCode: string;
}

export interface FakeUserAgent {
    full: string;
    appVersion: string;
    platform: string;
    vendor: string;
    browserName: string;
    browserVersion: string;
    osName: string;
    osVersion: string;
}

export interface FakeIdentity {
    id: string;
    createdAt: string;
    fingerprint: FakeFingerprint;
    geolocation: FakeGeolocation;
    userAgent: FakeUserAgent;
    ipHeaders: Record<string, string>;
    timezone: string;
    language: string;
    doNotTrack: boolean;
}

export interface KeyboardLayout {
    rows: KeyInfo[][];
    layoutId: string;
    isShuffled: boolean;
}

export interface KeyInfo {
    key: string;
    display: string;
    keyType: 'Character' | 'Shift' | 'Backspace' | 'Enter' | 'Space' | 'Tab' | 'CapsLock' | 'Number' | 'Symbol';
    width: number;
}

export interface LogEntry {
    id: string;
    timestamp: string;
    level: 'Debug' | 'Info' | 'Warning' | 'Error' | 'Security';
    category: string;
    message: string;
    details?: string;
}

export interface SshConnectionInfo {
    id: string;
    host: string;
    port: number;
    username: string;
    connected: boolean;
    connectedAt?: string;
}

export interface PingResult {
    host: string;
    ip?: string;
    reachable: boolean;
    latencyMs?: number;
    packetsSent: number;
    packetsReceived: number;
    packetLossPercent: number;
}

export interface PortScanResult {
    host: string;
    port: number;
    open: boolean;
    service?: string;
    latencyMs?: number;
}

export interface DnsResult {
    domain: string;
    addresses: string[];
    lookupTimeMs: number;
    recordType: string;
}

export interface HttpResult {
    url: string;
    statusCode: number;
    statusText: string;
    headers: Record<string, string>;
    body?: string;
    latencyMs: number;
    contentLength?: number;
}
