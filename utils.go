package main

import (
	"flag"
	"fmt"
	"os"
	"strings"
	"time"
)


func isExtractedFileRelevant(name string) bool {
	relevantFolders := [5]string{"maps", "models", "materials", "particles", "sound"}

	for _, folderPrefix := range relevantFolders {
		if strings.HasPrefix(name,folderPrefix) {
			return true
		}
	}
	return false
}


func initializeLogger(logger customLogger) {
	enableLogging := flag.Bool("l", false, "If set to true, a log file will be created when the program runs.")
	flag.Parse()
	
	
	if *enableLogging {
		logfile, err  := os.Create("sfm-content-patcher-"+time.Now().Format("2006-01-02-15-04-05")+".log")
	
		if err != nil {
			fmt.Println(err.Error())
		}
		defer logfile.Close()

		logger.SetOutput(logfile)
		logger.Println("logfile: ","sfm-content-patcher-"+time.Now().Format("2006-01-02-15-04-05")+".log")
	} else {
		logger.SetOutput(nil)
	}
}