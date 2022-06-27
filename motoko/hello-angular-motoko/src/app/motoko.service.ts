import { Injectable } from '@angular/core';
const ic_motoko = require('src/declarations/motoko').motoko;

@Injectable({
  providedIn: 'root'
})
export class MotokoService {

  constructor() { }
  public async greet(name:string){
    return await ic_motoko.greet(name);
  }
  public async test(){
    return await ic_motoko.test();
  }
}
