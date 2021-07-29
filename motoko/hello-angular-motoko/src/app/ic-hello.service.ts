import { Injectable } from '@angular/core';
const ic_hello = require('src/declarations/hello').hello;

@Injectable({
  providedIn: 'root'
})
export class IcHelloService {

  constructor() { }
  public async greet(name:string){
    return await ic_hello.greet(name);
  }
  public async test(){
    return await ic_hello.test();
  }
}
