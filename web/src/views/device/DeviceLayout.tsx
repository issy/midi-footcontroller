import { Outlet } from 'react-router';
import ConnectionManagerProvider from '@/views/device/connection-manager/ConnectionManagerProvider';
import { AppShell, Button, Group, Text } from '@mantine/core';
import ColourSchemeButton from '@/components/ColourSchemeButton';
import { useConnectionManagerContext } from '@/views/device/connection-manager/context';
import { MdOutlineUsb, MdOutlineUsbOff } from 'react-icons/md';

function BrowserNotSupported() {
  return (
    <aside role="alert">
      <h3>Your browser is not supported</h3>
      <p>Please use a browser with Web Serial support (such as Chrome) in order to connect to your device</p>
    </aside>
  );
}

const ConnectionStatus = () => {
  const conn = useConnectionManagerContext();

  return conn.isConnected ? (
    <Button color="red" leftSection={<MdOutlineUsbOff />}>
      <Text>Disconnect</Text>
    </Button>
  ) : (
    <Button leftSection={<MdOutlineUsb />}>
      <Text>Connect</Text>
    </Button>
  );
};

function DeviceLayout() {
  if (!('serial' in navigator)) {
    return <BrowserNotSupported />;
  }

  return (
    <ConnectionManagerProvider>
      <AppShell.Header>
        <Group h="100%" px="md" justify="space-between">
          Header
          <Group>
            {/* TODO: Update button */}
            {/* TODO: Connection menu button */}
            <ConnectionStatus />
            <ColourSchemeButton />
          </Group>
        </Group>
      </AppShell.Header>
      <Outlet />
    </ConnectionManagerProvider>
  );
}

export default DeviceLayout;
