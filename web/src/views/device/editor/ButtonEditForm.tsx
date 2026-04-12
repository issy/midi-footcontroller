import { z } from 'zod';
import { Controller, useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { Button, ColorPicker, Stack } from '@mantine/core';
import FormField from '@/components/FormField';
import { colourSchema, colourToHex, hexToColour } from '@/utils/colourMapping';

const buttonSchema = z.object({
  // TODO: Is this the same length defined in the firmware crate?
  // TODO: Refine this to alphanumeric characters only?
  text: z.string().max(16),
  colour: colourSchema,
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
            fieldName: 'text',
            type: 'text',
            label: 'Name',
          }}
        />
        <Controller
          name="colour"
          control={control}
          render={({ field }) => {
            const valueAsHex = colourToHex[field.value];
            return (
              <ColorPicker
                fullWidth
                value={valueAsHex}
                onChange={(hex) => {
                  field.onChange(hexToColour[hex as keyof typeof hexToColour]);
                }}
                swatchesPerRow={8}
                swatches={Object.keys(hexToColour)}
                format="hex"
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
