import { Injectable } from '@angular/core';


import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Login } from '../models/login';

@Injectable({
  providedIn: 'root'
})
export class LoginService {

  private loginUrl = "/api/login";

  constructor(private http: HttpClient) { }

  login(login: Login ): Observable<Login>{
    console.log(login);
    return this.http.post<Login>(this.loginUrl, login);
  }
}
