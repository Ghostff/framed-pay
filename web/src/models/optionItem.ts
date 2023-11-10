import {MapItem} from "@/models/mapItem";

export class OptionItem<T = string|number> extends MapItem<T>
{
    public description?: string;

    public constructor(value: T, label: string, description?: string) {
        super(value, label);
        this.description = description;
    }
}