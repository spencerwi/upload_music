[ ] Static "hello, please upload a file!" page  
[x] Allow uploading zip files -- need to understand any security concerns  
[ ] Auth?  
[~] Configuration file to define upload location and directory/filename pattern to follow for each audio file detected in the zip file  
	[X] Reading config file  
	[X] Sane solution for fallback/default values -- using serde default annotation, and throwing ConfigError for required keys
	[ ] Reading config files from OS-standard locations (like XDG maybe?)
[ ] Build the piece that reads audio file metadata (start with MP3, maybe add Ogg/AAC/FLAC? support)  
[ ] Build the piece that uses audio file metadata and the pattern from the config file to determine output file structure  
