import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {Hero} from "../hero";
import {HeroService, heroServiceProvider} from "../hero.service";

@Component({
  selector: 'app-demo-hero-list',
  template: `
    <h1>{{title}}</h1>
    <p>Heroes:</p>
    <ul>
      <li *ngFor="let hero of heroes; let i=index; let odd=odd; trackBy: trackByHeroes">
        {{i}}: <app-hero-list-item
          [hero]="hero"
          (selected)="selectHero(hero)"
        ></app-hero-list-item>
      </li>
    </ul>
    <p *ngIf="heroes!.length > 3">There are many heroes!</p>
  `,
  providers: [heroServiceProvider],
})
export class DemoHeroListComponent implements OnInit {
  title = 'Tour of heroes';
  heroes: Array<Hero>;
  myHero: Hero;

  @Output() selectedHero = new EventEmitter<Hero>();


  // Injector.
  // Resolution modifiers

  // @Optional() allows Angular to consider a service you inject to be optional.
  // This means that the NullInjector above the root injector won't throw an error but will use `null`

  // Use @Self() so that Angular will only look at the ElementInjector for the current component or directive.
  // A good use case for @Self() is to inject a service but only if it is available on the current host element.
  // To avoid errors in this situation, combine @Self() with @Optional().

  // With @SkipSelf(), Angular starts its search for a service in the parent ElementInjector, rather than in the current one.
  // Use case: same service is configured in the current @Component with different settings

  // @Host() lets you designate a component as the last stop in the injector tree when searching for providers.
  constructor(private heroService: HeroService) {
    this.title = 'Tour of Heroes';
  }

  ngOnInit() {
    let self = this;
    // Load heroes from the service
    this.heroService.heroes.subscribe({
      next(heroes){
        self.heroes = heroes;
      },
      error: console.error
    })
  }

  selectHero(hero: Hero){
    this.selectedHero.emit(hero);
  }

  // Optimize long list handling
  trackByHeroes(index: number, hero: Hero): number { return hero.id; }
}
