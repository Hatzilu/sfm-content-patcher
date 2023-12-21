package main

import (
	"fmt"
	"log"
	"os"
)

type customLogger struct {
	*log.Logger
	enabled bool
}

func CustomLogger(out *os.File, prefix string, flag int, enabled bool) *customLogger {
	return &customLogger{
		Logger:  log.New(out, prefix, flag),
		enabled: enabled,
	}
}

func (c *customLogger) Println(v ...interface{}) {
	if c.enabled {
		c.Logger.Println(v...)
	}
	// If logging is disabled, log using fmt instead.
	fmt.Println(v...)
}

func (c *customLogger) Fatal(v ...interface{}) {
	if c.enabled {
		c.Logger.Fatal(v...)
	}
	// If logging is disabled, log using fmt instead.
	fmt.Println(v...)
}