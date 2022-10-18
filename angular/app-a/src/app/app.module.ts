import {BrowserModule} from '@angular/platform-browser';
import {Inject, NgModule} from '@angular/core';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {TimersScreenComponent} from "./timer/timers-screen.component";
import {CountdownTimerComponent} from "./timer/countdown-timer-component";
import {HeroDemoModule} from "./hero-demo/hero-demo.module";
import {IndexComponent} from "./index.component";
import {APP_CONFIG, AppConfig} from "./app.config";
import { UrlNotFoundComponent } from './app-routing/url-not-found/url-not-found.component';
import { PersonInfoComponent } from './app-routing/person-info/person-info.component';
import {BrowserAnimationsModule} from "@angular/platform-browser/animations";
import { ServiceWorkerModule } from '@angular/service-worker';
import { environment } from '../environments/environment';
import { RouterModule } from '@angular/router';

@NgModule({
  declarations: [
    AppComponent,
    CountdownTimerComponent,
    TimersScreenComponent,
    IndexComponent,
    UrlNotFoundComponent,
    PersonInfoComponent,
  ],
  imports: [
    BrowserModule.withServerTransition({ appId: 'serverApp' }),
    BrowserAnimationsModule,
    AppRoutingModule,
    HeroDemoModule,
    ServiceWorkerModule.register('ngsw-worker.js', { enabled: environment.production }),
    RouterModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
  // The @Inject decorator is a constructor parameter used to specify a custom provider of a dependency.
  constructor(@Inject(APP_CONFIG) config: AppConfig) {
  }
}
