import { type ReactNode, useCallback, useEffect, useState } from 'react';
import { type ConnectionManager, ConnectionManagerContext } from './context';

const useConnectionManager = (): ConnectionManager => {
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

const ConnectionManagerProvider = ({ children }: { children: ReactNode }) => {
  // TODO: Wrap the connection manager in a device wrapper
  const connectionManager = useConnectionManager();

  return <ConnectionManagerContext value={connectionManager}>{children}</ConnectionManagerContext>;
};

export default ConnectionManagerProvider;
