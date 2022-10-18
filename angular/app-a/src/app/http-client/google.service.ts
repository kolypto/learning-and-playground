import { Injectable } from '@angular/core';
import {Observable} from "rxjs";
import {HttpClient, HttpResponse} from "@angular/common/http";
import {map, tap} from "rxjs/operators";

@Injectable({
  providedIn: 'root',
})
export class GoogleService {
  /** Load all data from the "users" table
   */
  loadUsers(): Observable<User[]> {
    return this.loadSheetData().pipe(
      map(response => response.body.values),
      map(table => {
        return table.slice(1).map(row => {
          return {
            id: row[0],
            name: row[1],
          };
        });
      })
    );
  }

  /** Add a user to the table back-end
   * @param user
   */
  addUser(user: User){
    return this.appendSheetRows([
      [user.id, user.name],
    ]).pipe(
      tap(res => console.log(res))
    );
  }

  // API keys to access Google APIs
  GOOGLE_MAPS_API_KEY = 'AIzaSyAHrdiIYSDhPbkBut5_9D6xo7nfs9ozDJ4';
  // A spreadsheet that contains named ranges:
  // users (id, name)
  GOOGLE_SHEET_ID = '1x-EVd0YLD3H2FzUM5IYT0Pt7MLoIwW6uXOrd5To0lII';
  // Google spreadsheet API url
  GOOGLE_SHEET_API_URL = `https://sheets.googleapis.com/v4/spreadsheets/${this.GOOGLE_SHEET_ID}`;

  /** Add a row to the table
   */
  appendSheetRows(rows: SheetData): Observable<HttpResponse<GoogleSheetValuesResponse>>{
    return this.http.post<GoogleSheetValuesResponse>(
      this._googleSheetUrl('/values/users', 'append', {
        insertDataOption: 'INSERT_ROWS',
        includeValuesInResponse: 'true',
        valueInputOption: 'RAW',
        alt: 'json'
      }),
      {
        majorDimension: 'ROWS',
        values: rows,
      },
      {observe: "response"},
    );
  }

  /** Load all data from a sheet
   */
  loadSheetData(): Observable<HttpResponse<GoogleSheetValuesResponse>> {
    return this.http.get<GoogleSheetValuesResponse>(
      this._googleSheetUrl('/values/users'),
      { observe: 'response' }
    );
  }

  /** Build a URL to a Google Spreadsheet API
   * @param uri
   * @param action
   * @param params
   */
  _googleSheetUrl(uri: string, action: string = null, params: {[key:string]: string} = {}) {
    // URL params
    params.key = this.GOOGLE_MAPS_API_KEY;
    let url_params = new URLSearchParams(params);
    // URL string
    let url = this.GOOGLE_SHEET_API_URL;
    if (uri) url += uri;
    if (action) url += `:${action}`;
    return url + '?' + url_params.toString();
  }

  constructor(private http: HttpClient) {}
}

export interface User {
  id: string;
  name: string;
}

interface GoogleSheetValuesResponse {
  range: string;
  majorDimension: string;
  values: string[][];
}

type SheetData = Array<Array<string>>;
