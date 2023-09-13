export class MapItem
{
    public label: string;
    public value: string|number;

    public constructor(value: string|number, label: string) {
        this.value = value;
        this.label = label;
    }
}