import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { HeroDemoRoutingModule } from './hero-demo-routing.module';
import { HeroDemoComponent } from './hero-demo.component';
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {DemoTemplateReferenceComponent} from "./heroes/demo-template-reference.component";
import {DemoHeroListComponent} from "./heroes/demo-hero-list.component";
import {DemoHeroListItemComponent} from "./heroes/demo-hero-list-item.component";
import {DemoHeroManagerComponent} from "./heroes/demo-hero-manager.component";
import {HeroListItemStyleDirective} from "./heroes/hero-list-item-style.directive";
import {BlinkerDirective} from "./heroes/blinker.directive";
import {IndexComponent} from "./forms/index.component";
import {ReactiveComponent} from "./forms/reactive.component";
import {TemplateComponent} from "./forms/template.component";
import {ForbiddenValidatorDirective} from "./forms/forbidden-name.directive";
import {IdentityRevealedValidatorDirective} from "./forms/identity-revealed.validator";
import {DynamicFormQuestionComponent} from "./forms/dynamic/dynamic.component";
import {DynamicFormComponent} from "./forms/dynamic/dynamic-form.component";
import {APP_CONFIG, HERO_DI_CONFIG} from "../app.config";


@NgModule({
  declarations: [
    HeroDemoComponent,
    DemoTemplateReferenceComponent,
    DemoHeroListComponent,
    DemoHeroListItemComponent,
    DemoHeroManagerComponent,
    HeroListItemStyleDirective,
    BlinkerDirective,
    IndexComponent,
    ReactiveComponent,
    TemplateComponent,
    ForbiddenValidatorDirective,
    IdentityRevealedValidatorDirective,
    DynamicFormQuestionComponent,
    DynamicFormComponent,
  ],
  imports: [
    CommonModule,
    HeroDemoRoutingModule,
    FormsModule,
    ReactiveFormsModule,
  ],
  providers: [
    // Here's how a static configuration is provided
    { provide: APP_CONFIG, useValue: HERO_DI_CONFIG },
  ],
  bootstrap: [HeroDemoComponent],
})
export class HeroDemoModule { }
