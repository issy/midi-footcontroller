import { AppShell, AspectRatio, Button, type MantineColor, SimpleGrid, Title } from '@mantine/core';
import { useParams } from 'react-router';

function DeviceButton({ color, text }: { color: MantineColor; text: string }) {
  return (
    <AspectRatio ratio={3}>
      <Button color={color} fullWidth h="100%">
        <Title>{text}</Title>
      </Button>
    </AspectRatio>
  );
}

function DevicePresetView() {
  const { presetId } = useParams<{ presetId: string }>();

  return (
    <AppShell.Main>
      <SimpleGrid cols={4} spacing="md">
        <DeviceButton color="red" text="Hello" />
        <DeviceButton color="blue" text="Hello" />
        <DeviceButton color="green" text="Hello" />
        <DeviceButton color="yellow" text="Hello" />
        <DeviceButton color="grape" text="Hello" />
        <DeviceButton color="green" text="Hello" />
        <DeviceButton color="blue" text="Hello" />
        <DeviceButton color="teal" text="Hello" />
      </SimpleGrid>
      {presetId}
    </AppShell.Main>
  );
}

export default DevicePresetView;
