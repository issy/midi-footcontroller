import { createContext, use, useCallback, useEffect, useState } from 'react';

export type ConnectionManager =
  | {
      isConnected: false;
      connect: () => Promise<void>;
    }
  | {
      isConnected: true;
      disconnect: () => Promise<void>;
    };

const ConnectionManagerContext = createContext<ConnectionManager | undefined>(undefined);
ConnectionManagerContext.displayName = 'ConnectionManagerContext';
export const ConnectionManagerProvider = ConnectionManagerContext.Provider;

export const useConnectionManagerContext = () => {
  const context = use(ConnectionManagerContext);
  if (context === undefined) {
    throw new Error('useConnectionManagerContext must be used within a ConnectionManagerProvider');
  }
  return context;
};

export const useConnectionManager = (): ConnectionManager => {
  const [_port, _setPort] = useState<SerialPort>();
  const [isConnected, setIsConnected] = useState(false);

  const connect = useCallback(async () => {
    const port = await navigator.serial.requestPort();
    await port.open({ baudRate: 31_250 });
    _setPort(port);
  }, []);

  const disconnect = useCallback(async () => {
    const ports = await navigator.serial.getPorts();
    for (const port of ports) {
      await port.close();
    }
  }, []);

  useEffect(() => {
    if (_port === undefined) return;

    function onConnectListener() {
      setIsConnected(true);
      console.log('Connected');
    }

    _port.addEventListener('connect', onConnectListener);
    return () => {
      _port.removeEventListener('connect', onConnectListener);
    };
  }, [_port]);

  useEffect(() => {
    if (_port === undefined) return;

    function onDisconnectListener() {
      setIsConnected(false);
      console.log('Disconnected');
    }

    _port.addEventListener('disconnect', onDisconnectListener);
    return () => {
      _port.removeEventListener('disconnect', onDisconnectListener);
    };
  }, [_port]);

  return isConnected ? { isConnected: true, disconnect } : { isConnected: false, connect };
};
