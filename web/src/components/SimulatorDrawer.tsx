import init from '../../simulator-pkg/simulator';
import { useMutation } from '@tanstack/react-query';
import { ActionIcon, Affix, Alert, Button, Flex } from '@mantine/core';
import { Fragment, useState } from 'react';
import classes from './Simulator.module.scss';
import { MdClose } from 'react-icons/md';

async function initialiseSimulator() {
  const { main } = await init();
  main();
}

function SimulatorDrawer() {
  const [opened, setOpened] = useState<boolean>(false);
  const { mutate, isPending, isError, error, isSuccess } = useMutation({
    mutationKey: ['simulator-initialised'],
    mutationFn: initialiseSimulator,
  });

  return (
    <Fragment>
      <Affix position={{ bottom: 'md', right: 'md' }} zIndex={1}>
        <Button
          onClick={() => {
            setOpened((curr) => !curr);
            if (!isSuccess) mutate();
          }}
          loading={isPending}
        >
          Simulator
        </Button>
      </Affix>
      <div style={{ display: opened ? 'block' : 'none' }} className={classes.simulatorDrawer}>
        <Flex mb="md" direction="row-reverse">
          <ActionIcon
            onClick={() => {
              setOpened(false);
            }}
            variant="subtle"
            color="gray"
          >
            <MdClose />
          </ActionIcon>
        </Flex>
        {isError && (
          <Alert color="red" title="Error">
            {error.message}
          </Alert>
        )}
        <div id="simulator-root" />
      </div>
    </Fragment>
  );
}

export default SimulatorDrawer;
