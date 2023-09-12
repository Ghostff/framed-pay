type closure = (item: MenuItem) => void

export class MenuItem<T = unknown> {
  public action: T
  public label: string
  public iconName?: string
  public iconClass                    = ''
  public isParent                     = false
  public isRoute                      = false
  public id: string | null            = null
  public onClick: closure | undefined = void 0

  public constructor(label: string, action: T, iconName?: string) {
    this.action   = action
    this.label    = label
    this.iconName = iconName
  }

  public static route(label: string, routeName: unknown, iconName?: string) {
    const self   = new MenuItem(label, routeName, iconName)
    self.isRoute = true

    return self
  }

  public static group(name: string, menus: Array<MenuItem>): MenuItem {
    const self    = new MenuItem(name, menus)
    self.isParent = true

    return self
  }

  public icon(iconName: string, className = ''): MenuItem {
    this.iconName  = iconName
    this.iconClass = className

    return this
  }

  public static action<T>(label: string, value: T, onClick: closure, iconName?: string) {
    const self   = new MenuItem(label, value, iconName)
    self.onClick = onClick

    return self
  }
}
