# Flying Ninja

Flying Ninja is a 2D side-scroller game where players interact with the flying ninja character using their keyboard's space bar to move up and down. The goal is to avoid the obstacles and obtain points for each obstacle you dodge. When the game ends, the user can add their score to the leaderboard.

The game's logic is written in [Motoko](https://internetcomputer.org/docs/motoko/main/getting-started/motoko-introduction), a programming language designed specifically for developing canisters on ICP.

## Project structure

The `/backend` folder contains the Motoko canister, `app.mo`. The `/frontend` folder contains web assets for the application's user interface. The user interface is written using the React framework. Edit the `mops.toml` file to add [Motoko dependencies](https://mops.one/) to the project.

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Deploy" in the upper right corner.
To open this project in ICP Ninja, click [here](https://icp.ninja/i?url=https://github.com/dfinity/examples/tree/master/motoko/flying_ninja).

To **download** or **reset** the project files, click the menu option next to the deploy button.

## Editing files

To make adjustments to this project, you can edit any file that is unlocked. Then, redeploy your application to view your changes.

To edit files that are immutable in ICP Ninja, you can export the project to GitHub or download the project to your local environment using the "Download files" option.

## Build and deploy from the command-line

To migrate your ICP Ninja project off of the web browser and develop it locally, follow these steps. These steps are necessary if you want to deploy this project for long-term, production use on the mainnet.

### 1. Download your project from ICP Ninja using the 'Download files' button on the upper left corner under the pink ninja star icon.

### 2. Open the `BUILD.md` file for further instructions.
