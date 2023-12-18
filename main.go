package main

import (
	"errors"
	"fmt"
	"os"
	"path"
)

func main() {
	
	required_vpk_files := [4]string{"tf2_misc_dir.vpk", "tf2_textures_dir.vpk", "tf2_sound_misc_dir.vpk", "tf2_sound_vo_english_dir.vpk"}

	// Detect needed directories
	partialTfDir := path.Join("common","Team Fortress 2")
	partialSfmDir := path.Join("common","SourceFilmmaker")

	tf2Dir, tf2DirErr := detectDirectory(partialTfDir)
	if tf2DirErr != nil {
		fmt.Println("Unable to detect tf2 directory")
		panic(tf2DirErr)

	}
	for i := range required_vpk_files {
		required_vpk_files[i] = path.Join(tf2Dir,"tf",required_vpk_files[i])
	}
	fmt.Println(required_vpk_files)

	sfmDir, sfmDirErr := detectDirectory(partialSfmDir)
	if sfmDirErr != nil {
		fmt.Println("Unable to detect SFM directory")
		panic(sfmDirErr)

	}
	fmt.Println("TF2 directory detected at ", tf2Dir)
	fmt.Println("SFM directory detected at ", sfmDir)
	// fmt.Print("Type a number: ")
	// fmt.Scan(&i)
	// fmt.Println("Your number is:", i)
}


func detectDirectory(partialPath string) (string, error) {
	systemDrives := GetLogicalDrives()

	for _, drive := range systemDrives {
		drive += ":"
		dirPath := path.Join(drive,"Program Files (x86)","Steam","steamapps",partialPath)
	
		_, err := os.Stat(dirPath)
		if err == nil {
			return dirPath, nil
		}
	
		dirPath = path.Join(drive,"SteamLibrary","steamapps",partialPath)
	
		_ ,err = os.Stat(dirPath)
		if err == nil {
			return dirPath, nil
		}
	}

	// recursively check for the 
	return "",errors.New("Directory not found")
}

