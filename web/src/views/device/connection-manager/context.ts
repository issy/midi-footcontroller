import { createContext, use } from 'react';

export type ConnectionManager =
  | {
      isConnected: false;
      connect: () => Promise<void>;
    }
  | {
      isConnected: true;
      disconnect: () => Promise<void>;
    };

export const ConnectionManagerContext = createContext<ConnectionManager | undefined>(undefined);
ConnectionManagerContext.displayName = 'ConnectionManagerContext';

export const useConnectionManagerContext = () => {
  const context = use(ConnectionManagerContext);
  if (context === undefined) {
    throw new Error('useConnectionManagerContext must be used within a ConnectionManagerProvider');
  }
  return context;
};
