import { Component, OnInit } from '@angular/core';
import {FormArray, FormBuilder, FormControl, FormGroup, Validators} from "@angular/forms";
import {forbiddenNameValidator} from "./forbidden-name.directive";
import {identityRevealedValidator} from "./identity-revealed.validator";

@Component({
  selector: 'app-forms-reactive',
  template: `
    <form [formGroup]="profileForm" (ngSubmit)="onSubmit()">
      <label> First Name:
        <input type="text" formControlName="firstName" required #name>
      </label>

      <div *ngIf="name.invalid && (name.dirty || name.touched)"
           class="alert alert-danger">

        <div *ngIf="name.errors.required">
          Name is required.
        </div>
        <div *ngIf="name.errors.minlength">
          Name must be at least 4 characters long.
        </div>
        <div *ngIf="name.errors.forbiddenName">
          Name cannot be Bob.
        </div>
      </div>

      <div *ngIf="profileForm.errors?.identityRevealed && (profileForm.touched || profileForm.dirty)" class="cross-validation-error-message alert alert-danger">
        Name cannot match alter ego.
      </div>

      <label> Last Name:
        <input type="text" formControlName="lastName">
      </label>

      <div formGroupName="address">
        <label>Street:
          <input type="text" formControlName="street">
        </label>

        <label>City:
          <input type="text" formControlName="city">
        </label>

        <label>State:
          <input type="text" formControlName="state">
        </label>

        <label>Zip Code:
          <input type="text" formControlName="zip">
        </label>
      </div>

      <div formArrayName="aliases">
        <h3>Aliases</h3> <button (click)="addAlias()">Add Alias</button>
        <div *ngFor="let alias of aliases.controls; let i=index">
          <label>Alias: <input type="text" [formControlName]="i"></label>
        </div>
      </div>

      <button type="submit" [disabled]="!profileForm.valid">Submit</button>  Form status: {{profileForm.status}}
    </form>
  `,
})
export class ReactiveComponent implements OnInit {
  profileForm = new FormGroup({
    firstName: new FormControl('', [
      Validators.required,
      Validators.minLength(4),
      forbiddenNameValidator(/bob/i),
    ]),
    lastName: new FormControl(''),
    // Using the FormBuilder for this part (example)
    address: this.fb.group({
      street: [''],
      city: [''],
      state: [''],
      zip: ['']
    }),
    // FormArray is used for dynamic forms
    aliases: this.fb.array([
      this.fb.control('')
    ]),
  },
    // Validators on a group ("cross-validation")
    { validators: identityRevealedValidator });

  /** Easy access to aliases
   */
  get aliases() {
    return this.profileForm.get('aliases') as FormArray;
  }

  // Add more elements to the form
  addAlias() {
    this.aliases.push(this.fb.control(''));
  }

  constructor(private fb: FormBuilder) { }

  ngOnInit(): void {
    // This is how to set values through a control
    this.profileForm.get('firstName').setValue('John');

    // patchValue() fails silently for nesting errors in complex forms
    this.profileForm.patchValue({
      firstName: 'Nancy',
      address: {
        street: '123 Drew Street'
      }
    })
  }

  onSubmit() {
    // TODO: Use EventEmitter with form value
    console.warn(this.profileForm.value);
  }
}
