import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { AppRoutingComponent } from './app-routing.component';
import {FirstComponent} from "./first/first.component";
import {SecondComponent} from "./second/second.component";
import {UrlNotFoundComponent} from "./url-not-found/url-not-found.component";
import {PersonInfoComponent} from "./person-info/person-info.component";
import {SecurityGuard} from "./security.guard";

const routes: Routes = [
  { path: '', component: AppRoutingComponent,
    children: [
      // Two static routes
      { path: 'first-component', component: FirstComponent },
      { path: 'second-component/:id', component: SecondComponent,
        data: {title: 'Page title from routing table'}},
      // A route with parameters
      { path: 'person-info', component: PersonInfoComponent,
        canActivate: [SecurityGuard]
      },
      // Wildcard route for page not found (for children)
      { path: '**', component: UrlNotFoundComponent },
    ]
  },
  // Wildcard route for page not found (for top-level)
  { path: '**', component: UrlNotFoundComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AppRoutingRoutingModule { }
