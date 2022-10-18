import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-demo-template-reference',
  template: `
    <label>Type something:
      <input #customerInput>
      <button (click)="sendValue(customerInput.value)">Send</button>
      <span>{{customerInput.value}}</span>
    </label>
  `,
})
export class DemoTemplateReferenceComponent {
  constructor() { }

  sendValue(value: string){
    console.log(value);
  }
}
