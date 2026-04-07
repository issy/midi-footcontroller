import { Navigate } from 'react-router';
import usePresets from '@/views/device/editor/use-presets';

function DeviceEditorPresetRedirect() {
  const { data: presets } = usePresets();

  if (presets === undefined || presets.length === 0) {
    return null;
  }

  return <Navigate to={presets[0].id} replace />;
}

export default DeviceEditorPresetRedirect;
