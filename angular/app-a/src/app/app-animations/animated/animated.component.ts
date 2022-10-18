import { Component, OnInit } from '@angular/core';
import { trigger, state, style, animate, transition } from '@angular/animations';

@Component({
  selector: 'app-animated',
  template: `
    <div [@appearDisappear] [@openClose]="isOpen ? 'open' : 'closed'" (click)="isOpen = !isOpen" class="open-close-container">
      <p>The box is now {{ isOpen ? 'Open' : 'Closed' }}!</p>
    </div>

    <!-- Disable animations with @.disabled -->
    <div [@.disabled]="true">
      <div [@appearDisappear] [@openClose]="isOpen ? 'open' : 'closed'" (click)="isOpen = !isOpen" class="open-close-container">
        <p>The box is now {{ isOpen ? 'Open' : 'Closed' }}!</p>
      </div>
    </div>
  `,
  styles: [
    `
      .open-close-container {
        border: 1px solid #dddddd;
        margin-top: 1em;
        padding: 20px 20px 0px 20px;
        color: #000000;
        font-weight: bold;
        font-size: 20px;
      }
    `
  ],
  animations: [
    // Named animation: assign to variables and reference here

    // The trigger() function describes the property name to watch for changes.
    // Use it: <div [@triggerName]="expression">...</div>;
    trigger('openClose', [
      // Named animation states
      // Use state() to define styles that are applied at the end of each transition, they persist after the animation has completed.
      state('open', style({ // style() animations
        height: '100px',
        opacity: 1,
        backgroundColor: 'yellow'
      })),
      state('closed', style({
        height: '50px',
        opacity: 0.5,
        backgroundColor: 'green'
      })),
      // State transitions
      // Use transition() to define intermediate styles, which create the illusion of motion during the animation.
      // When animations are disabled, transition() styles can be skipped, but state() styles can't.
      // Use * to match any state name
      transition('* => closed', [
        // You can include multiple state pairs within the same transition() argument:
        // transition( 'on => off, off => void' ).
        animate('1s')
        // Use keyframes() and multiple style() within for complex animations
        // Use group() to run multiple animations in parallel
      ]),
      transition('* => open', [
        animate('0.5s',
          // Use the wildcard * with a style to tell the animation to use whatever the current style value is,
          // style ({ opacity: '*' }),
          )
      ]),
    ]),
    // You can use the void state to configure transitions for an element that is entering or leaving a page.
    // :enter and :leave are aliases for the void => * and * => void transitions.
    trigger('appearDisappear', [
      transition(':enter', [
        style({ opacity: 0 }),
        animate('100ms', style({ opacity: 1 })),
      ]),
      transition(':leave', [
        animate('100ms', style({ opacity: 0 }))
      ])
    ]),

    // The query() function allows you to find inner elements within the element that is being animated.

    // The stagger() function allows you to define a timing gap between each queried item that is animated
    // and thus animates elements with a delay between them.
  ],
})
export class AnimatedComponent implements OnInit {
  isOpen: boolean = true;

  constructor() { }

  // Callbacks.
  // The animation trigger() function emits callbacks when it starts and when it finishes
  // Use:
  //     (@openClose.start)="onAnimationEvent($event)"
  //     (@openClose.done)="onAnimationEvent($event)"
  onAnimationEvent ( event: AnimationEvent ) {
    console.info(event);
  }

  ngOnInit(): void {
  }

}
