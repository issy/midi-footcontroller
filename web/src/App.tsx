import { Navigate, Route, Routes } from 'react-router';
import DeviceUpdateView from './views/device/update/DeviceUpdateView';
import DeviceEditorView from './views/device/editor/DeviceEditorView';
import NotFoundView from './views/NotFoundView';
import AppLayout from '@/views/AppLayout';
import DeviceConnectView from './views/device/DeviceConnectView';
import DeviceLayout from '@/views/device/DeviceLayout';

function App() {
  return (
    <Routes>
      <Route Component={AppLayout}>
        <Route index element={<Navigate to="device" replace />} />
        <Route path="device" Component={DeviceLayout}>
          <Route index Component={DeviceConnectView} />
          <Route path="editor" Component={DeviceEditorView} />
          <Route path="update" Component={DeviceUpdateView} />
        </Route>
        <Route path="*" Component={NotFoundView} />
      </Route>
    </Routes>
  );
}

export default App;
