import morphdom from "morphdom";
import {onBeforeElUpdated, onNodeAdded} from "./morphdom-options";

let wasm_app, JsRequest, JsResponse;

export function initialize(options) {
    wasm_app = options.app;
    JsRequest = options.JsRequest;
    JsResponse = options.JsResponse;
}

async function makeRequest(jsRequest) {
    const jsResponse = await wasm_app(jsRequest);

    if (jsResponse.status_code === "302") {
        const url = `${window.location.origin}${jsResponse.headers["location"]}`;
        return makeRequest(new JsRequest(url, "GET"))
    }

    morphdom(document.documentElement, jsResponse.body, {
        onBeforeElUpdated,
        onNodeAdded
    })
}


async function documentClickHandler(event) {
    const href = event.target.href;
    const origin = window.location.origin;

    if (event.target.tagName === "A" && href.startsWith(origin)) {
        event.preventDefault();

        const url = href.replace(origin, "");
        history.pushState(undefined, undefined, url);

        await makeRequest(new JsRequest(href, "GET"))
    }

    // TODO handle other submit form events INPUT
    if (event.target.tagName === "BUTTON" && event.target.type === "submit") {
        return;
    }

    event.preventDefault();
}

function formSubmitHandler(event) {

    const formData = new FormData(event.target);
    const bodyEncoded = new URLSearchParams(formData).toString();
    const url = event.target.action;
    const method = event.target.method;

    const jsRequest = new JsRequest(url, method);
    jsRequest.body = bodyEncoded;
    jsRequest.headers_append("Content-Type", event.target.encoding)

    makeRequest(jsRequest)

    event.preventDefault();
}

function handleBackNavigation(event) {
    const jsRequest = new JsRequest(window.location.href, "GET");
    makeRequest(jsRequest);
}

export function registerEvents() {
    document.addEventListener('click', documentClickHandler);
    document.querySelectorAll("form").forEach((form) =>
        form.addEventListener("submit", formSubmitHandler)
    );
    window.addEventListener('popstate', handleBackNavigation)
}

export function pageLoad() {
    const jsRequest = new JsRequest(window.location.href, "GET");
    makeRequest(jsRequest)
}
