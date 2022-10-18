import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-app-animations',
  template: `
  <h1>Animations</h1>

  <app-animated></app-animated>
  `,
})
export class AppAnimationsComponent implements OnInit {

  constructor() { }

  ngOnInit(): void {
  }

}
