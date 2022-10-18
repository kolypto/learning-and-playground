import { Component, OnInit } from '@angular/core';

// $ ng add @angular/localize
// $ ng xi18n

/* angular.json:
  projects/ * :
     "i18n": {
      "sourceLocale": "en-US",
      "locales": {
        "ru": "src/locale/messages.ru.xlf"
      }
    },
  projects/ * /build/options:
    "localize": true,
    "localize": ["ru"],
    "aot": true,
    "i18nMissingTranslation": "error", // "warning" // "ignore"
 */

/* app.component.ts:

  // locale in <html lang>
  constructor(@Inject(DOCUMENT) private document: Document,
              @Inject(LOCALE_ID) private locale: string
              ) {
    this.document.documentElement.lang = this.locale;
  }
*/

/* Set locale id manually:
  AppModule:
  providers: [ { provide: LOCALE_ID, useValue: 'fr' } ],
 */

@Component({
  selector: 'app-app-translations',
  // Pipes: |date, |currency, |decimal, |percent
  template: `
    <!-- Mark for translation; provide a meaning|description@@id.
    The extractor will use it as an identifier -->
    <h1 i18n="greets the user|Title of the main page">Hello</h1>

    <h2>{{this.localizedTitle}}</h2>

    <!-- Translatable text -->
    <p><ng-container i18n>Translatable text without a tag</ng-container></p>

    <!-- Translatable attribute -->
    <p><img [src]="logo" i18n-title title="Random image" /></p>

    <!-- Plurals -->
    <span i18n>
      Updated
        {minutes, plural,
          =0 {just now}
          =1 {one minute ago}
          other {{{minutes}} minutes ago}}

      <!-- depending on a variable: gender -->
      by
      {gender, select,
        male {him}
        female {her}
        other {them}}
    </span>
  `
})
export class AppTranslationsComponent implements OnInit {
  logo = 'https://picsum.photos/100/100';
  minutes = 1;
  gender = 'male';

  localizedTitle: string;

  constructor() { }

  ngOnInit(): void {
    // Code strings
    // :meaning|description@@id: message
    this.localizedTitle = $localize`:@@siteTitle:Example Title`;
  }

}
