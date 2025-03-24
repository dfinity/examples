killall dfx replica
set -e
git submodule update --init --recursive
rm -rf .dfx/ ./internet-identity/.dfx/
dfx start --background --clean --host 127.0.0.1:8000
II_FETCH_ROOT_KEY=1 dfx deploy internet_identity --no-wallet --argument '(null)'
npm install
./deploy.sh --no-wallet --argument null
