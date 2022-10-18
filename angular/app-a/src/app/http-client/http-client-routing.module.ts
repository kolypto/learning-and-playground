import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { HttpClientComponent } from './http-client.component';
import {EditUserComponent} from "./edit-user/edit-user.component";

const routes: Routes = [
  { path: '', component: HttpClientComponent },
  { path: 'add', component: EditUserComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class HttpClientRoutingModule { }
