import morphdom from "morphdom";
import {onBeforeElUpdated, onNodeAdded} from "./morphdom-options";

let wasmApp, JsRequest, JsResponse, hasRegisteredDocumentEvents

export function create(options) {
    wasmApp = options.app;
    JsRequest = options.JsRequest;
    JsResponse = options.JsResponse;

    return {
        pageLoaded: pageLoaded,
        start: loadCurrentPage
    }
}

async function makeRequest(jsRequest) {
    const jsResponse = await wasmApp(jsRequest);

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

function handleBackNavigation() {
    const jsRequest = new JsRequest(window.location.href, "GET");
    makeRequest(jsRequest);
}

function pageLoaded() {
    if (!hasRegisteredDocumentEvents) {
        document.addEventListener('click', documentClickHandler);
        window.addEventListener('popstate', handleBackNavigation)
        hasRegisteredDocumentEvents = true;
    }

    document.querySelectorAll("form").forEach((form) =>
        form.addEventListener("submit", formSubmitHandler)
    );
}

async function loadCurrentPage() {
    const jsRequest = new JsRequest(window.location.href, "GET");
    await makeRequest(jsRequest)
}

