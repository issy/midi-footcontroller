import { AppShell, AspectRatio, Button, type MantineColor, SimpleGrid, Stack, Title } from '@mantine/core';
import { useParams } from 'react-router';
import { useState } from 'react';

function DeviceButton({
  color,
  text,
  active,
  onClick,
}: {
  color: MantineColor;
  text: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <AspectRatio ratio={3}>
      <Button variant={active ? 'filled' : 'light'} color={color} fullWidth h="100%" onClick={onClick}>
        <Title>{text}</Title>
      </Button>
    </AspectRatio>
  );
}

interface PresetData {
  color: MantineColor;
  text: string;
  id: string;
}

const mockPresetData: Array<PresetData> = [
  {
    id: 'one',
    color: 'red',
    text: 'Hello',
  },
  {
    id: 'two',
    color: 'green',
    text: 'Hello',
  },
  {
    id: 'three',
    color: 'blue',
    text: 'Hello',
  },
  {
    id: 'four',
    color: 'yellow',
    text: 'Hello',
  },
  {
    id: 'five',
    color: 'green',
    text: 'Hello',
  },
  {
    id: 'six',
    color: 'grape',
    text: 'Hello',
  },
  {
    id: 'seven',
    color: 'teal',
    text: 'Hello',
  },
  {
    id: 'eight',
    color: 'blue',
    text: 'Hello',
  },
];

function DevicePresetView() {
  const { presetId } = useParams<{ presetId: string }>();
  const [active, setActive] = useState<string | undefined>(undefined);

  return (
    <AppShell.Main>
      <Stack gap="xs">
        <Title>{presetId}</Title>
        <SimpleGrid cols={4} spacing="md">
          {mockPresetData.map((preset) => (
            <DeviceButton
              key={`button_${preset.id}`}
              color={preset.color}
              text={preset.text}
              active={active === preset.id}
              onClick={() => {
                setActive(preset.id);
              }}
            />
          ))}
        </SimpleGrid>
      </Stack>
    </AppShell.Main>
  );
}

export default DevicePresetView;
