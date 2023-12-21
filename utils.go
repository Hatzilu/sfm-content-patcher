package main

import (
	"strings"
	"syscall"
)

func GetLogicalDrives() []string {

	kernel32, _ := syscall.LoadLibrary("kernel32.dll")
	getLogicalDrivesHandle, _ := syscall.GetProcAddress(kernel32, "GetLogicalDrives")

	var drives []string

	if ret, _, callErr := syscall.SyscallN(uintptr(getLogicalDrivesHandle), 0, 0, 0, 0); callErr != 0 {
		// handle error
	} else {
		drives = bitsToDrives(uint32(ret))
	}

	return drives

}

func isExtractedFileRelevant(name string) bool {
	relevantFolders := [5]string{"maps", "models", "materials", "particles", "sound"}

	for _, folderPrefix := range relevantFolders {
		if strings.HasPrefix(name,folderPrefix) {
			return true
		}
	}
	return false
}

func bitsToDrives(bitMap uint32) (drives []string) {
    availableDrives := []string{"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"}

    for i := range availableDrives {
        if bitMap & 1 == 1 {
            drives = append(drives, availableDrives[i])
        }
        bitMap >>= 1
    }

    return
}