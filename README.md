# pixsee

An image viewer program.

## Building the project

**TODO** basic Meson installation instructions

### Flatpak

Make sure you have `flatpak` and `flatpak-builder` installed. Then run the commands below. Replace `<application_id>` with the value you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```
flatpak install org.gnome.Sdk//41 org.freedesktop.Sdk.Extension.rust-stable//21.08 org.gnome.Platform//41
flatpak-builder --user flatpak_app build-aux/<application_id>.Devel.json
```

## Running the project

### Flatpak

Once the project is build, run the command below. Replace Replace `<application_id>` and `<project_name>` with the values you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```
flatpak-builder --run flatpak_app build-aux/<application_id>.Devel.json <project_name>
```
