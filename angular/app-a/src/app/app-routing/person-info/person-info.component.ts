import { Component, OnInit } from '@angular/core';
import {Observable, of} from "rxjs";
import {ActivatedRoute} from "@angular/router";
import {switchMap} from "rxjs/operators";

@Component({
  selector: 'app-person-info',
  template: `
    <h1>Person from URL parameter</h1>

    <p>Selected: {{this.selectedId}}
    <p>Name: {{(this.person$ | async)?.name}}
  `,
})
export class PersonInfoComponent implements OnInit {
  person$: Observable<User>;
  selectedId: number;

  constructor(private route: ActivatedRoute) {
  }

  ngOnInit(): void {
    // Sync: get query parameters
    let id = this.route.snapshot.paramMap.get('id');
    // this.person$ = this.service.getHero(id);

    // Async: get query parameters and immediately load relevan objects
    this.person$ = this.route.paramMap.pipe<User>(
      // When the `id` of a person is available, load it from a service
      switchMap(params => {
        // Get the id from a parameter
        this.selectedId = Number(params.get('id'));
        // Load it from the service
        // return this.service.getHeroes();
        return of(
          {id: this.selectedId, name: 'Johnson'} as User,
        );
      })
    );
  }
}

interface User {
  id: number;
  name: string;
}
