import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { RxjsPlaygroundComponent } from './rxjs-playground.component';

const routes: Routes = [{ path: '', component: RxjsPlaygroundComponent }];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class RxjsPlaygroundRoutingModule { }
