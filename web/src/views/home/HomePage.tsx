import classes from './HomePage.module.scss';
import { Button, Container, Flex, Stack, Text } from '@mantine/core';
import { FaGithub } from 'react-icons/fa6';
import { MdOpenInNew } from 'react-icons/md';

export function HomePage() {
  return (
    <div className={classes.container}>
      <Container size="xl">
        <Flex justify="space-between" gap="lg">
          <Stack className={classes.heroSection}>
            <h1 className={classes.title}>MIDI-X</h1>
            <Text c="dimmed" size="xl">
              A completely configurable MIDI controller built for the stage
            </Text>
            <Flex gap="md">
              <Button size="lg" variant="gradient" gradient={{ from: 'indigo', to: 'blueviolet', deg: 20 }}>
                Learn more
              </Button>
              <Button
                size="lg"
                variant="light"
                color="gray"
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
  );
}

export default HomePage;
