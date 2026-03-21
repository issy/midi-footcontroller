import { MdOutlineUsbOff } from 'react-icons/md';
import { type ConnectionManager, useConnectionManagerContext } from '@/views/device/connection-manager';
import { Button } from '@mantine/core';
import { Fragment } from 'react';
import { useMutation } from '@tanstack/react-query';

const DisconnectedView = ({ connect }: Pick<Extract<ConnectionManager, { isConnected: false }>, 'connect'>) => {
  const { isPending, mutate } = useMutation({
    mutationFn: connect,
    mutationKey: ['connect'],
  });

  return (
    <Fragment>
      <h3 className="flex">
        <MdOutlineUsbOff />
        Disconnected
      </h3>
      <Button
        loading={isPending}
        onClick={() => {
          mutate();
        }}
      >
        Connect
      </Button>
    </Fragment>
  );
};

function DeviceConnectView() {
  const conn = useConnectionManagerContext();

  return (
    <div className="h-screen w-screen flex items-center justify-center m-auto">
      <div className="flex flex-col items-center gap-4 text-4xl">
        {conn.isConnected ? <Fragment></Fragment> : <DisconnectedView connect={conn.connect} />}
      </div>
    </div>
  );
}

export default DeviceConnectView;
