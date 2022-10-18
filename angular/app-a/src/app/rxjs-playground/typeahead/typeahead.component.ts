import { Component, OnInit } from '@angular/core';
import {FormControl} from "@angular/forms";
import {debounceTime, distinctUntilChanged, filter, map, mergeMap, retryWhen, switchMap} from "rxjs/operators";
import {ajax, AjaxResponse} from "rxjs/ajax";
import {BehaviorSubject, Observable, observable, pipe, range, Subject, timer, zip} from "rxjs";

// Typically, a type-ahead has to do a series of separate tasks:
// * Listen for data from an input.
// * Trim the value (remove whitespace) and make sure it’s a minimum length.
// * Debounce (so as not to send off API requests for every keystroke, but instead wait for a break in keystrokes).
// * Don’t send a request if the value stays the same (rapidly hit a character, then backspace, for instance).
// * Cancel ongoing AJAX requests if their results will be invalidated by the updated results.

@Component({
  selector: 'rxjs-typeahead',
  template: `
    <label>Enter a place name: <input [formControl]="inputPlaceControl"></label>
    <p>{{suggestedPlace}}</p>
  `,
})
export class TypeaheadComponent implements OnInit {
  GOOGLE_MAPS_API_KEY = 'AIzaSyAHrdiIYSDhPbkBut5_9D6xo7nfs9ozDJ4';
  GOOGLE_MAPS_API_URL = `https://cors-anywhere.herokuapp.com/https://maps.googleapis.com/maps/api/place/autocomplete/json?key=${this.GOOGLE_MAPS_API_KEY}`;

  inputPlaceControl = new FormControl('');
  inputPlaceTypeahead: Observable<GoogleTypeaheadResponse> = this.inputPlaceControl.valueChanges.pipe(
    filter(text => text.length > 2),
    debounceTime(10),
    distinctUntilChanged(),
    switchMap((value) => ajax(this.GOOGLE_MAPS_API_URL + '&input=' + value)),
    exponentialBackoff(3, 250),
    map((response: AjaxResponse) => response.response)
  );

  suggestedPlace: string;

  constructor() {
  }

  ngOnInit(): void {
    this.inputPlaceTypeahead.subscribe((response) => {
      console.log(response);
      this.suggestedPlace = response.predictions[0].description;
    });
  }
}

interface GoogleTypeaheadResponse {
  status: string;
  predictions: Array<GoogleTypeaheadPrediction>;
}

interface GoogleTypeaheadPrediction {
  id: string;
  description: string;
  // more
}

// Example exponentialBackoff implementation
function exponentialBackoff(maxTries, ms) {
  return pipe(
    retryWhen(attempts => zip(range(1, maxTries), attempts)
      .pipe(
        map(([i]) => i * i),
        mergeMap(i =>  timer(i * ms))
      )
    )
  );
}
