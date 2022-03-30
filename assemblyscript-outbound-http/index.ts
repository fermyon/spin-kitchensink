// @ts-ignore
import * as wasi from "as-wasi";
import {
  Method,
  RequestBuilder,
} from "@deislabs/wasi-experimental-http";

export function _start(): void {
    wasi.Console.log("content-type: text/plain\n");
    
    let res = new RequestBuilder("http://localhost:3000/static/important.txt").method(Method.GET).send();
    wasi.Console.log(String.UTF8.decode(res.bodyReadAll().buffer));
}
