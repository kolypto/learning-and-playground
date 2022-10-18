import { Component, OnInit } from '@angular/core';
import {GoogleService, User} from "./google.service";
import {Observable} from "rxjs";

@Component({
  selector: 'app-http-client',
  providers: [
    GoogleService,
  ],
  template: `
    <table>
      <caption>Users</caption>
      <tbody>
        <tr *ngFor="let user of users$ | async">
          <th>{{ user.id }}</th>
          <td>{{ user.name }}</td>
        </tr>
      </tbody>
    </table>

    <a routerLink="add">Add user</a>
  `,
})
export class HttpClientComponent implements OnInit {

  users$: Observable<User[]> = null;

  constructor(private googleSheet: GoogleService) { }

  ngOnInit(): void {
    this.users$ = this.googleSheet.loadUsers();
  }

}
