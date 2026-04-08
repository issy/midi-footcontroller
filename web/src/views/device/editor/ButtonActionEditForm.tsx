import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';

const ProgramChangeSchema = z.object({
  type: z.literal('PROGRAM_CHANGE'),
  channel: z.number(),
  program: z.number(),
});

const ControlChangeSchema = z.object({
  type: z.literal('CONTROL_CHANGE'),
  channel: z.number(),
  control: z.number(),
  value: z.number(),
});

const NoteOnSchema = z.object({
  type: z.literal('NOTE_ON'),
  channel: z.number(),
  note: z.number(),
  velocity: z.number(),
});

const NoteOffSchema = z.object({
  type: z.literal('NOTE_OFF'),
  channel: z.number(),
  note: z.number(),
});

const ButtonActionSchema = () =>
  z.discriminatedUnion('type', [ProgramChangeSchema, ControlChangeSchema, NoteOnSchema, NoteOffSchema]);
const schema = ButtonActionSchema();

type FormValues = z.infer<ReturnType<typeof ButtonActionSchema>>;

function ButtonActionEditForm() {
  useForm<FormValues>({
    resolver: zodResolver(schema),
  });

  return <div>Form will go here</div>;
}

export default ButtonActionEditForm;
