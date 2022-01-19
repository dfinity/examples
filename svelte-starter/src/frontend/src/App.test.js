import '@testing-library/jest-dom';
import { render } from '@testing-library/svelte';
import App from './App';

test('can render the app', () => {
  process.env.FRONTEND_CANISTER_ID = 'test';
  const { getByText } = render(App);
  expect(getByText('Frontend canister ID: test')).toBeInTheDocument();
});
