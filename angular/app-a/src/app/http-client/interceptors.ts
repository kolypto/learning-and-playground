import {HTTP_INTERCEPTORS} from "@angular/common/http";
import {GoogleInterceptor} from "./google.http.interceptor";

/** Http interceptor providers in outside-in order */
export const httpInterceptorProviders = [
  // As you create new interceptors, add them to the httpInterceptorProviders array and you won't have to revisit the AppModule.
  { provide: HTTP_INTERCEPTORS, useClass: GoogleInterceptor, multi: true },
];
