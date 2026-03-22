import { MdOutlineUsb, MdOutlineUsbOff } from 'react-icons/md';
import { type ConnectionManager, useConnectionManagerContext } from '@/views/device/connection-manager';
import { Button, Text } from '@mantine/core';
import { Fragment } from 'react';
import { useMutation } from '@tanstack/react-query';

const DisconnectedView = ({ connect }: Pick<Extract<ConnectionManager, { isConnected: false }>, 'connect'>) => {
  const { isPending, mutate } = useMutation({
    mutationFn: connect,
    mutationKey: ['connect'],
  });

  return (
    <Fragment>
      <h3 style={{ display: 'flex' }}>
        <MdOutlineUsbOff />
        <Text>Disconnected</Text>
      </h3>
      <Button
        loading={isPending}
        onClick={() => {
          mutate();
        }}
      >
        <MdOutlineUsb />
        <Text>Connect</Text>
      </Button>
    </Fragment>
  );
};

function DeviceConnectView() {
  const conn = useConnectionManagerContext();

  return (
    <div>
      {conn.isConnected ? (
        <div>
          <Text>Connected</Text>
        </div>
      ) : (
        <DisconnectedView connect={conn.connect} />
      )}
    </div>
  );
}

export default DeviceConnectView;
