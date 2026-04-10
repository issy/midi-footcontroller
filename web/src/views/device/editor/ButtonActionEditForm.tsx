import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import FormField from '@/components/FormField';
import { Button } from '@mantine/core';

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

function ButtonActionEditForm() {
  const { handleSubmit, control } = useForm<FormValues>({
    resolver: zodResolver(schema),
    defaultValues: {
      type: 'CONTROL_CHANGE',
    },
  });

  const submitHandler = (data: FormValues) => {
    console.log('submitHandler', data);
  };

  return (
    <form
      onSubmit={(e) => {
        void handleSubmit(submitHandler)(e);
      }}
    >
      <FormField
        control={control}
        projection={{
          fieldName: 'type',
          type: 'select',
          label: 'Type',
          options: [{ value: 'CONTROL_CHANGE', label: 'CC Message' }],
        }}
      />
      <FormField
        control={control}
        projection={{
          fieldName: 'channel',
          type: 'number',
          label: 'Channel',
          placeholder: 'Select a MIDI channel',
        }}
      />
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
      <Button type="submit">Submit</Button>
    </form>
  );
}

export default ButtonActionEditForm;
