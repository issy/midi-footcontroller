import classes from './HomePage.module.scss';
import { AppShell, Button, Container, Flex, Group, Stack, Text, useSafeMantineTheme } from '@mantine/core';
import { FaGithub } from 'react-icons/fa6';
import { MdOpenInNew } from 'react-icons/md';
import { Fragment } from 'react';
import ColourSchemeButton from '@/components/ColourSchemeButton';
import { useNavigate } from 'react-router';

export function HomePage() {
  const { defaultGradient } = useSafeMantineTheme();
  const navigate = useNavigate();

  return (
    <Fragment>
      <AppShell.Header>
        <Group h="100%" px="md" justify="end">
          <Group>
            <Button
              onClick={() => {
                void navigate('/device');
              }}
            >
              Open Editor
            </Button>
            <ColourSchemeButton />
          </Group>
        </Group>
      </AppShell.Header>
      <div className={classes.container}>
        <Container size="xl">
          <Flex justify="space-between" gap="lg">
            <Stack className={classes.heroSection}>
              <h1 className={classes.title}>MIDI-X</h1>
              <Text c="dimmed" size="xl">
                A completely configurable MIDI controller built for the stage
              </Text>
              <Flex gap="md">
                <Button size="lg" variant="gradient" gradient={defaultGradient}>
                  Open Editor
                </Button>
                <Button size="lg" variant="light">
                  Learn More
                </Button>
                <Button
                  size="lg"
                  variant="outline"
                  leftSection={<FaGithub size={28} />}
                  rightSection={<MdOpenInNew />}
                  component="a"
                  href="https://github.com/issy/midi-footcontroller"
                  target="_blank"
                >
                  GitHub
                </Button>
              </Flex>
            </Stack>
          </Flex>
        </Container>
      </div>
    </Fragment>
  );
}

export default HomePage;
