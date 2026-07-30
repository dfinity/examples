import {Ed25519KeyIdentity} from '@icp-sdk/core/identity';
import {HttpAgent} from '@icp-sdk/core/agent';
import {safeGetCanisterEnv} from '@icp-sdk/core/agent/canister-env';
import {AssetManager} from '@icp-sdk/canisters/assets';
import {useEffect, useState} from "react";
import Masonry from "react-masonry-css";
import './App.css';

// Hardcoded principal: 535yc-uxytb-gfk7h-tny7p-vjkoe-i4krp-3qmcl-uqfgr-cpgej-yqtjq-rqe
// Should be replaced with authentication method e.g. Internet Identity when deployed on IC
const identity = Ed25519KeyIdentity.generate(new Uint8Array(Array.from({length: 32}).fill(0)));

// The ic_env cookie is set by the asset canister on all HTML responses. It
// contains the replica root key and the PUBLIC_* canister environment
// variables, so the app finds its own canister ID regardless of which URL the
// gateway serves it under (canister-id-based or name-based).
const canisterEnv = safeGetCanisterEnv();
const canisterId = canisterEnv?.["PUBLIC_CANISTER_ID:frontend"] ?? new URLSearchParams(window.location.search).get('canisterId');

if (!canisterId) {
    throw new Error("Canister ID for 'frontend' not found. Run 'icp deploy' first.");
}

const agent = HttpAgent.createSync({
    host: window.location.origin,
    rootKey: canisterEnv?.IC_ROOT_KEY,
    identity,
});

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
                if (e.message.includes('Caller does not have Prepare permission')) {
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
