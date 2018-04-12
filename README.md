### Rust Bracket Inserter

This is a little script to take care of one of the more tedious tasks I have to do here at work. I wrote this one of these in Javascript, but I figured it would be good practice for me to try this in Rust. We'll see how it goes.

I generate these txt files and then have to go in and add variables (these are provided in a JIRA ticket) add double-curleys to the variables (because the variables from the ticket aren't written correctly) and then rename the text file.

So, this script can be run and will generate a new txt file in the directory where it found the txt file.

So for example if you type in

```
cargo run
```

within the src folder, the new txt file will be in the Downloads folder.

The script asks for two things:
* the name of the file (it assumes the path to the file is to the Downloads foler)
* the extra parts of the new name (this takes the form of blah_blah_blah_  make sure you put that last underscore there)

So, for example, if you have a data collection tag called lexus_20180406.txt and you run the script, you answer "lexus_20180406.txt" for the first question and "com_data_collection_tag_" for the second. You will get a new file called lexus_com_data_collection_tag_20180406.txt with the double-curley brackets inserted (if there are p variables...if there are no p variables, then it just renames the file);