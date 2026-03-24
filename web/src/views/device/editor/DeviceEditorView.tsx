import { AppShell, Text, Title } from '@mantine/core';
import { Fragment } from 'react';

function DeviceEditorView() {
  return (
    <Fragment>
      <AppShell.Navbar p="md">
        <Title order={4}>Presets</Title>
      </AppShell.Navbar>
      <AppShell.Main>
        <Text>This is the main section, where the device editor will go</Text>
      </AppShell.Main>
    </Fragment>
  );
}

export default DeviceEditorView;
