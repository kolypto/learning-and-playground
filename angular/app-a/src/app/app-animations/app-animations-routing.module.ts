import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { AppAnimationsComponent } from './app-animations.component';

const routes: Routes = [{ path: '', component: AppAnimationsComponent }];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AppAnimationsRoutingModule { }
