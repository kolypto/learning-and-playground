import { Component, OnInit } from '@angular/core';
import {GoogleService, User} from "../google.service";
import {FormControl, FormGroup} from "@angular/forms";
import {Router} from "@angular/router";

/** Edit a user
 */
@Component({
  selector: 'app-edit-user',
  providers: [
    GoogleService,
  ],
  template: `
    <form [formGroup]="userForm" (ngSubmit)="onSubmit()">
        <p><label>Id: {{userForm.get('id').value}}</label>
        <p><label>Name:  <input type="text" formControlName="name" /></label>
        <input type="submit" />
    </form>
  `,
})
export class EditUserComponent implements OnInit {
  userForm = new FormGroup({
    id: new FormControl(),
    name: new FormControl(''),
  });

  constructor(private googleSheet: GoogleService, private router: Router){
    this.userForm.get('id').setValue(uuidv4());
  }

  ngOnInit(): void {
  }

  onSubmit(){
    this.googleSheet.addUser(this.userForm.value).subscribe(response => {
      this.router.navigate(['../']);
    });
  }
}


function uuidv4() {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}
