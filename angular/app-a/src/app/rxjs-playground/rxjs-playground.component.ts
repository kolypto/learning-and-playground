import { Component, OnInit } from '@angular/core';
import {from, fromEvent, interval, observable, Observable, of, pipe} from "rxjs";
import {ajax} from "rxjs/ajax";
import {catchError, map, retry} from "rxjs/operators";

@Component({
  selector: 'app-rxjs-playground',
  template: `
    <p><rxjs-location></rxjs-location>
    <p><rxjs-typeahead></rxjs-typeahead>
  `,
  styles: [],
})
export class RxjsPlaygroundComponent implements OnInit {
  constructor() {
  }
  ngOnInit(): void {
  }
}
