# touchpad
Nothing to see here. Like most other laptops mine also fails to disable my touchpad when typing. Instead of a full fetched solution that would fix the whole OS I took the lazy path of just speeding up my manual enable/disable process. For anyone suffering from the same issue the steps are 

## install
```
cargo install --path . # choose any other path you prefer or manually add to the PATH
```

## run
```
touchpad disable # disable the touchpad
touchpad enable # enable the touchpad
```

## known issues
This program will simply find the first device that has "touchpad" in the name (case insensitive). If you have multiple devices then updating the name in the source code should be the way to go. Since the ID of the device may change in every boot going via the name is practical but unique device identifiers are probably more practical.
