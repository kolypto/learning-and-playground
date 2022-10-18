import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { AppAnimationsRoutingModule } from './app-animations-routing.module';
import { AppAnimationsComponent } from './app-animations.component';
import {BrowserAnimationsModule} from "@angular/platform-browser/animations";
import { AnimatedComponent } from './animated/animated.component';


@NgModule({
  declarations: [AppAnimationsComponent, AnimatedComponent],
  imports: [
    CommonModule,
    AppAnimationsRoutingModule,
  ]
})
export class AppAnimationsModule { }
