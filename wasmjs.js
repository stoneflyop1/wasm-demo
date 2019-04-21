export function hello() {
    return 'hello';
}

export class DemoJs {
    constructor() {
        this._id = 42;
    }

    get id() {
        return this._id;
    }

    set id(n) {
        this._id = n;
    }

    render() {
        return `Id: ${this.id}`;
    }
}