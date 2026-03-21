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
  const [isConnected, setIsConnected] = useState(false);

  const connect = useCallback(async () => {
    const port = await navigator.serial.requestPort();
    await port.open({ baudRate: 31_250 });
  }, []);

  const disconnect = useCallback(async () => {
    const ports = await navigator.serial.getPorts();
    for (const port of ports) {
      await port.close();
    }
  }, []);

  useEffect(() => {
    function onConnectListener() {
      setIsConnected(true);
    }

    navigator.serial.addEventListener('connect', onConnectListener);
    return () => {
      navigator.serial.removeEventListener('connect', onConnectListener);
    };
  }, []);

  useEffect(() => {
    function onDisconnectListener() {
      setIsConnected(false);
    }

    navigator.serial.addEventListener('disconnect', onDisconnectListener);
    return () => {
      navigator.serial.removeEventListener('disconnect', onDisconnectListener);
    };
  }, []);

  return isConnected ? { isConnected: true, disconnect } : { isConnected: false, connect };
};
