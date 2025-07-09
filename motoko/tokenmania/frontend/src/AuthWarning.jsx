import React from 'react';

const FullPageAuthWarning = ({ showIdentity }) => {
  return (
    <div className="bg-infinite -my-8 h-screen w-screen px-0 text-white">
      <main className="flex flex-grow items-center justify-center p-4">
        <div className="w-full max-w-md">
          <div className="rounded-md border-l-4 border-white bg-black bg-opacity-30 p-6 shadow-lg">
            <div className="mb-4 flex items-center">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="mr-3 h-8 w-8"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 15v2m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"
                />
              </svg>
              <h2 className="text-xl font-bold">Authentication Required</h2>
            </div>
            <p className="mb-4">Please sign in to access token management features.</p>
          </div>
        </div>
      </main>
    </div>
  );
};

export default FullPageAuthWarning;
