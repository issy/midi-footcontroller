import { AppShell, NavLink, Stack, Title } from '@mantine/core';
import { Fragment, useMemo } from 'react';
import { Outlet, useLocation, useNavigate, useResolvedPath } from 'react-router';
import classes from './DeviceEditorLayout.module.scss';

function NavListItem({ path, label }: { path: string; label: string }) {
  const navigate = useNavigate();
  const resolvedPath = useResolvedPath(path);
  const location = useLocation();
  const active = useMemo(() => resolvedPath.pathname === location.pathname, [location.pathname, resolvedPath.pathname]);

  return (
    <NavLink
      onClick={() => {
        void navigate({ pathname: path });
      }}
      active={active}
      label={label}
      className={classes.navLink}
    />
  );
}

function DeviceEditorLayout() {
  return (
    <Fragment>
      <AppShell.Navbar p="md">
        <Title order={4}>Presets</Title>
        {/* Presets */}
        <Stack gap={0}>
          <NavListItem path="foo" label="Foo" />
          <NavListItem path="bar" label="Bar" />
          <NavListItem path="baz" label="Baz" />
        </Stack>
      </AppShell.Navbar>
      <Outlet />
    </Fragment>
  );
}

export default DeviceEditorLayout;
