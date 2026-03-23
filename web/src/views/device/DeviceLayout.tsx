import { Outlet } from 'react-router';
import ConnectionManagerProvider from '@/views/device/connection-manager/ConnectionManagerProvider';
import { ActionIcon, AppShell, Button, Group, Menu, Stack, Text } from '@mantine/core';
import ColourSchemeButton from '@/components/ColourSchemeButton';
import { useConnectionManagerContext } from '@/views/device/connection-manager/context';
import { MdOutlineUsb, MdOutlineUsbOff, MdArrowDropDown, MdCode } from 'react-icons/md';
import classes from './DeviceLayout.module.scss';

function DropdownButton() {
  return (
    <Group wrap="nowrap" gap={0}>
      <Button leftSection={<MdOutlineUsb />} className={classes.connectDropdownButton}>
        <Text>Connect</Text>
      </Button>
      <Menu transitionProps={{ transition: 'pop' }} position="bottom-end" withinPortal>
        <Menu.Target>
          <ActionIcon variant="filled" size={36} className={classes.connectDropdownIcon}>
            <MdArrowDropDown />
          </ActionIcon>
        </Menu.Target>
        <Menu.Dropdown>
          <Menu.Item
            leftSection={<MdCode />}
            onClick={() => {
              // TODO: Actually start and connect the simulator here
              console.log('Connect to simulator');
            }}
          >
            <Text>Connect to simulator</Text>
          </Menu.Item>
        </Menu.Dropdown>
      </Menu>
    </Group>
  );
}

function BrowserNotSupported() {
  return (
    <aside role="alert">
      <Stack>
        <Text size="lg" component="h3">
          Your browser is not supported
        </Text>
        <Text>Please use a browser with Web Serial support (such as Chrome) in order to connect to your device</Text>
      </Stack>
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
            <DropdownButton />
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
