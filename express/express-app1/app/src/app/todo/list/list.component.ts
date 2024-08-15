import { Component, OnInit } from '@angular/core';
import { Todo } from 'src/app/models/todo';
import { TodoService } from '../todo.service';

@Component({
  selector: 'app-list',
  templateUrl: './list.component.html',
  styleUrls: ['./list.component.scss']
})
export class ListComponent implements OnInit {

  public todos: Todo[] = [];

  constructor(private readonly todoService: TodoService)
  {}


  ngOnInit(): void {
      this.todoService.getTodos().subscribe((todos: Todo[])=> {
        console.log(todos);
        this.todos = todos;
      })
  }
}
