import type { EncryptedNote } from '../lib/backend';
import type { CryptoService } from './crypto';

export interface NoteModel {
  id: bigint;
  title: string;
  content: string;
  createdAt: number;
  updatedAt: number;
  tags: Array<string>;
}

type SerializableNoteModel = Omit<NoteModel, 'id'>;

export function noteFromContent(content: string, tags: string[]): NoteModel {
  const title = extractTitle(content);
  const creationTime = Date.now();

  return {
    id: BigInt(0),
    title,
    content,
    createdAt: creationTime,
    updatedAt: creationTime,
    tags,
  };
}

export async function serialize(
  note: NoteModel,
  cryptoService: CryptoService
): Promise<EncryptedNote> {
  const serializableNote: SerializableNoteModel = {
    title: note.title,
    content: note.content,
    createdAt: note.createdAt,
    updatedAt: note.updatedAt,
    tags: note.tags,
  };
  const encryptedNote = await cryptoService.encrypt(
    JSON.stringify(serializableNote)
  );
  return {
    id: note.id,
    encrypted_text: encryptedNote,
  };
}

export async function deserialize(
  enote: EncryptedNote,
  cryptoService: CryptoService
): Promise<NoteModel> {
  const serializedNote = await cryptoService.decrypt(enote.encrypted_text);
  const deserializedNote: SerializableNoteModel = JSON.parse(serializedNote);
  return {
    id: enote.id,
    ...deserializedNote,
  };
}

export function summarize(note: NoteModel, maxLength = 50) {
  const div = document.createElement('div');
  div.innerHTML = note.content;

  let text = div.innerText;
  const title = extractTitleFromDomEl(div);
  if (title) {
    text = text.replace(title, '');
  }

  return text.slice(0, maxLength) + (text.length > maxLength ? '...' : '');
}

function extractTitleFromDomEl(el: HTMLElement) {
  const title = el.querySelector('h1');
  if (title) {
    return title.innerText;
  }

  const blockElements = el.querySelectorAll(
    'h1,h2,p,li'
  ) as NodeListOf<HTMLElement>;
  for (const el of blockElements) {
    if (el.innerText?.trim().length > 0) {
      return el.innerText.trim();
    }
  }
  return '';
}

export function extractTitle(html: string) {
  const div = document.createElement('div');
  div.innerHTML = html;
  return extractTitleFromDomEl(div);
}
