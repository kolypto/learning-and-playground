import {Directive, ElementRef, Input} from '@angular/core';

@Directive({
  selector: '[appHeroListItemStyle]'
})
export class HeroListItemStyleDirective {
  _removed: boolean;

  @Input('appHeroListItemStyle')  // same name with the directive to allow [appHeroListItemStyle] binding at once
  set removed(removed: boolean){
    this._removed = removed;
    this.applyStyles();
  }

  el: ElementRef;

  // An ElementRef is a wrapper around a DOM element, whose nativeElement property exposes the DOM element for the directive to manipulate.
  constructor(el: ElementRef) {
    this.el = el;
  }

  applyStyles(){
    console.log(this._removed);
    this.el.nativeElement.style.textDecoration = this._removed? 'line-through' : 'inherit';
  }
}
