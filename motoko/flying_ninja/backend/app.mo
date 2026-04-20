import Array "mo:core/Array";
import Nat "mo:core/Nat";
import Random "mo:core/Random";

persistent actor FlyingNinja {
  type LeaderboardEntry = {
    name : Text;
    score : Nat;
  };

  private var leaderboard : [LeaderboardEntry] = [];

  // Returns if a certain score is good enough to warrant an entry on the leaderboard.
  public query func isHighScore(score : Nat) : async Bool {
    if (leaderboard.size() < 10) {
      return true;
    };
    // Whenever a new entry is added, the leaderboard is sorted.
    // We can safely assume that the last entry has the lowest score.
    return score > leaderboard[leaderboard.size() - 1].score;
  };

  // Adds a new entry to the leaderboard if the score is good enough.
  public func addLeaderboardEntry(name : Text, score : Nat) : async [LeaderboardEntry] {
    let newEntry : LeaderboardEntry = { name = name; score = score };

    // Add the new entry and sort the leaderboard
    leaderboard := leaderboard.concat([newEntry]).sort(
      func(a : LeaderboardEntry, b : LeaderboardEntry) : { #less; #equal; #greater } { Nat.compare(b.score, a.score) }
    );

    // Keep only the top 10 scores
    if (leaderboard.size() > 10) {
      leaderboard := leaderboard.sliceToArray(0, 10);
    };

    return leaderboard;
  };

  // Returns the current leaderboard.
  public query func getLeaderboard() : async [LeaderboardEntry] {
    return leaderboard;
  };

  // Produces secure randomness as a seed to the game.
  public func getRandomness() : async Blob {
    await Random.blob();
  };
};
