import { Component } from '@angular/core';
import { Login } from '../models/login';
import { LoginService } from '../service/login.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent {

  public user = "admin";
  public password = "";

  constructor(private loginService: LoginService){

  }

  onClick(): void {

    const loginUser: Login = {
      User: this.user,
      Password: this.password
    }

    this.loginService.login(loginUser);
    console.log("onClick");
  }
}
