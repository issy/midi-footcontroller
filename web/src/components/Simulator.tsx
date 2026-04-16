import init from '../../simulator-pkg/simulator';
import { useMutation } from '@tanstack/react-query';
import { useEffect } from 'react';

const initialiseSimulator = async () => {
  const { main } = await init();
  main();
};

function Simulator() {
  const { mutate, isPending, isError, isSuccess, error } = useMutation({
    mutationKey: ['simulator-initialised'],
    mutationFn: initialiseSimulator,
  });

  useEffect(() => {
    mutate();
  }, [mutate]);

  return (
    <div>
      {isPending && <p>Loading simulator...</p>}
      {isError && <p>Error loading simulator: {error.message}</p>}
      {isSuccess && <p>Simulator loaded successfully!</p>}
      <div id="simulator-root" />
    </div>
  );
}

export default Simulator;
