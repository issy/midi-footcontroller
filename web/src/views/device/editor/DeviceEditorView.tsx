import { AppShell, Flex } from '@mantine/core';

function DeviceEditorView() {
  return (
    <Flex>
      <AppShell.Navbar pos="static">
        {/* Preset list */}
        <p>Foo</p>
        <p>Bar</p>
        <p>Baz</p>
        <p>Hello</p>
        <p>World</p>
      </AppShell.Navbar>
      <AppShell.Main p="md">
        {/* Preset editor */}
        <p>Main content goes here I suppose, but is this underneath the navbar?</p>
      </AppShell.Main>
    </Flex>
  );
}

export default DeviceEditorView;
