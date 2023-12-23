package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"path"
	"path/filepath"
	"strings"
	"time"

	"github.com/NublyBR/go-vpk"
)

func main() {
	
	required_vpk_files := [4]string{"tf2_misc_dir.vpk", "tf2_textures_dir.vpk", "tf2_sound_misc_dir.vpk", "tf2_sound_vo_english_dir.vpk"}


	// Initialize logger according to -l flag
	enableLogging := flag.Bool("l", false, "If set to true, a log file will be created when the program runs.")
	flag.Parse()
	logger := CustomLogger(os.Stdout, "INFO: ", log.Ldate|log.Ltime, *enableLogging)
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


	// tf2Dir, tf2DirErr := detectDirectory(partialTfDir, logger)
	// if tf2DirErr != nil {
	// 	logger.Println("Unable to detect tf2 directory")
	// 	panic(tf2DirErr)

	// }

	var tf2Dir string
	fmt.Println("Please paste your \"Team Fortress 2\" directory path: ")
	fmt.Scanln(&tf2Dir)
	var sfmDir string
	fmt.Println("Please paste your \"SourceFilmmaker\" directory path: ")
	fmt.Scanln(&sfmDir)
	for i := range required_vpk_files {
		required_vpk_files[i] = path.Join(tf2Dir,"tf",required_vpk_files[i])
	}
	
	// sfmDir, sfmDirErr := detectDirectory(partialSfmDir, logger)
	// if sfmDirErr != nil {
	// 	logger.Println("Unable to detect SFM directory")
	// 	panic(sfmDirErr)
		
	// }
	// logger.Println("TF2 directory detected at ", tf2Dir)
	// logger.Println("SFM directory detected at ", sfmDir)
	
	// Extract vpk files
	for _, file := range required_vpk_files {	
		logger.Println("Extracting vpk ",file)
		
		pak, err := vpk.OpenDir(file) 
		if err != nil {
			logger.Fatal(err)
			panic(err)
		}

		defer pak.Close()

        for _, file := range pak.Entries() {
			isIrrelevantFile := isExtractedFileRelevant(file.Filename()) == false

			if isIrrelevantFile {
				continue
			}

			entry, err := file.Open()
			if err != nil {
				logger.Fatal(err)
				panic(err)
			}

			path := filepath.Join(sfmDir,"game","tf",file.Filename())

			existingFile, err := os.Stat(path)
			if err == nil && strings.Contains(file.Filename(), existingFile.Name()) {
				logger.Println("File already exists ", existingFile.Name())
				continue
			}

			// Ensure the directories exist by using os.MkdirAll
			dir := filepath.Dir(path)
			if dir_err := os.MkdirAll(dir, 0755); dir_err != nil {
					panic(dir_err)
			}
			logger.Println("Extracting file to",path)
			writeErr := ExtractVpkFile(entry, path)
			if writeErr != nil {
				logger.Fatal(err)
				panic(err)
			}
		}
	}
}


