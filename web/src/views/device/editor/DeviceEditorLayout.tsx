import { AppShell, NavLink, Stack, Title } from '@mantine/core';
import { Fragment, useMemo } from 'react';
import { Outlet, useLocation, useNavigate, useResolvedPath } from 'react-router';
import classes from './DeviceEditorLayout.module.scss';
import usePresets from '@/views/device/editor/use-presets';

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
  const { data: presets } = usePresets();

  return (
    <Fragment>
      <AppShell.Navbar p="sm">
        <Stack>
          <Title order={4}>Presets</Title>
          {/* Presets */}
          {presets !== undefined && (
            <Stack gap={0}>
              {presets.map(({ id, name }) => (
                <NavListItem key={id} path={id} label={name} />
              ))}
            </Stack>
          )}
        </Stack>
      </AppShell.Navbar>
      <Outlet />
    </Fragment>
  );
}

export default DeviceEditorLayout;
