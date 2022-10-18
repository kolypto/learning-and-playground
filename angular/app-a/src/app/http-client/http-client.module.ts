import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { HttpClientRoutingModule } from './http-client-routing.module';
import { HttpClientComponent } from './http-client.component';
import { HttpClientModule as AngularHttpClientModule } from '@angular/common/http';
import { EditUserComponent } from './edit-user/edit-user.component';
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {httpInterceptorProviders} from "./interceptors";


@NgModule({
  declarations: [
    HttpClientComponent,
    EditUserComponent,
  ],
  imports: [
    CommonModule,
    HttpClientRoutingModule,
    AngularHttpClientModule,
    FormsModule,
    ReactiveFormsModule,
  ],
  providers: [
    AngularHttpClientModule,
    httpInterceptorProviders,
  ]
})
export class HttpClientModule { }
