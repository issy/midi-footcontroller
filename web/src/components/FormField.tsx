import { Checkbox, NumberInput, Select, TextInput } from '@mantine/core';
import { type Control, Controller, type FieldPath, type FieldValues } from 'react-hook-form';

interface SelectOption {
  label: string;
  value: string;
}

type FieldType = 'select' | 'text' | 'number' | 'checkbox';

interface BaseFieldProjection<
  TFieldType extends FieldType,
  TFieldValues extends FieldValues,
  TFieldName extends FieldPath<TFieldValues>,
> {
  fieldName: TFieldName;
  type: TFieldType;
  label: string;
  /**
   * @default {false}
   */
  required?: boolean;
  placeholder?: string;
}

interface SelectFieldProjection<
  TFieldValues extends FieldValues,
  TFieldName extends FieldPath<TFieldValues>,
> extends BaseFieldProjection<'select', TFieldValues, TFieldName> {
  options: Array<SelectOption>;
  /**
   * @default {false}
   */
  clearable?: boolean;
}

type TextFieldProjection<TFieldValues extends FieldValues, TFieldName extends FieldPath<TFieldValues>> = BaseFieldProjection<
  'text',
  TFieldValues,
  TFieldName
>;

type NumberFieldProjection<TFieldValues extends FieldValues, TFieldName extends FieldPath<TFieldValues>> = BaseFieldProjection<
  'number',
  TFieldValues,
  TFieldName
>;

type CheckboxFieldProjection<TFieldValues extends FieldValues, TFieldName extends FieldPath<TFieldValues>> = Omit<
  BaseFieldProjection<'checkbox', TFieldValues, TFieldName>,
  'placeholder'
>;

type FieldProjection<TFieldValues extends FieldValues, TFieldName extends FieldPath<TFieldValues>> =
  | SelectFieldProjection<TFieldValues, TFieldName>
  | TextFieldProjection<TFieldValues, TFieldName>
  | NumberFieldProjection<TFieldValues, TFieldName>
  | CheckboxFieldProjection<TFieldValues, TFieldName>;

export type FormProjection<TFieldValues extends FieldValues> = {
  [K in FieldPath<TFieldValues>]?: FieldProjection<TFieldValues, K>;
};

interface FormFieldProps<TFieldValues extends FieldValues, TFieldName extends FieldPath<TFieldValues>> {
  control: Control<TFieldValues>;
  projection: FieldProjection<TFieldValues, TFieldName>;
}

function FormField<TFieldValues extends FieldValues, TFieldName extends FieldPath<TFieldValues>>({
  control,
  projection,
}: FormFieldProps<TFieldValues, TFieldName>) {
  const { label, fieldName } = projection;

  return (
    <Controller
      name={fieldName}
      control={control}
      render={({ field, fieldState }) => {
        const error = fieldState.error?.message;

        switch (projection.type) {
          case 'select': {
            const { options, clearable = false, placeholder } = projection;
            return (
              <Select
                label={label}
                error={error}
                placeholder={placeholder}
                data={options}
                value={field.value}
                onChange={field.onChange}
                clearable={clearable}
                autoComplete="off"
                labelProps={{ mb: 'xs' }}
                withAlignedLabels
              />
            );
          }
          case 'number': {
            const { placeholder } = projection;
            return (
              <NumberInput
                label={label}
                error={error}
                placeholder={placeholder}
                value={field.value}
                allowNegative={false}
                clampBehavior="strict"
                step={1}
                autoComplete="off"
                onValueChange={(e) => {
                  field.onChange(e.floatValue);
                }}
                labelProps={{ mb: 'xs' }}
              />
            );
          }
          case 'text': {
            const { placeholder } = projection;
            return (
              <TextInput
                label={label}
                error={error}
                placeholder={placeholder}
                value={field.value}
                onChange={(e) => {
                  field.onChange(e.currentTarget.value);
                }}
                autoComplete="off"
                labelProps={{ mb: 'xs' }}
              />
            );
          }
          case 'checkbox': {
            return (
              <Checkbox
                label={label}
                error={error}
                checked={field.value}
                onChange={(event) => {
                  field.onChange(event.currentTarget.checked);
                }}
              />
            );
          }
        }
      }}
    />
  );
}

export default FormField;
