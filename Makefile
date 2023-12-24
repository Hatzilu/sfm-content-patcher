build:
	go generate
	go build -o sfm.exe

generate-icon:
	rsrc -ico logo.ico -o icon.syso
