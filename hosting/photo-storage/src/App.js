import {Ed25519KeyIdentity} from '@dfinity/identity';
import {HttpAgent} from '@dfinity/agent';
import {AssetManager} from '@dfinity/assets';
import {useEffect, useState} from "react";
import Masonry from "react-masonry-css";
import './App.css';

// Hardcoded principal: 535yc-uxytb-gfk7h-tny7p-vjkoe-i4krp-3qmcl-uqfgr-cpgej-yqtjq-rqe
// Should be replaced with authentication method e.g. Internet Identity when deployed on IC
const identity = Ed25519KeyIdentity.generate(new Uint8Array(Array.from({length: 32}).fill(0)));
const isLocal = !window.location.host.endsWith('ic0.app');
const agent = new HttpAgent({
    host: isLocal ? `http://127.0.0.1:${window.location.port}` : 'https://ic0.app', identity,
});
if (isLocal) {
    agent.fetchRootKey();
}

// Canister id can be fetched from URL since frontend in this example is hosted in the same canister as file upload
const canisterId = new URLSearchParams(window.location.search).get('canisterId') ?? /(.*?)(?:\.raw)?\.ic0.app/.exec(window.location.host)?.[1] ?? /(.*)\.localhost/.exec(window.location.host)?.[1];

// Create asset manager instance for above asset canister
const assetManager = new AssetManager({canisterId, agent});

// Get file name, width and height from key
const detailsFromKey = (key) => {
    const fileName = key.split('/').slice(-1)[0];
    const width = parseInt(fileName.split('.').slice(-3)[0]);
    const height = parseInt(fileName.split('.').slice(-2)[0]);
    return {key, fileName, width, height}
}

// Get file name, width and height from file
const detailsFromFile = async (file) => {
    const src = await new Promise((resolve) => {
        const reader = new FileReader();
        reader.onload = () => resolve(reader.result);
        reader.readAsDataURL(file);
    })
    const [width, height] = await new Promise((resolve) => {
        const img = new Image();
        img.onload = () => resolve([img.naturalWidth, img.naturalHeight]);
        img.src = src;
    })
    const name = file.name.split('.');
    const extension = name.pop();
    const fileName = [name, width, height, extension].join('.');
    return {fileName, width, height}
}

const App = () => {
    const [uploads, setUploads] = useState([]);
    const [progress, setProgress] = useState(null);

    useEffect(() => {
        assetManager.list()
            .then(assets => assets
                .filter(asset => asset.key.startsWith('/uploads/'))
                .sort((a, b) => Number(b.encodings[0].modified - a.encodings[0].modified))
                .map(({key}) => detailsFromKey(key)))
            .then(setUploads);
    }, []);

    const uploadPhotos = () => {
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = 'image/*';
        input.multiple = true;
        input.onchange = async () => {
            setProgress(0);
            try {
                const batch = assetManager.batch();
                const items = await Promise.all(Array.from(input.files).map(async (file) => {
                    const {fileName, width, height} = await detailsFromFile(file);
                    const key = await batch.store(file, {path: '/uploads', fileName});
                    return {key, fileName, width, height};
                }));
                await batch.commit({onProgress: ({current, total}) => setProgress(current / total)});
                setUploads(prevState => [...items, ...prevState])
            } catch (e) {
                if (e.message.includes('Caller is not authorized')) {
                    alert("Caller is not authorized, follow Authorization instructions in README");
                } else {
                    throw e;
                }
            }
            setProgress(null)
        };
        input.click();
    }

    return (
        <div className={'App-wrapper'}>
            <Masonry breakpointCols={{default: 4, 600: 2, 800: 3}} className={'App-masonry'}
                     columnClassName="App-masonry-column">
                <button className={'App-upload'} onClick={uploadPhotos}>📂 Upload photo</button>
                {uploads.map(upload => (
                    <div key={upload.key} className={'App-image'} style={{aspectRatio: upload.width / upload.height}}>
                        <img src={upload.key} alt={upload.fileName} loading={'lazy'}/>
                    </div>))}
            </Masonry>
            {progress !== null && <div className={'App-progress'}>{Math.round(progress * 100)}%</div>}
        </div>
    );
}

export default App;
