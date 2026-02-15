import { Outlet } from 'react-router';
import { useCallback, useEffect, useState } from 'react';

function BrowserNotSupported() {
  return (
    <aside role="alert">
      <h3>Your browser is not supported</h3>
      <p>Please use a browser with Web Serial support (such as Chrome) in order to connect to your device</p>
    </aside>
  );
}

type ConnectionManager =
  | {
      isConnected: false;
      connect(): Promise<void>;
    }
  | {
      isConnected: true;
      disconnect(): Promise<void>;
    };

function useConnectionManager(): ConnectionManager {
  const [isConnected, setIsConnected] = useState(false);

  const connect = useCallback(async () => {
    const port = await navigator.serial.requestPort();
    await port.open({ baudRate: 115_200 });
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
}

function DeviceLayout() {
  // TODO: Use the device here and pass it down to the children through context
  useConnectionManager();

  if (!('serial' in navigator)) {
    return <BrowserNotSupported />;
  }

  return <Outlet />;
}

export default DeviceLayout;
