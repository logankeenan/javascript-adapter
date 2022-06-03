let onAnchorClicked, onFormSubmission, hasRegisteredDocumentEvents;

async function documentClickHandler(event) {
    const href = event.target.href;
    const origin = window.location.origin;

    if (event.target.tagName === "A" && href.startsWith(origin)) {
        event.preventDefault();

        const url = href.replace(origin, "");
        history.pushState(undefined, undefined, url);

        event.preventDefault();
        await onAnchorClicked(href);
    }

    // TODO handle other submit form events INPUT
    if (event.target.tagName === "BUTTON" && event.target.type === "submit") {
        return;
    }

    event.preventDefault();
}

async function formSubmitHandler(event) {
    const formData = new FormData(event.target);
    const bodyEncoded = new URLSearchParams(formData).toString();
    const url = event.target.action;
    const method = event.target.method;

    event.preventDefault();

    await onFormSubmission(url, method, bodyEncoded, event.target.encoding);
}

export function initialize(eventHandlers) {
    onAnchorClicked = eventHandlers.onAnchorClicked;
    onFormSubmission = eventHandlers.onFormSubmission;

    if (!hasRegisteredDocumentEvents) {
        document.addEventListener('click', documentClickHandler);
        window.addEventListener('popstate', eventHandlers.onPopState)
        hasRegisteredDocumentEvents = true;
    }

    document.querySelectorAll("form").forEach((form) =>
        form.addEventListener("submit", formSubmitHandler)
    );
}

