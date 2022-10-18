export class Hero {
  constructor(
    public id: number,
    public name: string,
    public deleted: boolean = false,
    public power?: string,
    public alterEgo?: string,
  ) { }
}
