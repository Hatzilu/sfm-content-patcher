package main

import (
	"errors"
	"fmt"
	"os"
	"path"
)

func main() {
	partialTfDir := path.Join("common","Team Fortress 2")
	partialSfmDir := path.Join("common","Team Fortress 2")

	tf2Dir, tf2DirErr := detectDirectory(partialTfDir)
	if tf2DirErr != nil {
		fmt.Println("Unable to detect tf2 directory")
		panic(tf2DirErr)

	}
	sfmDir, sfmDirErr := detectDirectory(partialSfmDir)
	if sfmDirErr != nil {
		fmt.Println("Unable to detect SFM directory")
		panic(sfmDirErr)

	}
	os.DirFS("/")
	fmt.Println(tf2Dir)
	fmt.Println(sfmDir)
	// fmt.Print("Type a number: ")
	// fmt.Scan(&i)
	// fmt.Println("Your number is:", i)
}


func detectDirectory(partialPath string) (string, error) {
	// fmt.Println(os.DirFS("/"))
	// exePath, exeErr := os.Executable()
	// if exeErr != nil {
	// 	panic(exeErr)
	// }

	systemDrives := GetLogicalDrives()

	for _, drive := range systemDrives {
		drive += ":"
		dirPath := path.Join(drive,"Program Files (x86)","Steam","steamapps",partialPath)
		fmt.Println("checking path ", dirPath)
	
		_, err := os.Stat(dirPath)
		if err == nil {
			return dirPath, nil
		}
	
		dirPath = path.Join(drive,"SteamLibrary","steamapps",partialPath)
		fmt.Println("checking path ", dirPath)
	
		_ ,err = os.Stat(dirPath)
		if err == nil {
			return dirPath, nil
		}
	}

	// recursively check for the 
	return "",errors.New("Directory not found")
}

