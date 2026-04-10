import { useForm, useWatch } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import FormField from '@/components/FormField';
import { Button, Stack } from '@mantine/core';
import { Fragment } from 'react';

// TODO: Should these be zero-indexed?
const midiChannelNumberSchema = z.int().min(1).max(16);
const midiValueNumberSchema = z.int().min(1).max(128);

const programChangeSchema = z.object({
  type: z.literal('PROGRAM_CHANGE'),
  channel: midiChannelNumberSchema,
  program: midiValueNumberSchema,
});

const controlChangeSchema = z.object({
  type: z.literal('CONTROL_CHANGE'),
  channel: midiChannelNumberSchema,
  control: midiValueNumberSchema,
  value: midiValueNumberSchema,
});

const noteOnSchema = z.object({
  type: z.literal('NOTE_ON'),
  channel: midiChannelNumberSchema,
  note: z.number(),
  velocity: midiValueNumberSchema,
});

const noteOffSchema = z.object({
  type: z.literal('NOTE_OFF'),
  channel: midiChannelNumberSchema,
  note: z.number(),
});

const buttonActionSchema = () =>
  z.discriminatedUnion('type', [programChangeSchema, controlChangeSchema, noteOnSchema, noteOffSchema]);
const schema = buttonActionSchema();

type FormValues = z.infer<ReturnType<typeof buttonActionSchema>>;

const defaultValues: Partial<FormValues> = {
  channel: 1,
  type: 'PROGRAM_CHANGE',
};

function ButtonActionEditForm({
  initialValues,
  onSubmit,
}: {
  initialValues?: FormValues;
  onSubmit: (values: FormValues) => Promise<void>;
}) {
  const { handleSubmit, control } = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: initialValues ?? defaultValues,
  });
  const actionType = useWatch({ control, name: 'type' });

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
            fieldName: 'channel',
            type: 'number',
            label: 'Channel',
            placeholder: 'Select a MIDI channel (1-16)',
          }}
        />
        <FormField
          control={control}
          projection={{
            fieldName: 'type',
            type: 'select',
            label: 'Type',
            options: [
              { value: 'PROGRAM_CHANGE', label: 'Program Change Message' },
              { value: 'CONTROL_CHANGE', label: 'CC Message' },
              { value: 'NOTE_ON', label: 'Note On' },
              { value: 'NOTE_OFF', label: 'Note Off' },
            ],
          }}
        />
        {actionType === 'PROGRAM_CHANGE' && (
          <Fragment>
            <FormField
              control={control}
              projection={{
                fieldName: 'program',
                type: 'number',
                label: 'Program',
              }}
            />
          </Fragment>
        )}
        {actionType === 'CONTROL_CHANGE' && (
          <Fragment>
            <FormField
              control={control}
              projection={{
                fieldName: 'control',
                type: 'number',
                label: 'Control',
              }}
            />
            <FormField
              control={control}
              projection={{
                fieldName: 'value',
                type: 'number',
                label: 'Value',
              }}
            />
          </Fragment>
        )}
        {actionType === 'NOTE_ON' ||
          (actionType === 'NOTE_OFF' && (
            <FormField control={control} projection={{ fieldName: 'note', type: 'number', label: 'Note' }} />
          ))}
        {actionType === 'NOTE_ON' && (
          <FormField
            control={control}
            projection={{
              fieldName: 'velocity',
              type: 'number',
              label: 'Velocity',
            }}
          />
        )}
        <Button type="submit">{initialValues === undefined ? 'Create' : 'Update'}</Button>
      </Stack>
    </form>
  );
}

export default ButtonActionEditForm;
