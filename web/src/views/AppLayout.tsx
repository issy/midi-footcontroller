import { Outlet } from 'react-router';
import { ActionIcon, AppShell, Group, useComputedColorScheme, useMantineColorScheme } from '@mantine/core';
import { MdDarkMode, MdLightMode } from 'react-icons/md';

function ColourSchemeButton() {
  const { setColorScheme } = useMantineColorScheme();
  const computedColorScheme = useComputedColorScheme(undefined, { getInitialValueInEffect: true });

  return (
    <Group justify="center">
      <ActionIcon
        onClick={() => {
          setColorScheme(computedColorScheme === 'light' ? 'dark' : 'light');
        }}
        variant="default"
        size="xl"
        radius="md"
        aria-label="Toggle color scheme"
      >
        {computedColorScheme === 'light' && <MdDarkMode />}
        {computedColorScheme === 'dark' && <MdLightMode />}
      </ActionIcon>
    </Group>
  );
}

function AppLayout() {
  return (
    <AppShell padding="md">
      <AppShell.Main>
        <ColourSchemeButton />
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}

export default AppLayout;
