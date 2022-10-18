import { Injectable } from '@angular/core';

@Injectable({
  // Global service, provided in the root injector
  providedIn: 'root',
})
export class UserService {
  user = { isAuthorized: true };
}
