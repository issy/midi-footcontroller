import { useConnectionManagerContext } from '@/views/device/connection-manager/context';
import { AppShell, Container, Stack, Text, Title } from '@mantine/core';
import classes from './DeviceConnectView.module.scss';
import { MdOutlineUsbOff } from 'react-icons/md';

// TODO: This should just show a big error state that the device is not connected
function DeviceConnectView() {
  const conn = useConnectionManagerContext();

  if (!conn.isConnected) {
    return (
      <div className={classes.container}>
        <Container size="xl">
          <Stack>
            <div className={classes.containerInner}>
              <MdOutlineUsbOff size={60} className={classes.disconnectedIcon} />
              <Title className={classes.disconnectedText}>Disconnected</Title>
            </div>
          </Stack>
        </Container>
      </div>
    );
  }

  return (
    <AppShell.Main>
      <div>
        <div>
          <Text>Connected</Text>
        </div>
      </div>
    </AppShell.Main>
  );
}

export default DeviceConnectView;
