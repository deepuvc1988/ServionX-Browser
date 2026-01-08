import { FC, useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { KeyboardLayout, KeyInfo } from '../lib/types';

interface VirtualKeyboardProps {
    isOpen: boolean;
    onClose: () => void;
    onInput: (char: string) => void;
}

const VirtualKeyboard: FC<VirtualKeyboardProps> = ({ isOpen, onClose, onInput }) => {
    const [layout, setLayout] = useState<KeyboardLayout | null>(null);
    const [isShift, setIsShift] = useState(false);
    const [isCaps, setIsCaps] = useState(false);
    const [shuffled, setShuffled] = useState(false);
    const [inputValue, setInputValue] = useState('');

    useEffect(() => {
        if (isOpen) {
            loadLayout();
        }
    }, [isOpen, shuffled]);

    const loadLayout = async () => {
        try {
            const layout = await invoke<KeyboardLayout>('get_virtual_keyboard_layout', { shuffled });
            setLayout(layout);
        } catch (error) {
            console.error('Failed to load keyboard layout:', error);
        }
    };

    const handleKeyPress = async (key: KeyInfo) => {
        try {
            const result = await invoke<any>('process_virtual_key', {
                key: key.key,
                isShift,
                isCaps,
            });

            if (result.type === 'Character') {
                setInputValue(prev => prev + (typeof result === 'object' && 'Character' in result ? String.fromCharCode(result.Character) : key.key));
                onInput(key.key);
            } else if (result.type === 'Backspace') {
                setInputValue(prev => prev.slice(0, -1));
                onInput('Backspace');
            } else if (result.type === 'Enter') {
                onInput('Enter');
            } else if (result.type === 'Space') {
                setInputValue(prev => prev + ' ');
                onInput(' ');
            } else if (result.type === 'ModifierToggled') {
                if (result.modifier === 'Shift') {
                    setIsShift(result.active);
                } else if (result.modifier === 'CapsLock') {
                    setIsCaps(result.active);
                }
            }
        } catch (error) {
            console.error('Key processing error:', error);
        }
    };

    if (!isOpen || !layout) return null;

    return (
        <div className={`virtual-keyboard ${isOpen ? 'open' : ''}`}>
            <div className="keyboard-header">
                <div className="keyboard-title">
                    🔒 Secure Virtual Keyboard
                    {shuffled && <span className="badge badge-warning" style={{ marginLeft: 8 }}>Shuffled</span>}
                </div>
                <div style={{ display: 'flex', gap: 8 }}>
                    <button
                        className="btn btn-secondary"
                        style={{ padding: '4px 12px', fontSize: 12 }}
                        onClick={() => setShuffled(!shuffled)}
                    >
                        {shuffled ? 'Standard' : 'Shuffle'}
                    </button>
                    <button className="panel-close" onClick={onClose}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                            <line x1="18" y1="6" x2="6" y2="18" />
                            <line x1="6" y1="6" x2="18" y2="18" />
                        </svg>
                    </button>
                </div>
            </div>

            {/* Input preview */}
            <div style={{
                padding: '8px 12px',
                marginBottom: 12,
                background: 'var(--bg-tertiary)',
                borderRadius: 6,
                fontFamily: 'var(--font-mono)',
                minHeight: 36,
                display: 'flex',
                alignItems: 'center',
            }}>
                {inputValue || <span style={{ color: 'var(--text-muted)' }}>Type using the keyboard below...</span>}
                <span style={{
                    display: 'inline-block',
                    width: 2,
                    height: 18,
                    background: 'var(--accent-primary)',
                    marginLeft: 2,
                    animation: 'blink 1s infinite',
                }} />
            </div>

            {layout.rows.map((row, rowIndex) => (
                <div key={rowIndex} className="keyboard-row">
                    {row.map((key, keyIndex) => (
                        <button
                            key={`${rowIndex}-${keyIndex}`}
                            className={`keyboard-key ${key.width > 1.2 ? 'wide' : ''} ${key.keyType === 'Space' ? 'space' : ''}`}
                            style={{ minWidth: key.width * 40 }}
                            onClick={() => handleKeyPress(key)}
                        >
                            {key.keyType === 'Shift' && isShift ? '⇧ ON' :
                                key.keyType === 'CapsLock' && isCaps ? '⇪ ON' :
                                    (isShift || isCaps) && key.keyType === 'Character' ? key.display.toUpperCase() :
                                        key.display}
                        </button>
                    ))}
                </div>
            ))}

            <style>{`
        @keyframes blink {
          0%, 50% { opacity: 1; }
          51%, 100% { opacity: 0; }
        }
      `}</style>
        </div>
    );
};

export default VirtualKeyboard;
