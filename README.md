# NODIFICATOR

### **Nodificator has no mercy!**

A script to recursively remove all node_modules throughout a given folder.
![nodificator](https://i.ibb.co/PN5vd42/nodificator.png)

### Build

Build the crate using cargo:

```
cargo build --release
```

### Usage:

Choose a directory that you want to free from the bourdon of node_modules and run :

```
./nodificator <path>
```

You can also type "h" or "--help" to access the commands available:

```
./nodificator h
//or
./nodificator --help
```

<br>

Todo:

- [ ] Improve README file
- [x] Re-Organize modules
- [x] Improve Error Handling
- [x] Add Help functionality
- [x] Add a counter that counts number of node_modules removed recursively.
