import React from 'react';

const CreateToken = ({ actor, setTokenCreated }) => {
  const [tokenName, setTokenName] = React.useState();
  const [tokenSymbol, setTokenSymbol] = React.useState();
  const [tokenSupply, setTokenSupply] = React.useState();
  const [tokenLogo, setTokenLogo] = React.useState();

  const createToken = async (e) => {
    e.preventDefault();
    if (!tokenName || !tokenSymbol || !tokenSupply || !tokenLogo) {
      alert('Please fill in all fields');
      return;
    }
    const result = await actor.create_token({
      token_name: tokenName,
      token_symbol: tokenSymbol,
      initial_supply: tokenSupply * Number(10 ** 8),
      token_logo: tokenLogo
    });

    if ('Ok' in result) {
      setTokenCreated(true);
    } else if ('Err' in result) {
      console.error('Failed to create token:', result.Err);
    }
  };

  const handleImageChange = (e) => {
    const file = e.target.files[0];

    if (!file) {
      return;
    }
    // Check file size
    if (file.size > 1024 * 1024) {
      alert('File is too large. Please select a file under 1MB.');
      return;
    }
    const reader = new FileReader();
    reader.onload = (e) => {
      setTokenLogo(e.target.result);
    };
    reader.readAsDataURL(file);
  };

  return (
    <div className="mx-auto max-w-2xl p-4">
      <h1 className="text-center text-2xl font-bold">Create a new token</h1>
      <form className="mt-4 space-y-4">
        <div>
          <label className="block" htmlFor="tokenName">
            <b>Token name</b>
          </label>
          <input
            className="w-full rounded border border-gray-300 p-2"
            id="tokenName"
            type="text"
            value={tokenName}
            onChange={(e) => setTokenName(e.target.value)}
          />
        </div>
        <div>
          <label className="block" htmlFor="tokenSymbol">
            <b>Token symbol</b>
          </label>
          <input
            className="w-full rounded border border-gray-300 p-2"
            id="tokenSymbol"
            type="text"
            value={tokenSymbol}
            onChange={(e) => setTokenSymbol(e.target.value)}
          />
        </div>
        <div>
          <label className="block" htmlFor="tokenSupply">
            <b>Initial token supply</b>
          </label>
          <input
            className="w-full rounded border border-gray-300 p-2"
            id="tokenSupply"
            type="number"
            value={tokenSupply}
            onChange={(e) => setTokenSupply(e.target.value)}
          />
        </div>
        <div>
          <label className="block" htmlFor="tokenLogo">
            <b>Token Logo</b>
          </label>
          <input
            className="w-full rounded border border-gray-300 p-2"
            id="tokenLogo"
            type="file"
            accept="image/*"
            onChange={handleImageChange}
            required
          />
          {tokenLogo && <img src={tokenLogo} alt="Token logo preview" className="mt-2 max-w-xs" />}
        </div>
        <p>The principal signed in will be set as the token minter.</p>
        <button className="w-full rounded bg-blue-500 p-2 text-white" type="submit" onClick={createToken}>
          Create token
        </button>
      </form>
    </div>
  );
};

export default CreateToken;
