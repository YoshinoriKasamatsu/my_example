import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { map, Observable } from 'rxjs';
import { Todo } from '../models/todo';

@Injectable({
  providedIn: 'root'
})
export class TodoService {

  constructor(private readonly httpClient: HttpClient) { }


  getTodos(): Observable<Todo[]>{
    return this.httpClient.get<Todo[]>("/api/todos").pipe(map((todos: Todo[]) => {
      return todos;
    }))
  }
}
