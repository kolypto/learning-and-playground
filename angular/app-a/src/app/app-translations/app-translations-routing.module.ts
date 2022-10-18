import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { AppTranslationsComponent } from './app-translations.component';

const routes: Routes = [{ path: '', component: AppTranslationsComponent }];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AppTranslationsRoutingModule { }
