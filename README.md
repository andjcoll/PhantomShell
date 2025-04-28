# PhantomShell

A mock CLI shell for reading and acting upon a virtual file system (VFS).

## Usage

Display the help message:

```cli
shell --help
```

Output the VFS:

```cli
shell -f <VFS>
```

List the files in the VFS:

```cli
shell -f <VFS> --dir
```

Print the content of a file (in VFS) to standard output:

```cli
shell -f <VFS> --cat <FILENAME>
```

Print the disk usage of the VFS:

```cli
shell -f <VFS> --du
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[GPL-3.0](https://choosealicense.com/licenses/gpl-3.0/)
