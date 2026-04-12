import { z } from 'zod';
import type { MantineColor } from '@mantine/core';
import { Colour as ProtocolColour } from '@/generated/proto/device/v1/device_pb';

export const colourSchema = z.enum(['RED', 'GREEN', 'BLUE', 'YELLOW', 'ORANGE', 'PURPLE', 'CYAN', 'WHITE']);

export type Colour = z.infer<typeof colourSchema>;

// TODO: Pick different hex values
export const colourToHex = {
  RED: '#fa5252',
  GREEN: '#34d399',
  BLUE: '#2563eb',
  YELLOW: '#fbbf24',
  ORANGE: '#f97316',
  PURPLE: '#8b5cf6',
  CYAN: '#06b6d4',
  WHITE: '#ffffff',
} as const satisfies { [key in Colour]: `#${string}` };

export const hexToColour = Object.fromEntries(Object.entries(colourToHex).map(([key, value]) => [value, key]));

export const colourToMantineColour = {
  RED: 'red',
  GREEN: 'green',
  BLUE: 'blue',
  YELLOW: 'yellow',
  ORANGE: 'orange',
  PURPLE: 'violet',
  CYAN: 'cyan',
  WHITE: 'gray',
} as const satisfies { [Key in Colour]: MantineColor };

export const colourToProtocolColour = {
  RED: ProtocolColour.RED,
  GREEN: ProtocolColour.GREEN,
  BLUE: ProtocolColour.BLUE,
  YELLOW: ProtocolColour.YELLOW,
  ORANGE: ProtocolColour.ORANGE,
  PURPLE: ProtocolColour.PURPLE,
  CYAN: ProtocolColour.CYAN,
  WHITE: ProtocolColour.WHITE,
} as const satisfies { [Key in Colour]: ProtocolColour };
