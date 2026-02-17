import { Button } from '@headlessui/react';

function DeviceConnectView() {
  return (
    <div>
      <Button
        onClick={() => {
          console.log('connect me');
        }}
      >
        Connect
      </Button>
    </div>
  );
}

export default DeviceConnectView;
