import classes from './HomePage.module.scss';
import { Container, Flex, Title } from '@mantine/core';

export function HomePage() {
  return (
    <div className={classes.container}>
      <Container size="xl">
        <Flex justify="space-between" gap="lg">
          <div className={classes.heroSection}>
            <Title className={classes.title}>MIDI-X</Title>
          </div>
        </Flex>
      </Container>
    </div>
  );
}

export default HomePage;
