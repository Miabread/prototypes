import * as trpcNext from '@trpc/server/adapters/next';
import { appRouter } from '../../../server';
import { createContext } from '../../../server/createContext';

export default trpcNext.createNextApiHandler({
    router: appRouter,
    createContext,
});
