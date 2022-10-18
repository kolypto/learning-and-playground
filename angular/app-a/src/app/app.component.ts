import {Component, Inject, LOCALE_ID} from '@angular/core';
import {DOCUMENT} from "@angular/common";

@Component({
  selector: 'app-root',
  // templateUrl: './app.component.html',
  template: `
    <nav>
      <ul>
        <li><a routerLink="/heroes" routerLinkActive="active">Heroes</a></li>
        <li><a routerLink="/timers" routerLinkActive="active">Timers</a></li>
        <li><a routerLink="/rxjs-playground" routerLinkActive="active">RxJS playground</a></li>
        <li><a routerLink="/http-client" routerLinkActive="active">HTTP Client</a></li>
        <li><a routerLink="/app-routing" routerLinkActive="active">Routing</a></li>
        <li><a routerLink="/app-animations" routerLinkActive="active">Animations</a></li>
        <li><a routerLink="/app-translations" routerLinkActive="active">Translations</a></li>
      </ul>
    </nav>

    <router-outlet></router-outlet>
  `,
})

export class AppComponent {
  // Locale in <html lang>
  constructor(@Inject(DOCUMENT) private document: Document,
              @Inject(LOCALE_ID) private locale: string
              ) {
    this.document.documentElement.lang = this.locale;
  }
}
