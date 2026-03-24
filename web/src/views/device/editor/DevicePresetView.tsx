import { AppShell } from '@mantine/core';
import { useParams } from 'react-router';

function DevicePresetView() {
  const { presetId } = useParams<{ presetId: string }>();

  return (
    <AppShell.Main>
      <div>
        <p>foobar</p>
        <p>{presetId}</p>
      </div>
    </AppShell.Main>
  );
}

export default DevicePresetView;
