const identityUtils = require("./utils/identity");
const { 
  delegatedAdministrator,  
  anonymousActor,
  getRandomActor,
  getRandomPrincipal,
  anonymousPrincipal
} = identityUtils;

const onePrincipal = getRandomPrincipal();

// NOTE the test for allowed creators capacity is with maxCapacity.js and must be run seperately (can take a while)

jest.setTimeout(60000);

beforeEach(async () => {
  // reset the list
  let result = await delegatedAdministrator.get_allowed_creators_list();
  expect(result?.ok?.allowed).toBeTruthy();
  for (let p of result.ok.allowed) {
      await delegatedAdministrator.remove_allowed_creator({ who: p });
  }
  result = await delegatedAdministrator.get_allowed_creators_list();
  if (result?.ok.allowed.length !== 0) {
    throw new Error("could not reset the creators allow list before each creators allowed list functionality tests")
  }
});

describe("test functionality of the allowed creators list", () => {
  it("should allow authorized caller to add a principal to the list", async () => {
      const result = await delegatedAdministrator.add_allowed_creator({ who: onePrincipal });
      expect(result.ok).toStrictEqual({
          message: `Successfully added ${onePrincipal.toString()} to creators allowed list.`,
      });
  });
  it("should return an error if the principal to add is already on the list", async () => {
      let result = await delegatedAdministrator.add_allowed_creator({ who: onePrincipal });
      result = await delegatedAdministrator.add_allowed_creator({ who: onePrincipal });
      expect(result.err).toStrictEqual({
          kind: {
              AlreadyAdded: null,
          },
          message: [`The principal ${onePrincipal.toString()} is already present on the creators allowed list.`],
      });
  });
  it("should not allow unauthorized caller to add a principal to the list", async () => {
      const result = await getRandomActor().add_allowed_creator({ who: onePrincipal });
      expect(result.err).toStrictEqual({
          kind: {
              NotAuthorized: null,
          },
          message: [`You are not authorized to modify the creators allowed list.`],
      });
  });
  it("should not allow authorized caller to add the anonymous principal to the list", async () => {
      const result = await delegatedAdministrator.add_allowed_creator({ who: anonymousPrincipal });
      expect(result.err).toStrictEqual({
          kind: {
              AnonymousIneligible: null,
          },
          message: [`The anonymous caller is not elgible to be on the creators allowed list.`],
      });
  });
  it("should not allow unauthorized callers from getting the list of allowed creators", async () => {
      let result = await getRandomActor().get_allowed_creators_list();
      expect(result.err).toStrictEqual({
          kind: {
              NotAuthorized: null,
          },
      });
      result = await anonymousActor.get_allowed_creators_list();
      expect(result.err).toStrictEqual({
          kind: {
              NotAuthorized: null,
          },
      });
  });
  it("should get the correct list of allowed creators", async () => {
      // just adding trivial randomness for the test
      await delegatedAdministrator.add_allowed_creator({ who: getRandomPrincipal() });
      await delegatedAdministrator.add_allowed_creator({ who: getRandomPrincipal() });
      await delegatedAdministrator.add_allowed_creator({ who: getRandomPrincipal() });
      let result = await delegatedAdministrator.get_allowed_creators_list();
      // there is the array of principals 
      expect(result?.ok?.allowed).toBeTruthy();
      for (let p of result.ok.allowed) {
          await delegatedAdministrator.remove_allowed_creator({ who: p });
      }
      result = await delegatedAdministrator.get_allowed_creators_list();
      // since everything should have been removed
      expect(result?.ok?.allowed.length).toStrictEqual(0);
      await delegatedAdministrator.add_allowed_creator({ who: onePrincipal });
      result = await delegatedAdministrator.get_allowed_creators_list();
      let allowed = result?.ok?.allowed;
      expect(`${allowed}`).toStrictEqual(`${onePrincipal}`);
  });
  it("should not allow unauthorized callers to remove any principals from the list", async () => {
      let result = await anonymousActor.remove_allowed_creator({ who: onePrincipal });
      expect(result.err).toStrictEqual({
          kind: {
              NotAuthorized: null,
          },
          message: [`You are not authorized to modify the creators allowed list.`],
      });
      result = await getRandomActor().remove_allowed_creator({ who: onePrincipal });
      expect(result.err).toStrictEqual({
          kind: {
              NotAuthorized: null,
          },
          message: [`You are not authorized to modify the creators allowed list.`],
      });
  });
  it("should return error if principal to remove isn't on the list prior to call for removal", async () => {
      const aPrincipal = getRandomPrincipal();
      await delegatedAdministrator.add_allowed_creator({ who: aPrincipal });
      await delegatedAdministrator.remove_allowed_creator({ who: aPrincipal });
      let result = await delegatedAdministrator.remove_allowed_creator({ who: aPrincipal });
      expect(result.err).toStrictEqual({
          kind: {
              NotFound: null,
          },
          message: [`Could not remove ${aPrincipal.toText()}, principal not found in creators allowed list.`],
      });
  });
  it("should remove a specified principal alright if it is already present on the creators allowed list", async () => {
      const aPrincipal = getRandomPrincipal();
      await delegatedAdministrator.add_allowed_creator({ who: aPrincipal });
      let result = await delegatedAdministrator.remove_allowed_creator({ who: aPrincipal });
      expect(result.ok).toStrictEqual({
          message: `Successfully removed principal ${aPrincipal.toText()} from the creators allowed list.`,
      });
  });
});