import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { FormsModule } from '@angular/forms';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { LoginComponent } from './login/login.component';
import { LoginService } from './service/login.service';
import { HttpClientModule } from '@angular/common/http';
import { ListComponent } from './todo/list/list.component';
import { TodoModule } from './todo/todo.module';

@NgModule({
  declarations: [
    AppComponent,
    LoginComponent,
    ListComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    FormsModule,
    HttpClientModule, 
    TodoModule,
  ],
  providers: [LoginService],
  bootstrap: [AppComponent]
})
export class AppModule { }
