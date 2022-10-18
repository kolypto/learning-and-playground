import { Component, OnInit } from '@angular/core';
import {ActivatedRoute, Router} from "@angular/router";

@Component({
  selector: 'app-url-not-found',
  template: `
    <h1>404</h1>
    <p>Page not found</p>

    <p>Route: {{route.url.toString()}}, params: {{route.params|json}}</p>
  `,
})
export class UrlNotFoundComponent implements OnInit {

  constructor(public router: Router, public route: ActivatedRoute) { }

  ngOnInit(): void {
    console.log(this.route);
  }

}
