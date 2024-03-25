const secondsToTime = (seconds: number): string => {
  const options: Intl.DateTimeFormatOptions = {
    timeStyle: 'short',
  };
  const milliseconds = seconds * 1000;
  // We only support english for now.
  return new Date(milliseconds).toLocaleTimeString('en', options);
};

const secondsToDate = (seconds: number): string => {
  const options: Intl.DateTimeFormatOptions = {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  };
  const milliseconds = seconds * 1000;
  // We only support english for now.
  return new Date(milliseconds).toLocaleDateString('en', options);
};

export const nanoSecondsToDateTime = (nanoSeconds: bigint): string => {
  const seconds = Number(nanoSeconds / BigInt(1e9));
  return `${secondsToDate(seconds)} ${secondsToTime(seconds)}`;
};
