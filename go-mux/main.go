package main

import (
	"fmt"
	"net/http"
	"net/http/httputil"

	"github.com/gorilla/mux"

	spinhttp "github.com/fermyon/spin/sdk/go/http"
)

func helloHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Hello, Fermyon from a Spin component written in Go!!")
}

func dumpRequest(w http.ResponseWriter, r *http.Request) {
	dump, err := httputil.DumpRequest(r, true)
	if err != nil {
		http.Error(w, fmt.Sprint(err), http.StatusInternalServerError)
		return
	}

	fmt.Fprintln(w, string(dump))
}

func init() {
	r := mux.NewRouter()
	r.HandleFunc("/go-mux", helloHandler)
	r.HandleFunc("/go-mux/request", dumpRequest)
	spinhttp.Handle(r.ServeHTTP)
}

func main() {}
