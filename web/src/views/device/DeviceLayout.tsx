import { Outlet } from 'react-router';
import { ConnectionManagerProvider, useConnectionManager } from '@/views/device/connection-manager';

function BrowserNotSupported() {
  return (
    <aside role="alert">
      <h3>Your browser is not supported</h3>
      <p>Please use a browser with Web Serial support (such as Chrome) in order to connect to your device</p>
    </aside>
  );
}

function DeviceLayout() {
  // TODO: Wrap the connection manager in a device wrapper
  const connectionManager = useConnectionManager();

  if (!('serial' in navigator)) {
    return <BrowserNotSupported />;
  }

  return (
    <ConnectionManagerProvider value={connectionManager}>
      <Outlet />
    </ConnectionManagerProvider>
  );
}

export default DeviceLayout;
