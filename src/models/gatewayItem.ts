export class GatewayItem
{
    public id: string;
    public name: string;
    public icon: string;

    public constructor(id: string, name: string, icon: string) {
        this.id = id;
        this.name = name;
        this.icon = icon;
    }
}