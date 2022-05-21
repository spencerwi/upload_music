[ ] Static "hello, please upload a file!" page  
[x] Allow uploading zip files -- need to understand any security concerns  
[ ] Auth?  
[X] Configuration file to define upload location and directory/filename pattern to follow for each audio file detected in the zip file  
	[X] Reading config file  
	[X] Sane solution for fallback/default values -- using serde default annotation, and throwing ConfigError for required keys
	[X] Reading config files from OS-standard locations (like XDG maybe?)
[X] Build the piece that reads audio file metadata (start with MP3, maybe add Ogg/AAC/FLAC? support)  
[X] Build the piece that uses audio file metadata and the pattern from the config file to determine output file structure  
[X] Change the zip-file extraction so that it iterates over all files, grabs audio files only, and runs them through the file-renamer engine to determine where to extract them.
[ ] Figure out the best way to extract track number, since that's not always present
[ ] DECENT RESPONSE MESSAGES
[ ] More tests!
