/* "Barrel" of Http Interceptors */
import {HttpErrorResponse, HttpEvent, HttpHandler, HttpInterceptor, HttpRequest} from '@angular/common/http';
import {Injectable} from "@angular/core";
import {Observable} from "rxjs";
import {tap} from "rxjs/operators";

/** Handle errors
 */
@Injectable()
export class GoogleInterceptor implements HttpInterceptor {
  constructor() {}

  intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    return next.handle(req).pipe(
      tap(
        null,
        (error: HttpErrorResponse) => {
          window.alert(error.error.error.message);
        }
        ),
    );
  }
}
