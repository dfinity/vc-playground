export const toNullable = <T>(value: T | undefined): [] | [T] =>
  value === undefined ? [] : [value];
