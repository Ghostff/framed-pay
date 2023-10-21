export class GatewayItem {
    public id: string;
    public route: { name: string, params: { id: string } };
    public title: string;
    public icon: string;

    public constructor(id: string, title: string, icon: string) {
        this.id = id;
        this.title = title;
        this.route = {name: 'integrations-vendor', params: {id}};
        this.icon = icon;
    }
}