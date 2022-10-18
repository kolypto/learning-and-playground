import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { AppTranslationsRoutingModule } from './app-translations-routing.module';
import { AppTranslationsComponent } from './app-translations.component';


@NgModule({
  declarations: [AppTranslationsComponent],
  imports: [
    CommonModule,
    AppTranslationsRoutingModule
  ]
})
export class AppTranslationsModule { }
