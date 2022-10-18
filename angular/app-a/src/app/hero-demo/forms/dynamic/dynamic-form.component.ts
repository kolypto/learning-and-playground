import { Component, Input, OnInit }  from '@angular/core';
import { FormGroup }                 from '@angular/forms';

import { QuestionBase }              from './dynamic-question';
import { QuestionControlService }    from './question-control.service';

@Component({
  selector: 'app-dynamic-form',
  // This service is local to the component (and its children)
  providers: [ QuestionControlService ],
  template: `
    <div>
      <form (ngSubmit)="onSubmit()" [formGroup]="form">

        <div *ngFor="let question of questions" class="form-row">
          <app-question [question]="question" [form]="form"></app-question>
        </div>

        <div class="form-row">
          <button type="submit" [disabled]="!form.valid">Save</button>
        </div>
      </form>

      <div *ngIf="payLoad" class="form-row">
        <strong>Saved the following values</strong><br>{{payLoad}}
      </div>
    </div>
  `
})
export class DynamicFormComponent implements OnInit {

  @Input() questions: QuestionBase<string>[] = [];
  form: FormGroup;
  payLoad = '';

  constructor(private qcs: QuestionControlService) {  }

  ngOnInit() {
    this.form = this.qcs.toFormGroup(this.questions);
  }

  onSubmit() {
    this.payLoad = JSON.stringify(this.form.getRawValue());
  }
}
