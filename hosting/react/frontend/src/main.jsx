import React from 'react';
import ReactDOM from 'react-dom/client';
import ninjaLogo from '/ninja-logo.svg';
import '../index.css';

function App() {
  return (
    <div className="flex h-screen items-center justify-center bg-purple-600">
      <img src={ninjaLogo} alt="Ninja Logo" className="h-80 w-80" />
    </div>
  );
}

export default App;

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
