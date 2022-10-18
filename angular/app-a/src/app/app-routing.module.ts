import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import {TimersScreenComponent} from "./timer/timers-screen.component";
import {IndexComponent} from "./index.component";


const routes: Routes = [
  {
    path: '',
    component: IndexComponent,
  },
  { path: 'heroes',
    // This is how lazy loading of modules works:
    // loadChildren: import().then()
    // Also be sure to remove the HeroDemoModule from the AppModule.
    loadChildren: () => import('./hero-demo/hero-demo.module').then(m => m.HeroDemoModule)},
  {path: 'timers',
    component: TimersScreenComponent},
  { path: 'rxjs-playground',
    loadChildren: () => import('./rxjs-playground/rxjs-playground.module').then(m => m.RxjsPlaygroundModule) },
  { path: 'http-client',
    loadChildren: () => import('./http-client/http-client.module').then(m => m.HttpClientModule) },
  { path: 'app-routing', loadChildren: () => import('./app-routing/app-routing.module').then(m => m.AppRoutingModule) },
  { path: 'app-animations', loadChildren: () => import('./app-animations/app-animations.module').then(m => m.AppAnimationsModule) },
  { path: 'app-translations', loadChildren: () => import('./app-translations/app-translations.module').then(m => m.AppTranslationsModule) }];
//  { path: '',   redirectTo: '/first-component', pathMatch: 'full' }, // redirect to `first-component`
//  { path: '**', component: PageNotFoundComponent },  // Wildcard route for a 404 page
//  { path: 'hero/:id',      component: HeroDetailComponent },

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
