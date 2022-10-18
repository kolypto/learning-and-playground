import { Component, OnInit } from '@angular/core';
import {from, fromEvent, interval, observable, Observable, of, pipe} from "rxjs";
import {ajax} from "rxjs/ajax";
import {catchError, map, retry} from "rxjs/operators";

@Component({
  selector: 'rxjs-location',
  template: `
    Coordinates:
    <span *ngIf="location">
      {{ location.latitude }}, {{ location.longitude }} (r={{location.accuracy}})
    </span>
  `,
})
export class LocationComponent implements OnInit {
  location$: Observable<Coordinates>;
  location: Coordinates;

  constructor() {
    // An Observable instance begins publishing values only when someone subscribes to it.
    // A typical observable creates a new, independent execution for each subscribed observer.
    this.location$ = new Observable((observer) => {
      let watchId: number;

      // Geolocation service is available?
      if ('geolocation' in navigator) {
        // Subscribe to it, translate its updates to observer stream
        watchId = navigator.geolocation.watchPosition((position: Position) => {
          // Stream of positions
          observer.next(position);
        }, (error: PositionError) => {
          observer.error(error);
        });

        // Return an object that defines how to unsubscribe
        return {
          unsubscribe() {
            navigator.geolocation.clearWatch(watchId);
          }
        }
      } else {
        // Geolocation service not available or denied
        observer.error('Geolocation not available');
      }
    }).pipe(
      // Operators: transform incoming values
      map((position: Position) => position.coords)
      // RxJS has unbound operators which can be prepared and re-used using pipe() function
    );

    // If what you need is a multicast observer, then you'll need to make a function that stores them into
    // an array and manually invokes observer.next() on them for every incoming value (like sockets)

    // Other observables
    of(1, 2, 3); // list of synchronous values
    from([1, 2, 3]); // list of synchronous values
    from(fetch('/api/endpoint')); // from promise
    ajax('/api/data'); // from AJAX request
    interval(1000); // sleep & call in 1000ms
    fromEvent(document.getElementsByTagName('body'), 'mousemove');  // element event

    // Other operators
    pipe(
      retry(3), // Retry up to 3 times before failing
      catchError(err => of([])),  // ignore error, proceed with a value
    );
  }

  ngOnInit(): void {
    // Subscribe to location updates
    const locationsSubscription = this.location$.subscribe({
      next(location) {
        console.log('Current Position: ', location);
        this.location = location;
      },
      error(msg) {
        console.log('Error Getting Location: ', msg);
      },
      complete(){
        console.log('Done watching location');
      }
    });

    // Stop listening for location after 10 seconds
    setTimeout(() => {
      locationsSubscription.unsubscribe();
    }, 30*1000);
  }
}
