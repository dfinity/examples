import { useState, useEffect, ChangeEvent } from 'react';
import { safeGetCanisterEnv } from '@icp-sdk/core/agent/canister-env';
import { backend } from './actor';
import './App.css';

interface ImageInfo {
  id: bigint;
  name: string;
  content_type: string;
}


function App() {
  const [images, setImages] = useState<ImageInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [uploading, setUploading] = useState(false);
  const [uploadMessage, setUploadMessage] = useState('');
  const [uploadState, setUploadState] = useState<'idle' | 'uploading' | 'success' | 'error'>('idle');

  useEffect(() => {
    loadImages();
  }, []);

  const loadImages = async () => {
    try {
      setLoading(true);
      const imageList = await backend.list_images();
      setImages(imageList);
    } catch (error) {
      console.error('Error loading images:', error);
      setUploadMessage('Error loading images');
      setUploadState('error');
    } finally {
      setLoading(false);
    }
  };

  const handleFileUpload = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    if (!file.type.startsWith('image/')) {
      setUploadMessage('Please select an image file');
      setUploadState('error');
      return;
    }

    setUploading(true);
    setUploadMessage('Uploading...');
    setUploadState('uploading');

    try {
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);

      const imageId = await backend.upload_image(
        file.name,
        file.type,
        uint8Array
      );

      setUploadMessage(`Image uploaded successfully! ID: ${imageId}`);
      setUploadState('success');
      await loadImages(); // Refresh the image list

      // Clear the file input
      event.target.value = '';
      setTimeout(() => setUploadState('idle'), 3000);
    } catch (error) {
      console.error('Error uploading image:', error);
      setUploadMessage('Error uploading image');
      setUploadState('error');
    } finally {
      setUploading(false);
    }
  };


  return (
    <div className="app">
      <div className="container">
        <h1>Photo Gallery</h1>

        <div className="upload-section">
          <h2>Upload New Image</h2>
          <div className="upload-form">
            <input
              type="file"
              accept="image/*"
              onChange={handleFileUpload}
              disabled={uploading}
              className="file-input"
            />
            {uploadState !== 'idle' && (
              <div className={`status ${uploadState}`}>
                {uploadMessage}
              </div>
            )}
          </div>
        </div>

        <div className="gallery-section">
          <h2>Gallery</h2>
          {loading ? (
            <div className="loading">Loading images...</div>
          ) : images.length === 0 ? (
            <div className="no-images">No images uploaded yet. Upload your first image above!</div>
          ) : (
            <div className="gallery">
              {images.map((image) => (
                <ImageCard key={image.id.toString()} image={image} />
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

interface ImageCardProps {
  image: ImageInfo;
}

function ImageCard({ image }: ImageCardProps) {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);

  // Compute canister ID once — it never changes during the component's lifetime.
  const canisterId = safeGetCanisterEnv()?.["PUBLIC_CANISTER_ID:backend"];

  const getImageUrl = (imageId: bigint): string => {
    // Use HTTP gateway URL to load images directly from the backend canister.
    // The backend serves images via http_request at /image/<id>, with
    // long-lived Cache-Control headers so browsers cache images after first load.
    const origin = window.location.origin;
    const isLocal = origin.includes('localhost') || origin.includes('127.0.0.1');
    if (isLocal) {
      // icp-cli starts the local replica on port 8000 by default. The Vite dev server
      // runs on a different port (5173), so window.location.port would be wrong here.
      return `http://${canisterId}.localhost:8000/image/${imageId}`;
    }
    return `https://${canisterId}.icp.net/image/${imageId}`;
  };

  const handleImageLoad = () => {
    setLoading(false);
  };

  const handleImageError = () => {
    setLoading(false);
    setError(true);
  };

  return (
    <div className="image-card">
      {loading && !error && (
        <div className="image-placeholder">Loading...</div>
      )}
      {error ? (
        <div className="image-placeholder">Failed to load</div>
      ) : (
        <img
          src={getImageUrl(image.id)}
          onLoad={handleImageLoad}
          onError={handleImageError}
          style={{ display: loading ? 'none' : 'block' }}
        />
      )}
      <div className="image-info">
        <div className="image-name">{image.name}</div>
      </div>
    </div>
  );
}

export default App;
