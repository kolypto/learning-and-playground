import { Injectable } from '@angular/core';
import {Hero} from "./hero";
import {Observable} from "rxjs";
import {UserService} from "../user.service";

// @Injectable({  // Not used anymore because there's a heroServiceProvider down there
//   // specify the provider of the decorated service class with the root injector, or with the injector for a specific NgModule.
//   providedIn: 'root',
// })
export class HeroService {
  /**
   * @param isAuthorized Only authorized user accounts have access to the list of heroes
   */
  constructor(private isAuthorized: boolean) { }

  heroes: Observable<Array<Hero>> = new Observable((observer) => {
    observer.next([
      new Hero(1, 'Windstorm'),
      new Hero(13, 'Bombasto'),
      new Hero(15, 'Magneta'),
      new Hero(20, 'Tornado')
    ]);
    return {
      unsubscribe(){}
    }
  });
}

// A factory provider (with arguments) needs a factory function
// It will receive its arguments from another service, the `UserService`
let heroServiceFactory = (userService: UserService) => {
  return new HeroService(userService.user.isAuthorized);
};

// Use this one in `providers = []` instead
export let heroServiceProvider =
  // tells Angular that the provider is a factory function whose implementation is heroServiceFactory.
  { provide: HeroService,
    useFactory: heroServiceFactory,
    deps: [UserService]
  };
