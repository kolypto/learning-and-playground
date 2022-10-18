import {Component} from '@angular/core';
import {Hero} from "../hero";

@Component({
  selector: 'app-hero-manager',
  template: `
    <h1>Hero List</h1>
    <app-demo-hero-list (selectedHero)="favoriteHero=$event"></app-demo-hero-list>
    <div *ngIf="favoriteHero">Favorite: {{favoriteHero!.name}}</div>
  `,
})
export class DemoHeroManagerComponent {
  favoriteHero: Hero;
}
