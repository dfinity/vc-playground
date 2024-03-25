import type { Identity } from '@dfinity/agent';
import type { ImagesList, ImageData } from '../../declarations/rp/rp.did';

const images: ImageData[] = [
  'https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
  'https://images.unsplash.com/photo-1597077962467-be16edcc6a43?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2MzZ8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
  'https://images.unsplash.com/photo-1553184570-557b84a3a308?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2NTF8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
  'https://images.unsplash.com/photo-1509130446053-899ae7358ce6?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2NjF8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
  'https://images.unsplash.com/photo-1620005839871-7ac4aed5ddbc?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2NzN8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
  'https://images.unsplash.com/photo-1597531072931-8fceba101e4e?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY2OTB8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
  'https://images.unsplash.com/photo-1510111652602-195fc654aa83?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY0Nzl8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
  'https://images.unsplash.com/photo-1612145342709-eadb6e22acca?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY3MDh8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
  'https://images.unsplash.com/photo-1597077917598-97ca3922a317?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzY3MjF8&amp;ixlib=rb-4.0.3&amp;w=300&amp;h=300&amp;auto=format&amp;fit=crop',
].map((url) => ({ url }));

// TODO: Use call to actor method to get images
/* eslint-disable-next-line */
export const queryImages = async ({ identity }: { identity: Identity }): Promise<ImagesList> => {
  return {
    images,
  };
  // const actor = await getRpCanister(identity);
  // await actor.list_images({ group_name_substring: [] });
  // if ('Ok' in response) {
  //   return response.Ok;
  // }
  // throw response.Err;
};
