import { AppShell, AspectRatio, Button, Card, Grid, type MantineColor, SimpleGrid, Stack, Text, Title } from '@mantine/core';
import { useParams } from 'react-router';
import { Fragment, useState } from 'react';
import { MdAdd } from 'react-icons/md';

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

const mockPresetButtonsData: Array<PresetData> = [
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

type ButtonActionData = {
  channel: number;
} & (
  | { type: 'PROGRAM_CHANGE'; program: number }
  | { type: 'CONTROL_CHANGE'; control: number; value: number }
  | { type: 'NOTE_ON'; note: number; velocity: number }
  | { type: 'NOTE_OFF'; note: number }
);

type ButtonActionDataType = ButtonActionData['type'];

const formatButtonActionDataType = (type: ButtonActionDataType): string => {
  switch (type) {
    case 'PROGRAM_CHANGE':
      return 'PC';
    case 'CONTROL_CHANGE':
      return 'CC';
    case 'NOTE_ON':
      return 'Note On';
    case 'NOTE_OFF':
      return 'Note Off';
  }
};

const mockButtonActions: Array<ButtonActionData> = [
  {
    type: 'PROGRAM_CHANGE',
    channel: 1,
    program: 40,
  },
  {
    type: 'CONTROL_CHANGE',
    channel: 2,
    control: 1,
    value: 127,
  },
  {
    type: 'NOTE_ON',
    channel: 3,
    note: 60,
    velocity: 127,
  },
  {
    type: 'NOTE_OFF',
    channel: 4,
    note: 60,
  },
];

function DevicePresetView() {
  const { presetId } = useParams<{ presetId: string }>();
  const [activeButton, setActiveButton] = useState<string | undefined>(undefined);

  return (
    <AppShell.Main>
      <Stack gap="xs">
        <Title>{presetId}</Title>
        <SimpleGrid cols={4} spacing="md">
          {mockPresetButtonsData.map((button) => (
            <DeviceButton
              key={`button_${button.id}`}
              color={button.color}
              text={button.text}
              active={activeButton === button.id}
              onClick={() => {
                setActiveButton(button.id);
              }}
            />
          ))}
        </SimpleGrid>
        <Grid columns={2}>
          <Grid.Col span={1}>
            <Stack>
              <Card>
                <Title size="h3">Button editor</Title>
              </Card>
              <Card>
                <Text>Action editor</Text>
              </Card>
            </Stack>
          </Grid.Col>
          <Grid.Col span={1}>
            <Stack>
              {mockButtonActions.map((action, index) => (
                <Card key={`action_${index.toString()}`}>
                  <SimpleGrid cols={4} spacing="sm" component="dl" m={0}>
                    <div style={{ display: 'block' }}>
                      <Text fw={700}>Type</Text>
                      <Text>{formatButtonActionDataType(action.type)}</Text>
                    </div>
                    <div style={{ display: 'block' }}>
                      <Text fw={700}>Channel</Text>
                      <Text>{action.channel}</Text>
                    </div>
                    {action.type === 'PROGRAM_CHANGE' && (
                      <div style={{ display: 'block' }}>
                        <Text fw={700}>Program</Text>
                        <Text>{action.program}</Text>
                      </div>
                    )}
                    {action.type === 'CONTROL_CHANGE' && (
                      <Fragment>
                        <div style={{ display: 'block' }}>
                          <Text fw={700}>Control</Text>
                          <Text>{action.control}</Text>
                        </div>
                        <div style={{ display: 'block' }}>
                          <Text fw={700}>Value</Text>
                          <Text>{action.value}</Text>
                        </div>
                      </Fragment>
                    )}
                    {action.type === 'NOTE_ON' && (
                      <Fragment>
                        <div style={{ display: 'block' }}>
                          <Text fw={700}>Note</Text>
                          <Text>{action.note}</Text>
                        </div>
                        <div style={{ display: 'block' }}>
                          <Text fw={700}>Velocity</Text>
                          <Text>{action.velocity}</Text>
                        </div>
                      </Fragment>
                    )}
                    {action.type === 'NOTE_OFF' && (
                      <div style={{ display: 'block' }}>
                        <Text fw={700}>Note</Text>
                        <Text>{action.note}</Text>
                      </div>
                    )}
                  </SimpleGrid>
                </Card>
              ))}
              <Button
                onClick={() => {
                  /* empty */
                }}
                style={{ width: '100%' }}
                variant="light"
                color="gray"
                size="lg"
                leftSection={<MdAdd />}
              >
                <Text>Add</Text>
              </Button>
            </Stack>
          </Grid.Col>
        </Grid>
      </Stack>
    </AppShell.Main>
  );
}

export default DevicePresetView;
