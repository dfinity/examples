import { fileupload } from "../../declarations/fileupload";

let file;

const uploadChunk = async ({filename,chunk_index, chunk}) => fileupload.create_chunk({
  filename,
  chunk_index,
  chunk: [...new Uint8Array(await chunk.arrayBuffer())]
})

const upload = async () => {
  
  if (!file) {
    alert('No file selected');
    return;
  }

  console.log('start upload');

  const filename = file.name;
  const promises = [];
  const chunkSize = 500000;
  let chunk_index =0;
  for (let start = 0; start < file.size; start += chunkSize) {
    const chunk = file.slice(start, start + chunkSize);

    promises.push(uploadChunk({
      filename,
      chunk_index,
      chunk
    }));
    chunk_index=chunk_index+1;
  }

  const chunkIds = await Promise.all(promises);

  console.log("chunkIDs: ",chunkIds);


  
  let img = await fileupload.commit_batch(
    filename,
    chunkIds,
     file.type
  )

  console.log('uploaded : ' + img);

  loadImage(img);
}

const loadImage = (filename) => {

  if (!filename) {
    return;
  }
  
  const newImage = document.createElement('img');
  newImage.src = `http://localhost:8000/assets/${filename}?canisterId=rrkah-fqaaa-aaaaa-aaaaq-cai`;

  const img = document.querySelector('section:last-of-type img');
  img?.parentElement.removeChild(img);

  const section = document.querySelector('section:last-of-type');
  section?.appendChild(newImage);
}

const input = document.querySelector('input');
input?.addEventListener('change', ($event) => {
  file = $event.target.files?.[0];
});

const btnUpload = document.querySelector('button.upload');
btnUpload?.addEventListener('click', upload);