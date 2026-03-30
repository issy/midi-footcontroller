import { Navigate, Route, Routes } from 'react-router';
import DeviceUpdateView from './views/device/update/DeviceUpdateView';
import DeviceEditorLayout from './views/device/editor/DeviceEditorLayout';
import NotFoundView from './views/NotFoundView';
import AppLayout from '@/views/AppLayout';
import DeviceConnectView from './views/device/DeviceConnectView';
import DeviceLayout from '@/views/device/DeviceLayout';
import DevicePresetView from '@/views/device/editor/DevicePresetView';

function App() {
  return (
    <Routes>
      <Route Component={AppLayout}>
        <Route index element={<Navigate to="device" replace />} />
        <Route path="device" Component={DeviceLayout}>
          <Route index Component={DeviceConnectView} />
          <Route path="editor" Component={DeviceEditorLayout}>
            <Route path=":presetId" Component={DevicePresetView} />
          </Route>
          <Route path="update" Component={DeviceUpdateView} />
        </Route>
        <Route path="*" Component={NotFoundView} />
      </Route>
    </Routes>
  );
}

export default App;
