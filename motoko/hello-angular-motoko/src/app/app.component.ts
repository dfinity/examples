import { Component } from "@angular/core";
import { MotokoService } from "./motoko.service";
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  standalone: true,
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  public title = 'hello-angular-motoko';
  public response = 'Nothing yet';
  public duration: number = 0;
  
  constructor(private motokoService: MotokoService){
  }

  public async getResponse(username:string = 'Angular'){
    const start = Date.now();
	  console.log("start request")
    this.response = await this.motokoService.greet(username);
    this.duration = Date.now() - start;
    console.log("request time", this.duration)
  }
}
