import { Component, OnInit } from '@angular/core';
import {ActivatedRoute} from "@angular/router";

@Component({
  selector: 'app-second',
  template: `
    <h1>{{pageTitle}}</h1>
    <p>
      second works! Route value: {{product}}
    </p>
  `,
})
export class SecondComponent implements OnInit {
  // product from route parameters
  product?: string;
  // page title from routing table
  pageTitle?: string;

  constructor(public route: ActivatedRoute) { }

  ngOnInit(): void {
    // Get route parameter
    this.route.paramMap.subscribe(params => {
      this.product = 'something';
    });

    // Get route data
    this.route.data.subscribe(data => {
      this.pageTitle = data.title;
    })
  }

}
