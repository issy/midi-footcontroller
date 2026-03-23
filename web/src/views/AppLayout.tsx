import { AppShell } from '@mantine/core';
import { Outlet } from 'react-router';

function AppLayout() {
  return (
    <AppShell header={{ height: 60 }} navbar={{ width: '300', breakpoint: 'sm', collapsed: { desktop: false } }} padding="md">
      <Outlet />
    </AppShell>
  );
}

export default AppLayout;
