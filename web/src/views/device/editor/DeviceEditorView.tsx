import { AppShell, Text } from '@mantine/core';
import { Fragment } from 'react';

function DeviceEditorView() {
  return (
    <Fragment>
      <AppShell.Navbar p="md">Navbar</AppShell.Navbar>
      <AppShell.Main>
        <Text>This is the main section, where the device editor will go</Text>
      </AppShell.Main>
    </Fragment>
  );
}

export default DeviceEditorView;
