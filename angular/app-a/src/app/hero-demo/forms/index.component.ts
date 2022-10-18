import { Component } from '@angular/core';
import {Observable} from "rxjs";
import {QuestionBase} from "./dynamic/dynamic-question";
import {QuestionService} from "./dynamic/dynamic-question.service";

@Component({
  selector: 'app-forms-index',
  // This service is local to the component (and its children)
  providers: [QuestionService],
  template: `
    <h1>Reactive forms</h1>
    <app-forms-reactive></app-forms-reactive>

    <h1>Template forms</h1>
    <app-form-template></app-form-template>

    <h1>Dynamic form</h1>
    <app-dynamic-form [questions]="questions$ | async"></app-dynamic-form>
  `,
})
export class IndexComponent {
  // NOTE: It's an observable, rendered into the templates using an Async Pipe (| async)
  questions$: Observable<QuestionBase<any>[]>;

  constructor(service: QuestionService) {
    this.questions$ = service.getQuestions();
  }
}
