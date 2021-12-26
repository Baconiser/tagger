# tagger
Commandline tool for tagging windows folders and files

It creates a "tags" folder in your %HOME% dir and adds it to the windows `QuickAccess`

![image](https://user-images.githubusercontent.com/6066027/147416327-a5acca47-5e8e-4241-ab70-420ff82b9de6.png)


The tag is a file in the tags folder, the target dir/file is just a symlink. 

![image](https://user-images.githubusercontent.com/6066027/147416437-4f7b3073-3fe3-4a31-bffd-e5f8353ce41c.png)

usage: 

`tagger.exe C://targetdir` 
It then prompts you to enter the tag.

If the tag exists in the Tag folder it adds the symlink otherwise the tag will be created as a directory and the symlink created.

I added it to my rightclick menu for simpler usage.
