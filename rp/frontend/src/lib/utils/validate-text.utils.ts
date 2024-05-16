import { NO_EMPTY_MESSAGE, PROFANITY_MESSAGE } from '$lib/constants/messages';
import { isProfane } from 'no-profanity';

// Throws an error if the text is not valid.
export const validateText = (text: string): void => {
  if (isProfane(text)) {
    throw new Error(PROFANITY_MESSAGE);
  }
  if (text.trim() === '') {
    throw new Error(NO_EMPTY_MESSAGE);
  }
};
