import classes from './HomePage.module.scss';
import { Container, Flex, Text } from '@mantine/core';

export function HomePage() {
  return (
    <div className={classes.container}>
      <Container size="xl">
        <Flex justify="space-between" gap="lg">
          <div className={classes.heroSection}>
            <h1 className={classes.title}>MIDI-X</h1>
            <Text c="dimmed" size="xl">
              A completely configurable MIDI controller built for the stage
            </Text>
          </div>
        </Flex>
      </Container>
    </div>
  );
}

export default HomePage;
