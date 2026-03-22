import { Outlet } from 'react-router';
import { AppShell } from '@mantine/core';

function AppLayout() {
  return (
    <AppShell mah="100vh" navbar={{ width: '300', breakpoint: 'sm', collapsed: { desktop: false } }}>
      <Outlet />
    </AppShell>
  );
}

export default AppLayout;
