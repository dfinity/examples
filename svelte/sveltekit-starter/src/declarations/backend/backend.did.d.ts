import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
	greet: ActorMethod<[string], string>;
}
