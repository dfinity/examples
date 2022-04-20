killall dfx replica
set -e
git submodule update --init --recursive
rm -rf .dfx/ ./internet-identity/.dfx/
dfx start --background
pushd internet-identity
npm install
II_ENV=development dfx deploy --no-wallet --argument null
popd
npm install
./deploy.sh --no-wallet --argument null
