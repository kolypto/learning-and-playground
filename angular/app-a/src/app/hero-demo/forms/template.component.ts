import { Component } from '@angular/core';
import {Hero} from "../hero";

@Component({
  selector: 'app-form-template',
  template: `
    <div class="container">
      <h1>Hero Form</h1>
      <form (ngSubmit)="onSubmit()" #heroForm="ngForm">

        {{diagnostic}}

        <div class="form-group">
          <label for="name">Name</label>
          <input type="text" class="form-control" id="name"
                 required minlength="4" appForbiddenName="bob"
                 [(ngModel)]="model.name" name="name" #spy
                 #name="ngModel"
          >
          <!-- ng-touched ng-dirty ng-valid  -->
          <!-- ng-untouched ng-pristine ng-invalid  -->
          <b>class</b>: {{spy.className}}

          <div [hidden]="name.valid || name.pristine"
               class="alert alert-danger">
            Validation errors: {{name.errors|json}}

            <div *ngIf="name.errors?.required">Name is required.</div>
            <div *ngIf="name.errors?.minlength">Name must be at least 4 characters long.</div>
            <div *ngIf="name.errors?.forbiddenName">Name cannot be Bob.</div>

          </div>
        </div>

        <div class="form-group">
          <label for="alterEgo">Alter Ego</label>
          <input type="text"  class="form-control" id="alterEgo" [(ngModel)]="model.alterEgo" name="alterEgo">
        </div>

        <div class="form-group">
          <label for="power">Hero Power</label>
          <select class="form-control"  id="power" required [(ngModel)]="model.power" name="power">
            <option *ngFor="let pow of powers" [value]="pow">{{pow}}</option>
          </select>
        </div>

        <button type="submit" class="btn btn-success">Submit</button>

        <button type="button" class="btn btn-default"
                (click)="newHero(); heroForm.reset()">Reset</button>

      </form>
    </div>
  `,
  styles: [
    // WOW! Including bootstrap as an URL!
    "@import url('https://unpkg.com/bootstrap@3.3.7/dist/css/bootstrap.min.css');",
    `
      /* NOTE: put it into <link rel="stylesheet" href="assets/forms.css"> */
      .ng-valid[required], .ng-valid.required  {
        border-left: 5px solid #42A948; /* green */
      }

      .ng-invalid:not(form)  {
        border-left: 5px solid #a94442; /* red */
      }
    `
  ]
})
export class TemplateComponent {
  powers = ['Really Smart', 'Super Flexible', 'Super Hot', 'Weather Changer'];

  model = new Hero(18, 'Dr IQ', false, this.powers[0],'Chuck Overstreet');

  constructor() { }

  get diagnostic() { return JSON.stringify(this.model); }

  newHero() {
    this.model = new Hero(42, '', false);
  }

  onSubmit(){

  }
}
