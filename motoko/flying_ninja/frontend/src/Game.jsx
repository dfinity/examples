import React, { useEffect, useState } from 'react';
import Ninja from './Ninja';
import Pipes from './Pipes';
import Score from './Score';
import Leaderboard from './Leaderboard';
import { backend } from 'declarations/backend';
import '../index.css';

class SeededRNG {
  state0;
  state1;

  constructor(seed) {
    const view = new DataView(seed.buffer);
    this.state0 = view.getUint32(0, true);
    this.state1 = view.getUint32(4, true);
  }

  next() {
    let s1 = this.state0;
    const s0 = this.state1;
    this.state0 = s0;
    s1 ^= s1 << 23;
    s1 ^= s1 >> 17;
    s1 ^= s0;
    s1 ^= s0 >> 26;
    this.state1 = s1;
    return (s0 + s1) / 4294967296; // This already returns a number between 0 and 1
  }
}

const Game = () => {
  const gravity = 1;
  const jumpHeight = -10;
  const ninjaStartY = 200;
  const pipeStartX = 400;
  const gapHeight = 200;

  const [rng, setRng] = useState(null);
  const [leaderboard, setLeaderboard] = useState([]);

  // Add this function to fetch the leaderboard
  const fetchLeaderboard = async () => {
    try {
      const entries = await backend.getLeaderboard();
      setLeaderboard(entries);
    } catch (error) {
      console.error('Failed to fetch leaderboard:', error);
    }
  };

  // this is run once when the page is loaded
  useEffect(() => {
    const initialize = async () => {
      try {
        // fetch the leaderboard
        await fetchLeaderboard();
        // initialize the seed with randomness from the internet computer
        const randomness = await backend.getRandomness();
        const seed = new Uint8Array(randomness);

        // create a new SeededRNG with the first 8 bytes of the seed
        setRng(new SeededRNG(seed.slice(0, 8)));
      } catch (error) {
        console.error('Failed to initialize seed:', error);
      }
    };

    initialize();
  }, []);

  const [gameState, setGameState] = useState('initial');
  const [ninjaY, setNinjaY] = useState(ninjaStartY);
  const [ninjaVelocity, setNinjaVelocity] = useState(0);
  const [pipeX, setPipeX] = useState(pipeStartX);
  const [gapPosition, setGapPosition] = useState(100);
  const [score, setScore] = useState(0);
  const [showNameInput, setShowNameInput] = useState(false);
  const [playerName, setPlayerName] = useState('');

  const startGame = () => {
    setGameState('playing');
    resetGame();
  };

  const resetGame = () => {
    setNinjaY(ninjaStartY);
    setNinjaVelocity(0);
    setPipeX(pipeStartX);
    setGapPosition(100);
    setScore(0);
    setGameState('playing');
  };

  // Handle gravity and ninja movement
  useEffect(() => {
    const handleInteraction = () => {
      if (gameState === 'playing') {
        setNinjaVelocity(jumpHeight);
      }
    };

    window.addEventListener('keypress', (event) => {
      if (event.key === ' ' && gameState === 'playing') {
        setNinjaVelocity(jumpHeight);
      }
    });

    window.addEventListener('touchstart', handleInteraction);
    window.addEventListener('mousedown', handleInteraction);

    return () => {
      window.removeEventListener('keypress', handleInteraction);
      window.removeEventListener('touchstart', handleInteraction);
      window.removeEventListener('mousedown', handleInteraction);
    };
  }, [gameState]);

  useEffect(() => {
    let gameLoop;

    if (gameState === 'playing' && rng) {
      gameLoop = setInterval(() => {
        setNinjaY((prevY) => Math.min(prevY + ninjaVelocity, window.innerHeight - 10));
        setNinjaVelocity((prevVelocity) => prevVelocity + gravity);

        setPipeX((prevX) => {
          if (prevX < -5) {
            // rng.next() already returns a number between 0 and 1
            setGapPosition(rng.next() * (window.innerHeight - gapHeight));
            setScore((prevScore) => prevScore + 1);
            return window.innerWidth + 5;
          }
          return prevX - 5;
        });

        // Collision detection
        if (
          ninjaY < 0 ||
          ninjaY + 30 >= window.innerHeight ||
          (pipeX < 130 && pipeX > 80 && (ninjaY < gapPosition || ninjaY > gapPosition + gapHeight))
        ) {
          setGameState('gameOver');
          checkHighScore();
        }
      }, 30);
    }

    return () => clearInterval(gameLoop);
  }, [ninjaY, ninjaVelocity, pipeX, gapPosition, gameState, rng]);

  const checkHighScore = async () => {
    const isHighScore = await backend.isHighScore(BigInt(score));
    console.log('isHighScore', isHighScore);
    if (isHighScore) {
      setShowNameInput(true);
    }
  };

  const submitScore = async () => {
    if (playerName.trim() !== '') {
      await backend.addLeaderboardEntry(playerName, BigInt(score));
      setShowNameInput(false);
      setGameState('gameOver');
      await fetchLeaderboard(); // update the leaderboard
    }
  };

  return (
    <div style={{ position: 'relative', minHeight: '100dvh', height: '100%', width: '100%' }}>
      {gameState === 'initial' && (
        <div
          style={{
            position: 'absolute',
            top: '50%',
            left: '50%',
            transform: 'translate(-50%, -50%)',
            textAlign: 'center'
          }}
        >
          <h1>Flying Ninja</h1>
          <p>Press the spacebar, click your mouse, or touch the screen on mobile to make the ninja jump.</p>
          <p>Avoid the pipes and try to get the highest score!</p>
          <button
            onClick={startGame}
            style={{
              fontSize: '24px',
              padding: '10px 20px',
              cursor: 'pointer',
              marginBottom: '20px'
            }}
          >
            Start Game
          </button>
          <Leaderboard entries={leaderboard} />
        </div>
      )}
      {gameState === 'playing' && (
        <>
          <div
            style={{
              backgroundImage: `url("/background.jpg")`,
              backgroundSize: 'cover',
              height: '100dvh',
              width: '100%',
              overflow: 'hidden',
              position: 'relative'
            }}
          >
            <Ninja ninjaY={ninjaY} />
            <Pipes pipeX={pipeX} gapHeight={gapHeight} gapPosition={gapPosition} />
          </div>
          <div style={{ position: 'absolute', top: 0, right: '150px' }}>
            <Score score={score} color={'white'} />
          </div>
        </>
      )}

      {gameState !== 'playing' && (
        <div style={{ position: 'absolute', top: 0, right: '150px' }}>
          <Score score={score} color={'black'} />
        </div>
      )}

      {gameState === 'gameOver' && (
        <div
          style={{
            position: 'absolute',
            top: '50%',
            left: '50%',
            transform: 'translate(-50%, -50%)',
            textAlign: 'center'
          }}
        >
          <div style={{ fontSize: '50px', marginBottom: '20px' }}>
            <h2>Game Over</h2>
          </div>
          <button
            onClick={resetGame}
            style={{
              fontSize: '24px',
              padding: '10px 20px',
              cursor: 'pointer',
              marginBottom: '20px'
            }}
          >
            Try Again
          </button>
          <Leaderboard entries={leaderboard} />
          {showNameInput && (
            <div
              style={{
                marginTop: '20px',
                padding: '20px',
                border: 'black',
                borderRadius: '10px'
              }}
            >
              <h2>New High Score!</h2>
              <p>Enter your name:</p>
              <input
                type="text"
                value={playerName}
                onChange={(e) => setPlayerName(e.target.value)}
                style={{ marginBottom: '10px' }}
              />
              <button onClick={submitScore}>Submit</button>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default Game;
