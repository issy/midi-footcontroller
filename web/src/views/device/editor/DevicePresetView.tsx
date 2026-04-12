import { ActionIcon, AppShell, AspectRatio, Button, Card, Flex, Grid, SimpleGrid, Stack, Text, Title } from '@mantine/core';
import { useParams } from 'react-router';
import { Fragment, useState } from 'react';
import { MdAdd, MdClose, MdDelete, MdEdit, MdOutlineVisibility, MdOutlineVisibilityOff } from 'react-icons/md';
import ButtonActionEditForm from '@/views/device/editor/ButtonActionEditForm';
import ButtonEditForm from './ButtonEditForm';
import { type Colour, colourToMantineColour } from '@/utils/colourMapping';

function DeviceButton({
  colour,
  text,
  active,
  onClick,
}: {
  colour: Colour;
  text: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <AspectRatio ratio={3}>
      <Button variant={active ? 'filled' : 'light'} color={colourToMantineColour[colour]} fullWidth h="100%" onClick={onClick}>
        <Title>{text}</Title>
      </Button>
    </AspectRatio>
  );
}

interface PresetData {
  colour: Colour;
  text: string;
  id: string;
}

const mockPresetButtonsData: Array<PresetData> = [
  {
    id: 'one',
    colour: 'RED',
    text: 'Hello',
  },
  {
    id: 'two',
    colour: 'GREEN',
    text: 'Hello',
  },
  {
    id: 'three',
    colour: 'BLUE',
    text: 'Hello',
  },
  {
    id: 'four',
    colour: 'YELLOW',
    text: 'Hello',
  },
  {
    id: 'five',
    colour: 'GREEN',
    text: 'Hello',
  },
  {
    id: 'six',
    colour: 'PURPLE',
    text: 'Hello',
  },
  {
    id: 'seven',
    colour: 'CYAN',
    text: 'Hello',
  },
  {
    id: 'eight',
    colour: 'BLUE',
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
  const [activeButton, setActiveButton] = useState<string>(mockPresetButtonsData[0].id);
  const activeButtonData = mockPresetButtonsData.find((button) => button.id === activeButton);
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [buttonEditorExpanded, setButtonEditorExpanded] = useState<boolean>(true);

  return (
    <AppShell.Main>
      <Stack gap="xs">
        <Title>{presetId}</Title>
        <SimpleGrid cols={4} spacing="md">
          {mockPresetButtonsData.map((button) => (
            <DeviceButton
              key={`button_${button.id}`}
              colour={button.colour}
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
              {activeButtonData !== undefined && (
                <Card withBorder>
                  <Flex justify="space-between">
                    <Title size="h3">Button editor</Title>
                    <ActionIcon
                      variant="subtle"
                      color="neutral"
                      onClick={() => {
                        setButtonEditorExpanded((value) => !value);
                      }}
                    >
                      {buttonEditorExpanded ? <MdOutlineVisibilityOff /> : <MdOutlineVisibility />}
                    </ActionIcon>
                  </Flex>
                  {buttonEditorExpanded && (
                    <ButtonEditForm
                      key={activeButton}
                      initialValues={activeButtonData}
                      onSubmit={(values) =>
                        new Promise<void>((resolve) => {
                          console.log(values);
                          resolve();
                        })
                      }
                    />
                  )}
                </Card>
              )}
              {isEditing && (
                <Card withBorder>
                  <Flex justify="space-between">
                    <Title size="h3">Action editor</Title>
                    <ActionIcon
                      variant="subtle"
                      color="neutral"
                      onClick={() => {
                        setIsEditing(false);
                      }}
                    >
                      <MdClose />
                    </ActionIcon>
                  </Flex>
                  <ButtonActionEditForm
                    onSubmit={(values) =>
                      new Promise<void>((resolve) => {
                        console.log(values);
                        setIsEditing(false);
                        resolve();
                      })
                    }
                  />
                </Card>
              )}
            </Stack>
          </Grid.Col>
          <Grid.Col span={1}>
            <Stack>
              {mockButtonActions.map((action, index) => (
                <Card
                  key={`action_${index.toString()}`}
                  bd={isEditing ? 'solid 1px var(--mantine-primary-color-filled)' : undefined}
                  bg={isEditing ? 'var(--mantine-primary-color-light)' : undefined}
                  withBorder
                >
                  <Flex justify="space-between">
                    <SimpleGrid cols={4} spacing="sm" component="dl" m={0} w="100%">
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
                    <Flex gap="xs" align="center">
                      <ActionIcon
                        variant="default"
                        size="lg"
                        aria-label="Edit"
                        onClick={() => {
                          setIsEditing(true);
                        }}
                      >
                        <MdEdit />
                      </ActionIcon>
                      <ActionIcon variant="filled" size="lg" color="red" aria-label="Delete">
                        <MdDelete />
                      </ActionIcon>
                    </Flex>
                  </Flex>
                </Card>
              ))}
              <Button
                onClick={() => {
                  setIsEditing(true);
                }}
                style={{ width: '100%' }}
                variant="default"
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
