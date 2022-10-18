import {Component, EventEmitter, Input, OnInit, Output} from '@angular/core';
import {Hero} from "../hero";

@Component({
  selector: 'app-hero-list-item',
  template: `
    <ng-container>
      <a appBlinker
        [appHeroListItemStyle]="hero.deleted"
        (click)="selectHero()"
      >{{ hero.name }}</a>
      <a *ngIf="!hero.deleted" (click)="deleteHero(hero, $event)">[x]</a>
    </ng-container>
  `,
})
export class DemoHeroListItemComponent {
  @Input()
  hero: Hero;

  @Output() selected = new EventEmitter<Hero>();

  constructor() {
  }

  deleteHero(hero: Hero, $event: Event){
    hero.deleted = true;
    console.log($event);
  }

  selectHero(){
    this.selected.emit(this.hero);
  }
}
