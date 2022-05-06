package main

import (
	"fmt"
	"net/http"
	"os"

	spinhttp "github.com/fermyon/spin/sdk/go/http"
)

const serviceURLEnv = "SERVICE_URL"

func init() {
	spinhttp.Handle(func(w http.ResponseWriter, r *http.Request) {
		url := os.Getenv(serviceURLEnv)
		resp, err := spinhttp.Get(url)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Cannot send HTTP request to %v: %v", url, err)
			send404(w)
		}

		fmt.Fprintln(w, resp.Body)
	})
}

func send404(w http.ResponseWriter) {
	w.WriteHeader(http.StatusNotFound)
	w.Header().Add("content-type", "text/plain")
	w.Write([]byte("Not Found"))
}

func main() {}
