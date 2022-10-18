import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { RxjsPlaygroundRoutingModule } from './rxjs-playground-routing.module';
import { RxjsPlaygroundComponent } from './rxjs-playground.component';
import { LocationComponent } from './location/location.component';
import { TypeaheadComponent } from './typeahead/typeahead.component';
import {FormsModule, ReactiveFormsModule} from "@angular/forms";


@NgModule({
  declarations: [RxjsPlaygroundComponent, LocationComponent, TypeaheadComponent],
  imports: [
    CommonModule,
    RxjsPlaygroundRoutingModule,
    ReactiveFormsModule,
  ]
})
export class RxjsPlaygroundModule { }
