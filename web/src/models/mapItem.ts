export class MapItem<T = string|number>
{
    public label: string;
    public value: T;

    public constructor(value: T, label: string) {
        this.value = value;
        this.label = label;
    }
}