killall dfx replica
set -e
git submodule update --init --recursive
rm -rf .dfx/ ./internet-identity/.dfx/
dfx start --background
II_ENV=development dfx deploy internet_identity --no-wallet --argument '(null)'
npm install
./deploy.sh --no-wallet --argument null
