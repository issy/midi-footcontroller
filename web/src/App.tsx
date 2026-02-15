import { Route, Routes } from 'react-router';
import HomeView from './views/home/HomeView';
import DeviceLayout from './views/device/DeviceLayout';
import DeviceUpdateView from './views/device/update/DeviceUpdateView';
import DeviceEditorView from './views/device/editor/DeviceEditorView';

function App() {
  return (
    <Routes>
      <Route index Component={HomeView} />
      <Route path="device" Component={DeviceLayout}>
        <Route path="update" Component={DeviceUpdateView} />
        <Route path="editor" Component={DeviceEditorView} />
      </Route>
    </Routes>
  );
}

export default App;
