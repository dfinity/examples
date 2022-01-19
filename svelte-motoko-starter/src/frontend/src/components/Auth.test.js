import '@testing-library/jest-dom';
import { render, fireEvent } from '@testing-library/svelte';
import { tick } from 'svelte';

jest.mock('@dfinity/agent');

jest.mock('../lib/actor', () => ({
  createActor: jest.fn(() => ({
    whoami: () =>
      Promise.resolve({
        toString: function () {
          return 'test_principal';
        },
        isAnonymous: function () {
          return true;
        },
      }),
  })),
}));
jest.mock('@dfinity/auth-client');

import Auth from '../components/Auth';
import { AuthClient } from '@dfinity/auth-client';

test('can render anonymous visitor', async () => {
  AuthClient.create.mockImplementation(
    jest.fn(() =>
      Promise.resolve({
        isAuthenticated: jest.fn(() => false),
      })
    )
  );

  const { getByText } = render(Auth);
  expect(getByText('Querying caller identity...')).toBeInTheDocument();
  await tick();
  expect(
    getByText('Your principal ID is', { exact: false })
  ).toBeInTheDocument();
  expect(getByText('test_principal')).toBeInTheDocument();
  expect(getByText('(anonymous)', { exact: false })).toBeInTheDocument();
});

test('can sign in', async () => {
  let loggedIn = false;
  AuthClient.create.mockImplementation(
    jest.fn(() =>
      Promise.resolve({
        isAuthenticated: jest.fn(() => loggedIn),
        getIdentity: jest.fn(),
        login: ({ onSuccess }) => {
          loggedIn = true;
          onSuccess();
        },
      })
    )
  );

  const { getByText } = render(Auth);
  await tick();

  fireEvent.click(getByText('Authenticate in with Internet Identity'));
  await tick();
  expect(getByText('Querying caller identity...')).toBeInTheDocument();
  await tick();
  expect(getByText('Log out')).toBeInTheDocument();
});
