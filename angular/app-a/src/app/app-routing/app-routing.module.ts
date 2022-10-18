import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { AppRoutingRoutingModule } from './app-routing-routing.module';
import { AppRoutingComponent } from './app-routing.component';
import {FirstComponent} from "./first/first.component";
import {SecondComponent} from "./second/second.component";


@NgModule({
  declarations: [
    AppRoutingComponent,
    FirstComponent,
    SecondComponent,
  ],
  imports: [
    CommonModule,
    AppRoutingRoutingModule,
  ],
})
export class AppRoutingModule { }
