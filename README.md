# increfile
if a file exists already, write to file2, or file3, etc.

```
[nix-shell:~/code/increfile]$ cp ./target/debug/increfile .
[nix-shell:~/code/increfile]$ echo "foo" | ./increfile "bar"
[nix-shell:~/code/increfile]$ echo "foo" | ./increfile "bar"
[nix-shell:~/code/increfile]$ echo "foo" | ./increfile "bar"
[nix-shell:~/code/increfile]$ echo "foo" | ./increfile "bar"
[nix-shell:~/code/increfile]$ echo "foo" | ./increfile "bar"
[nix-shell:~/code/increfile]$ ls bar*
bar  bar2  bar3  bar4  bar5
[nix-shell:~/code/increfile]$ cat bar
foo
[nix-shell:~/code/increfile]$ 
```
