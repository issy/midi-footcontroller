import { Outlet } from 'react-router';
import { AppShell } from '@mantine/core';

function AppLayout() {
  return (
    <AppShell padding="md">
      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
    </AppShell>
  );
}

export default AppLayout;
