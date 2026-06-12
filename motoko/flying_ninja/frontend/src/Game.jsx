import React, { useEffect, useRef, useState } from 'react';
import Ninja from './Ninja';
import Pipes from './Pipes';
import Score from './Score';
import Leaderboard from './Leaderboard';
import { backend } from './actor';
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
    return (s0 + s1) / 4294967296;
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

  const fetchLeaderboard = async () => {
    try {
      const entries = await backend.getLeaderboard();
      setLeaderboard(entries);
    } catch (error) {
      console.error('Failed to fetch leaderboard:', error);
    }
  };

  useEffect(() => {
    const initialize = async () => {
      try {
        await fetchLeaderboard();
        const randomness = await backend.getRandomness();
        const seed = new Uint8Array(randomness);
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

  // Refs hold the current values used inside the game loop interval so that
  // the effect does not need to re-run (and re-create the interval) on every
  // frame. Without refs, the dep array would include frame-level state and
  // React 18 StrictMode's double-invocation would register two intervals,
  // causing the score to increment by 2 per gate in development.
  const ninjaYRef = useRef(ninjaStartY);
  const ninjaVelocityRef = useRef(0);
  const pipeXRef = useRef(pipeStartX);
  const gapPositionRef = useRef(100);
  const scoreRef = useRef(0);

  const startGame = () => {
    resetGame();
  };

  const resetGame = () => {
    ninjaYRef.current = ninjaStartY;
    ninjaVelocityRef.current = 0;
    pipeXRef.current = pipeStartX;
    gapPositionRef.current = 100;
    scoreRef.current = 0;
    setNinjaY(ninjaStartY);
    setNinjaVelocity(0);
    setPipeX(pipeStartX);
    setGapPosition(100);
    setScore(0);
    setGameState('playing');
  };

  useEffect(() => {
    const handleInteraction = () => {
      if (gameState === 'playing') {
        ninjaVelocityRef.current = jumpHeight;
        setNinjaVelocity(jumpHeight);
      }
    };

    window.addEventListener('keypress', handleInteraction);
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
        // Update ninja position
        const newY = Math.min(ninjaYRef.current + ninjaVelocityRef.current, window.innerHeight - 10);
        ninjaYRef.current = newY;
        setNinjaY(newY);

        const newVelocity = ninjaVelocityRef.current + gravity;
        ninjaVelocityRef.current = newVelocity;
        setNinjaVelocity(newVelocity);

        // Update pipe position
        let newPipeX = pipeXRef.current - 5;
        if (newPipeX < -5) {
          const newGap = rng.next() * (window.innerHeight - gapHeight);
          gapPositionRef.current = newGap;
          setGapPosition(newGap);
          scoreRef.current += 1;
          setScore(scoreRef.current);
          newPipeX = window.innerWidth + 5;
        }
        pipeXRef.current = newPipeX;
        setPipeX(newPipeX);

        // Collision detection
        if (
          ninjaYRef.current < 0 ||
          ninjaYRef.current + 30 >= window.innerHeight ||
          (pipeXRef.current < 130 &&
            pipeXRef.current > 80 &&
            (ninjaYRef.current < gapPositionRef.current ||
              ninjaYRef.current > gapPositionRef.current + gapHeight))
        ) {
          setGameState('gameOver');
          checkHighScore();
        }
      }, 30);
    }

    return () => clearInterval(gameLoop);
  }, [gameState, rng]);

  const checkHighScore = async () => {
    const isHighScore = await backend.isHighScore(BigInt(scoreRef.current));
    console.log('isHighScore', isHighScore);
    if (isHighScore) {
      setShowNameInput(true);
    }
  };

  const submitScore = async () => {
    if (playerName.trim() !== '') {
      await backend.addLeaderboardEntry(playerName, BigInt(scoreRef.current));
      setShowNameInput(false);
      setGameState('gameOver');
      await fetchLeaderboard();
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
