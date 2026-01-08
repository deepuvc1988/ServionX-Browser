import { FC, useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { PingResult, PortScanResult, DnsResult, HttpResult } from '../lib/types';

interface ITToolsPanelProps {
    isOpen: boolean;
    onClose: () => void;
}

const ITToolsPanel: FC<ITToolsPanelProps> = ({ isOpen, onClose }) => {
    const [activeTab, setActiveTab] = useState<'ssh' | 'network' | 'http'>('ssh');

    // SSH state
    const [sshHost, setSshHost] = useState('');
    const [sshPort, setSshPort] = useState('22');
    const [sshUser, setSshUser] = useState('');
    const [sshPassword, setSshPassword] = useState('');
    const [sshConnected, setSshConnected] = useState(false);
    const [sshOutput, setSshOutput] = useState<string[]>([]);
    const [sshCommand, setSshCommand] = useState('');
    const [connectionId, setConnectionId] = useState('');

    // Network tools state
    const [networkHost, setNetworkHost] = useState('');
    const [networkTool, setNetworkTool] = useState<'ping' | 'port' | 'dns'>('ping');
    const [portRange, setPortRange] = useState('22,80,443,3306,5432');
    const [networkResult, setNetworkResult] = useState<any>(null);
    const [isLoading, setIsLoading] = useState(false);

    // HTTP state
    const [httpUrl, setHttpUrl] = useState('https://');
    const [httpMethod, setHttpMethod] = useState('GET');
    const [httpResult, setHttpResult] = useState<HttpResult | null>(null);

    const terminalRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (terminalRef.current) {
            terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
        }
    }, [sshOutput]);

    const handleSshConnect = async () => {
        try {
            setIsLoading(true);
            setSshOutput(['Connecting to ' + sshHost + '...']);

            const result = await invoke<any>('ssh_connect', {
                host: sshHost,
                port: parseInt(sshPort),
                username: sshUser,
                password: sshPassword,
            });

            setConnectionId(result.id);
            setSshConnected(true);
            setSshOutput(prev => [...prev, 'Connected successfully!', '']);
        } catch (error: any) {
            setSshOutput(prev => [...prev, 'Error: ' + error]);
        } finally {
            setIsLoading(false);
        }
    };

    const handleSshDisconnect = async () => {
        try {
            await invoke('ssh_disconnect', { connectionId });
            setSshConnected(false);
            setSshOutput(prev => [...prev, 'Disconnected.']);
            setConnectionId('');
        } catch (error: any) {
            setSshOutput(prev => [...prev, 'Error: ' + error]);
        }
    };

    const handleSshExecute = async () => {
        if (!sshCommand.trim()) return;

        try {
            setSshOutput(prev => [...prev, `$ ${sshCommand}`]);

            const result = await invoke<any>('ssh_execute', {
                connectionId,
                command: sshCommand,
            });

            if (result.stdout) {
                setSshOutput(prev => [...prev, result.stdout]);
            }
            if (result.stderr) {
                setSshOutput(prev => [...prev, 'stderr: ' + result.stderr]);
            }

            setSshCommand('');
        } catch (error: any) {
            setSshOutput(prev => [...prev, 'Error: ' + error]);
        }
    };

    const handlePing = async () => {
        try {
            setIsLoading(true);
            const result = await invoke<PingResult>('network_ping', { host: networkHost, count: 4 });
            setNetworkResult(result);
        } catch (error) {
            console.error('Ping failed:', error);
        } finally {
            setIsLoading(false);
        }
    };

    const handlePortScan = async () => {
        try {
            setIsLoading(true);
            const ports = portRange.split(',').map(p => parseInt(p.trim())).filter(p => !isNaN(p));
            const result = await invoke<PortScanResult[]>('network_port_scan', {
                host: networkHost,
                ports,
                timeoutMs: 1000,
            });
            setNetworkResult(result);
        } catch (error) {
            console.error('Port scan failed:', error);
        } finally {
            setIsLoading(false);
        }
    };

    const handleDnsLookup = async () => {
        try {
            setIsLoading(true);
            const result = await invoke<DnsResult>('network_dns_lookup', { domain: networkHost });
            setNetworkResult(result);
        } catch (error) {
            console.error('DNS lookup failed:', error);
        } finally {
            setIsLoading(false);
        }
    };

    const handleHttpRequest = async () => {
        try {
            setIsLoading(true);
            const result = await invoke<HttpResult>('http_request', {
                url: httpUrl,
                method: httpMethod,
                headers: null,
                body: null,
            });
            setHttpResult(result);
        } catch (error: any) {
            setHttpResult({
                url: httpUrl,
                statusCode: 0,
                statusText: 'Error: ' + error,
                headers: {},
                latencyMs: 0,
            });
        } finally {
            setIsLoading(false);
        }
    };

    if (!isOpen) return null;

    return (
        <div className={`panel ${isOpen ? 'open' : ''}`}>
            <div className="panel-header">
                <h2 className="panel-title">IT Tools</h2>
                <button className="panel-close" onClick={onClose}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                        <line x1="18" y1="6" x2="6" y2="18" />
                        <line x1="6" y1="6" x2="18" y2="18" />
                    </svg>
                </button>
            </div>

            <div className="panel-content">
                <div className="tools-tabs">
                    <button
                        className={`tools-tab ${activeTab === 'ssh' ? 'active' : ''}`}
                        onClick={() => setActiveTab('ssh')}
                    >
                        SSH/SFTP
                    </button>
                    <button
                        className={`tools-tab ${activeTab === 'network' ? 'active' : ''}`}
                        onClick={() => setActiveTab('network')}
                    >
                        Network
                    </button>
                    <button
                        className={`tools-tab ${activeTab === 'http' ? 'active' : ''}`}
                        onClick={() => setActiveTab('http')}
                    >
                        HTTP
                    </button>
                </div>

                {activeTab === 'ssh' && (
                    <div className="animate-fadeIn">
                        {!sshConnected ? (
                            <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
                                <input
                                    className="input"
                                    type="text"
                                    placeholder="Host"
                                    value={sshHost}
                                    onChange={(e) => setSshHost(e.target.value)}
                                />
                                <input
                                    className="input"
                                    type="text"
                                    placeholder="Port"
                                    value={sshPort}
                                    onChange={(e) => setSshPort(e.target.value)}
                                />
                                <input
                                    className="input"
                                    type="text"
                                    placeholder="Username"
                                    value={sshUser}
                                    onChange={(e) => setSshUser(e.target.value)}
                                />
                                <input
                                    className="input"
                                    type="password"
                                    placeholder="Password"
                                    value={sshPassword}
                                    onChange={(e) => setSshPassword(e.target.value)}
                                />
                                <button
                                    className="btn btn-primary"
                                    onClick={handleSshConnect}
                                    disabled={isLoading}
                                >
                                    {isLoading ? 'Connecting...' : 'Connect'}
                                </button>
                            </div>
                        ) : (
                            <div>
                                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 12 }}>
                                    <span className="badge badge-success">Connected to {sshHost}</span>
                                    <button className="btn btn-secondary" onClick={handleSshDisconnect}>
                                        Disconnect
                                    </button>
                                </div>
                            </div>
                        )}

                        <div className="terminal" ref={terminalRef} style={{ marginTop: 16 }}>
                            <div className="terminal-output">
                                {sshOutput.map((line, i) => (
                                    <div key={i}>{line}</div>
                                ))}
                            </div>
                            {sshConnected && (
                                <div className="terminal-input-line">
                                    <span className="terminal-prompt">$</span>
                                    <input
                                        className="terminal-input"
                                        type="text"
                                        value={sshCommand}
                                        onChange={(e) => setSshCommand(e.target.value)}
                                        onKeyDown={(e) => e.key === 'Enter' && handleSshExecute()}
                                        placeholder="Enter command..."
                                    />
                                </div>
                            )}
                        </div>
                    </div>
                )}

                {activeTab === 'network' && (
                    <div className="animate-fadeIn">
                        <div style={{ display: 'flex', gap: 8, marginBottom: 16 }}>
                            <button
                                className={`tools-tab ${networkTool === 'ping' ? 'active' : ''}`}
                                onClick={() => setNetworkTool('ping')}
                            >
                                Ping
                            </button>
                            <button
                                className={`tools-tab ${networkTool === 'port' ? 'active' : ''}`}
                                onClick={() => setNetworkTool('port')}
                            >
                                Port Scan
                            </button>
                            <button
                                className={`tools-tab ${networkTool === 'dns' ? 'active' : ''}`}
                                onClick={() => setNetworkTool('dns')}
                            >
                                DNS
                            </button>
                        </div>

                        <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
                            <input
                                className="input"
                                type="text"
                                placeholder="Host or Domain"
                                value={networkHost}
                                onChange={(e) => setNetworkHost(e.target.value)}
                            />

                            {networkTool === 'port' && (
                                <input
                                    className="input"
                                    type="text"
                                    placeholder="Ports (comma-separated)"
                                    value={portRange}
                                    onChange={(e) => setPortRange(e.target.value)}
                                />
                            )}

                            <button
                                className="btn btn-primary"
                                onClick={() => {
                                    if (networkTool === 'ping') handlePing();
                                    else if (networkTool === 'port') handlePortScan();
                                    else handleDnsLookup();
                                }}
                                disabled={isLoading || !networkHost}
                            >
                                {isLoading ? 'Running...' : 'Run'}
                            </button>
                        </div>

                        {networkResult && (
                            <div style={{ marginTop: 16, background: 'var(--bg-tertiary)', borderRadius: 8, padding: 16 }}>
                                {networkTool === 'ping' && (
                                    <div>
                                        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 8 }}>
                                            <span>Host: {networkResult.host}</span>
                                            <span className={`badge ${networkResult.reachable ? 'badge-success' : 'badge-error'}`}>
                                                {networkResult.reachable ? 'Reachable' : 'Unreachable'}
                                            </span>
                                        </div>
                                        {networkResult.ip && <div>IP: {networkResult.ip}</div>}
                                        {networkResult.latencyMs && <div>Latency: {networkResult.latencyMs}ms</div>}
                                        <div>Packet Loss: {networkResult.packetLossPercent}%</div>
                                    </div>
                                )}

                                {networkTool === 'port' && Array.isArray(networkResult) && (
                                    <div>
                                        {networkResult.map((port: PortScanResult) => (
                                            <div key={port.port} style={{ display: 'flex', justifyContent: 'space-between', padding: '4px 0' }}>
                                                <span>Port {port.port} {port.service && `(${port.service})`}</span>
                                                <span className={`badge ${port.open ? 'badge-success' : 'badge-error'}`}>
                                                    {port.open ? 'Open' : 'Closed'}
                                                </span>
                                            </div>
                                        ))}
                                    </div>
                                )}

                                {networkTool === 'dns' && (
                                    <div>
                                        <div>Domain: {networkResult.domain}</div>
                                        <div>Lookup Time: {networkResult.lookupTimeMs}ms</div>
                                        <div style={{ marginTop: 8 }}>Addresses:</div>
                                        {networkResult.addresses.map((addr: string, i: number) => (
                                            <div key={i} style={{ fontFamily: 'var(--font-mono)', marginLeft: 8 }}>{addr}</div>
                                        ))}
                                    </div>
                                )}
                            </div>
                        )}
                    </div>
                )}

                {activeTab === 'http' && (
                    <div className="animate-fadeIn">
                        <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
                            <div style={{ display: 'flex', gap: 8 }}>
                                <select
                                    className="input"
                                    style={{ width: 100 }}
                                    value={httpMethod}
                                    onChange={(e) => setHttpMethod(e.target.value)}
                                >
                                    <option>GET</option>
                                    <option>POST</option>
                                    <option>PUT</option>
                                    <option>DELETE</option>
                                    <option>HEAD</option>
                                </select>
                                <input
                                    className="input"
                                    type="text"
                                    placeholder="URL"
                                    value={httpUrl}
                                    onChange={(e) => setHttpUrl(e.target.value)}
                                />
                            </div>

                            <button
                                className="btn btn-primary"
                                onClick={handleHttpRequest}
                                disabled={isLoading || !httpUrl}
                            >
                                {isLoading ? 'Sending...' : 'Send Request'}
                            </button>
                        </div>

                        {httpResult && (
                            <div style={{ marginTop: 16, background: 'var(--bg-tertiary)', borderRadius: 8, padding: 16 }}>
                                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 8 }}>
                                    <span>Status: {httpResult.statusCode} {httpResult.statusText}</span>
                                    <span className={`badge ${httpResult.statusCode >= 200 && httpResult.statusCode < 300 ? 'badge-success' : 'badge-error'}`}>
                                        {httpResult.latencyMs}ms
                                    </span>
                                </div>

                                {httpResult.contentLength && (
                                    <div>Content Length: {httpResult.contentLength} bytes</div>
                                )}

                                <div style={{ marginTop: 8 }}>Headers:</div>
                                <div style={{ maxHeight: 150, overflowY: 'auto', fontSize: 12, fontFamily: 'var(--font-mono)' }}>
                                    {Object.entries(httpResult.headers).slice(0, 10).map(([key, value]) => (
                                        <div key={key}>{key}: {value}</div>
                                    ))}
                                </div>

                                {httpResult.body && (
                                    <>
                                        <div style={{ marginTop: 8 }}>Response Body:</div>
                                        <pre style={{ maxHeight: 200, overflowY: 'auto', fontSize: 12, whiteSpace: 'pre-wrap' }}>
                                            {httpResult.body.substring(0, 2000)}
                                            {httpResult.body.length > 2000 && '...'}
                                        </pre>
                                    </>
                                )}
                            </div>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
};

export default ITToolsPanel;
