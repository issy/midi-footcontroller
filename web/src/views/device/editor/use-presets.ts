import { useQuery } from '@tanstack/react-query';

interface PresetData {
  id: string;
  name: string;
}

const mockPresets: ReadonlyArray<PresetData> = [
  {
    id: 'foo',
    name: 'Foo',
  },
  {
    id: 'bar',
    name: 'Bar',
  },
  {
    id: 'baz',
    name: 'Baz',
  },
];

function getPresets() {
  return mockPresets;
}

function usePresets() {
  return useQuery({
    queryKey: ['presets'],
    queryFn: getPresets,
  });
}

export default usePresets;
