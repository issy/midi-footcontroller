import { z } from 'zod';
import { Controller, useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { Button, ColorInput, Stack } from '@mantine/core';
import FormField from '@/components/FormField';

const hexToColour = {
  '#fa5252': 'RED',
  '#34d399': 'GREEN',
  '#2563eb': 'BLUE',
  '#fbbf24': 'YELLOW',
  '#f97316': 'ORANGE',
  '#8b5cf6': 'PURPLE',
  '#06b6d4': 'CYAN',
  '#ffffff': 'WHITE',
} as const;

const colourToHex = Object.fromEntries(Object.entries(hexToColour).map(([key, value]) => [value, key])) as Record<
  (typeof hexToColour)[keyof typeof hexToColour],
  keyof typeof hexToColour
>;

const buttonSchema = z.object({
  // TODO: Is this the same length defined in the firmware crate?
  // TODO: Refine this to alphanumeric characters only?
  name: z.string().max(16),
  colour: z.enum(['RED', 'GREEN', 'BLUE', 'YELLOW', 'ORANGE', 'PURPLE', 'CYAN', 'WHITE']),
});

type FormValues = z.infer<typeof buttonSchema>;

// TODO: Provide initial values
interface ButtonEditFormProps {
  initialValues: FormValues;
  onSubmit: (values: FormValues) => Promise<void>;
}

function ButtonEditForm({ initialValues, onSubmit }: ButtonEditFormProps) {
  const {
    control,
    handleSubmit,
    formState: { isSubmitting, isDirty },
  } = useForm({
    resolver: zodResolver(buttonSchema),
    defaultValues: initialValues,
  });

  return (
    <form
      onSubmit={(e) => {
        void handleSubmit(onSubmit)(e);
      }}
    >
      <Stack gap="md" p="xs">
        <FormField
          control={control}
          projection={{
            fieldName: 'name',
            type: 'text',
            label: 'Name',
          }}
        />
        <Controller
          name="colour"
          control={control}
          render={({ field, fieldState }) => {
            const error = fieldState.error?.message;
            const valueAsHex = colourToHex[field.value];
            return (
              <ColorInput
                value={valueAsHex}
                onChange={(hex) => {
                  field.onChange(hexToColour[hex as keyof typeof hexToColour]);
                }}
                swatches={Object.keys(hexToColour)}
                error={error}
                format="hex"
                withEyeDropper={false}
                withPicker={false}
              />
            );
          }}
        />
        <Button type="submit" disabled={!isDirty} loading={isSubmitting}>
          Update
        </Button>
      </Stack>
    </form>
  );
}

export default ButtonEditForm;
