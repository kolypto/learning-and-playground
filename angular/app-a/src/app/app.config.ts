import { InjectionToken } from '@angular/core';

export interface AppConfig {
  apiEndpoint: string,
  title: string,
}

export const HERO_DI_CONFIG: AppConfig = {
  apiEndpoint: 'api.heroes.com',
  title: 'Dependency Injection'
};

// Because a TS Interface can't be used as a DI key (class name), we have to provide it manually
// Declare as:
// providers: [{ provide: APP_CONFIG, useValue: HERO_DI_CONFIG }]
// Inject as:
// @Inject(APP_CONFIG) config: AppConfig
export const APP_CONFIG = new InjectionToken<AppConfig>('app.config');
