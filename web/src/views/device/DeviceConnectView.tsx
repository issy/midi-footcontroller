import { useConnectionManagerContext } from '@/views/device/connection-manager/context';
import { AppShell, Text } from '@mantine/core';

// TODO: This should just show a big error state that the device is not connected
function DeviceConnectView() {
  const conn = useConnectionManagerContext();

  return (
    <AppShell.Main>
      <div>
        {conn.isConnected ? (
          <div>
            <Text>Connected</Text>
          </div>
        ) : (
          <div>
            <Text>Disconnected</Text>
            <Text>Testing!!!</Text>
          </div>
        )}
      </div>
    </AppShell.Main>
  );
}

export default DeviceConnectView;
