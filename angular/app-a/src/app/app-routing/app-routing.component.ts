import { Component, OnInit } from '@angular/core';
import {ActivatedRoute, ParamMap, Router} from "@angular/router";

@Component({
  selector: 'app-app-routing',
  template: `
    <h1>Angular Router App</h1>
    <!-- Menu -->
    <nav>
      <ul>
        <li><a routerLink="first-component" routerLinkActive="active">First Component</a></li>
        <li><a [routerLink]="['second-component', 1]" routerLinkActive="active">Second Component, id=1</a></li>
        <li><a routerLink="person-info" [queryParams]="{id: 1}" routerLinkActive="active">Person info (parameter)</a></li>
        <li><a routerLink="404" routerLinkActive="active">non-existent page</a></li>
      </ul>
    </nav>

    <!-- Insertion point: <router-outlet>-->
    <router-outlet></router-outlet>

    <p>Current route: see console.
    <p>All routes: see console.
  `
})
export class AppRoutingComponent implements OnInit {

  constructor(private router: Router, private route: ActivatedRoute) { }

  ngOnInit(): void {
    // DEBUG: Show all routes
    console.info('Current route', this.route);
    console.info('Routes', this.router.config);

    this.route.queryParams.subscribe(params => {
      console.log('Route params', params);
    });
  }
}
