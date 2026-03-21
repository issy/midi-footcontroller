import { createContext, use, useCallback, useEffect, useState } from 'react';

const connectToPort = async () => {
  const port = await navigator.serial.requestPort();
  await port.open({ baudRate: 115_200 }); // TODO: Is this the right baud rate? TBD
  return port;
};

type UseConnectionManagerReturn =
  | {
      port: undefined;
      connect: () => Promise<void>;
    }
  | {
      port: SerialPort;
      disconnect: () => Promise<void>;
    };

export const useConnectionManager = (): UseConnectionManagerReturn => {
  const [port, setPort] = useState<SerialPort | undefined>();

  const connect = useCallback(async () => {
    const port = await connectToPort();
    setPort(port);
  }, []);

  const disconnect = useCallback(async () => {
    if (port === undefined) return;
    await port.close();
  }, [port]);

  useEffect(() => {
    if (port === undefined) return;
    // Add event listeners for port events here
  }, [port]);

  if (port === undefined) return { port, connect };
  return { port, disconnect };
};

const DeviceConnectionContext = createContext<UseConnectionManagerReturn | undefined>(undefined);

export const useDeviceConnectionContext = () => {
  const context = use(DeviceConnectionContext);
  if (context === undefined) {
    throw new Error('useDeviceConnectionContext must be used within a DeviceConnectionProvider');
  }
  return context;
};

export default DeviceConnectionContext;
