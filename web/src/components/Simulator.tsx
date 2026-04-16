import init from '../../simulator-pkg/simulator';
import { useMutation } from '@tanstack/react-query';
import { Button } from '@mantine/core';

const initialiseSimulator = async () => {
  const { main } = await init();
  main();
};

function Simulator() {
  const { mutate, isPending, isError, isSuccess, error } = useMutation({
    mutationKey: ['simulator-initialised'],
    mutationFn: initialiseSimulator,
  });

  return (
    <div>
      <Button
        onClick={() => {
          mutate();
        }}
      >
        Start Simulator
      </Button>
      {isPending && <p>Loading simulator...</p>}
      {isError && <p>Error loading simulator: {error.message}</p>}
      {isSuccess && <p>Simulator loaded successfully!</p>}
      <div id="simulator-root" />
    </div>
  );
}

export default Simulator;
