# drshell
This project is for learning rust. It supports some basic built-in commands, pipeline, redirections and simple completions.
### functions
### built-in commands
the built-ins support the arg "-h" to show their descriptions.
* echo
* exit
* type
* pwd
* cd
* lsbuiltin
* history
### redirections
* \> or 1\> (redirect stdout to write to a new file)
* \>\> or 1\>\> (redirect stdout to append to a file)
* 2\> (redirect stderr to write to a new file)
* 2\>\> (redirect stdout to append to a file)
### pipeline
Use the operator "|" to create pipeline.
### warnings
The pipeline and redirections depend on the os threads, therefore, the installation path of this application needs to be added to the environment variable $PATH.
