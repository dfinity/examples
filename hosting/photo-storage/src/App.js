import {Ed25519KeyIdentity} from '@dfinity/identity';
import {HttpAgent} from '@dfinity/agent';
import {AssetManager} from '@dfinity/assets';
import {useEffect, useState} from "react";
import Masonry from "react-masonry-component";
import './App.css';

// Hardcoded principal: 535yc-uxytb-gfk7h-tny7p-vjkoe-i4krp-3qmcl-uqfgr-cpgej-yqtjq-rqe
// Should be replaced with authentication method e.g. Internet Identity when deployed on IC
const identity = Ed25519KeyIdentity.generate(new Uint8Array(Array.from({length: 32}).map(() => 0)));
const isLocal = !window.location.host.endsWith('ic0.app');
const agent = new HttpAgent({
    host: isLocal ? 'http://127.0.0.1:8000' : 'https://ic0.app',
    identity,
});
if (isLocal) {
    agent.fetchRootKey();
}

// Canister id can be fetched from URL since frontend in this example is hosted in the same canister as file upload
const canisterId = new URLSearchParams(window.location.search).get('canisterId') ?? /(.*?)(?:\.raw)?\.ic0.app/.exec(window.location.host)?.[1] ?? /(.*)\.localhost/.exec(window.location.host)?.[1];

// Create asset manager instance for above asset canister
const assetManager = new AssetManager({canisterId, agent});

const App = () => {
    const [uploads, setUploads] = useState([]);
    const [progress, setProgress] = useState(1);
    useEffect(() => {
        assetManager.list()
            .then(assets => assets
                .filter(asset => asset.key.startsWith('/uploads/'))
                .sort((a, b) => Number(b.encodings[0].modified - a.encodings[0].modified))
                .map(asset => asset.key)
            )
            .then(setUploads);
    }, []);

    const uploadPhotos = () => {
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = 'image/*';
        input.multiple = true;
        input.onchange = async () => {
            const batch = assetManager.batch();
            const keys = await Promise.all(Array.from(input.files).map(file => batch.store(file, {path: '/uploads'})));
            await batch.commit({onProgress: ({current, total}) => setProgress(current / total)});
            setUploads(prevState => [...keys, ...prevState])
        };
        input.click();
    }

    return (
        <>
            <div className={'App-wrapper'}>
                <Masonry options={{transitionDuration: 0, columnWidth: '.App-image', gutter: 10}}
                         className={'App-masonry'}>
                    <button className={'App-upload'} onClick={uploadPhotos}>Upload photo</button>
                    {uploads.map(upload => <img src={upload} className={'App-image'}
                                                alt={upload.split('/').slice(-1)[0]}/>)}
                </Masonry>
            </div>
            {progress < 1 && <div className={'App-progress'}>{Math.round(progress * 100)}%</div>}
        </>
    );
}

export default App;
