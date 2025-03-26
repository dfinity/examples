import React from 'react';

const Card = ({ icon, title, value }) => (
  <div className="border-infinite/20 hover:border-infinite/40 transform rounded-xl border bg-white/80 p-6 shadow-lg backdrop-blur-sm transition-all duration-300 hover:scale-105 hover:shadow-xl">
    <div className="mb-2 flex items-center">
      <span className="bg-infinite/10 mr-3 rounded-full p-2 text-4xl">{icon}</span>
      <span className="text-lg font-semibold tracking-tight text-black">{title}</span>
    </div>
    <div className="text-infinite text-3xl font-extrabold tracking-wide">{Object.values(value)}</div>
  </div>
);

const CardDisplay = ({ cards, loading }) => {
  if (loading) {
    return (
      <div className="flex h-48 items-center justify-center">
        <div className="border-t-3 border-b-3 border-infinite h-12 w-12 animate-spin rounded-full"></div>
      </div>
    );
  }

  return (
    <div className="bg-infinite/30 mb-8 p-8 shadow-lg backdrop-blur-md transition-all duration-500 hover:shadow-xl">
      <h2 className="mb-6 inline-block pb-2 text-3xl font-bold leading-tight tracking-tight text-white">
        Token Information
      </h2>
      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {cards.map((card, index) => (
          <Card key={index} {...card} />
        ))}
      </div>
    </div>
  );
};

export default CardDisplay;
