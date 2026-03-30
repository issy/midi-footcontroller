import { ActionIcon, Group, useComputedColorScheme, useMantineColorScheme } from '@mantine/core';
import { MdDarkMode, MdLightMode } from 'react-icons/md';

function ColourSchemeButton() {
  const { setColorScheme } = useMantineColorScheme();
  const computedColorScheme = useComputedColorScheme(undefined, { getInitialValueInEffect: true });

  return (
    <Group justify="center">
      <ActionIcon
        onClick={() => {
          setColorScheme(computedColorScheme === 'light' ? 'dark' : 'light');
        }}
        variant="default"
        size="xl"
        radius="md"
        aria-label="Toggle color scheme"
      >
        {computedColorScheme === 'light' && <MdDarkMode />}
        {computedColorScheme === 'dark' && <MdLightMode />}
      </ActionIcon>
    </Group>
  );
}

export default ColourSchemeButton;
