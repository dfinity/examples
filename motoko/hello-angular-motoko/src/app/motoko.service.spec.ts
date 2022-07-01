import { TestBed } from '@angular/core/testing';
import { MotokoService } from "./motoko.service";

describe("MotokoService", () => {
  let motokoService: MotokoService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    motokoService = TestBed.inject(MotokoService);
  });

  it('should be created', () => {
    expect(motokoService).toBeTruthy();
  });

  it('should return test reposonse from motoko', async () => {
    const reponse = await motokoService.test();
    expect(reponse).toEqual('test from main.mo')
  });
});
