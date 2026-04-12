import { z } from 'zod';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { Button, Stack } from '@mantine/core';
import FormField from '@/components/FormField';

const buttonSchema = z.object({
  // TODO: Is this the same length defined in the firmware crate?
  // TODO: Refine this to alphanumeric characters only?
  name: z.string().max(16),
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
        <Button type="submit" disabled={!isDirty} loading={isSubmitting}>
          Update
        </Button>
      </Stack>
    </form>
  );
}

export default ButtonEditForm;
