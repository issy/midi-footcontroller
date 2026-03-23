import { Outlet } from 'react-router';
import { ConnectionManagerProvider, useConnectionManager } from '@/views/device/connection-manager';
import { AppShell, Group } from '@mantine/core';
import ColourSchemeButton from '@/components/ColourSchemeButton';

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
      <AppShell.Header>
        <Group h="100%" px="md" justify="space-between">
          Header
          <Group>
            {/* TODO: Update button */}
            {/* TODO: Connection menu button */}
            <ColourSchemeButton />
          </Group>
        </Group>
      </AppShell.Header>
      <Outlet />
    </ConnectionManagerProvider>
  );
}

export default DeviceLayout;
