import {AfterViewInit, Component, ViewChild} from "@angular/core";
import {CountdownTimerComponent} from "./countdown-timer-component";

@Component({
  selector: 'app-timers-screen',
  template: `
    <h1>Timers</h1>

    <h2>Timer 1: using a #reference variable</h2>
        <button (click)="timer.restart()">Restart</button>
        <div class="seconds">{{timer.seconds}}</div>
        <app-countdown-timer #timer></app-countdown-timer>

    <h2>Timer 2: using a @ViewChild()</h2>
    UGLY!
        <button (click)="restart()">Restart</button>
        <div class="seconds">{{seconds()}}</div>
        <app-countdown-timer></app-countdown-timer>
  `,
})
export class TimersScreenComponent implements AfterViewInit {
  // Timer 2: ViewChild
  @ViewChild(CountdownTimerComponent)
  private timerComponent: CountdownTimerComponent;

  ngAfterViewInit() {
    // Redefine `seconds()` to get from the `CountdownTimerComponent.seconds` ...
    // but wait a tick first to avoid one-time devMode
    // unidirectional-data-flow-violation error
    setTimeout(() => this.seconds = () => this.timerComponent.seconds, 0);
  }
  seconds() { return 0; }
  restart() { this.timerComponent.restart(); }
}
