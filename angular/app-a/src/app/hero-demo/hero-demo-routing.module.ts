import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { HeroDemoComponent } from './hero-demo.component';

const routes: Routes = [
  { path: '', component: HeroDemoComponent }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class HeroDemoRoutingModule { }
