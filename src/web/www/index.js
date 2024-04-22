///////// HELPER FUNCS
const createElem = (elemName, content) => {
    const elem = document.createElement(elemName);
    elem.innerHTML = content;
    return elem;
};

const P = (content) => createElem("p", content);
const H2 = (content) => createElem("h2", content);
const H3 = (content) => createElem("h3", content);
const TEXT_INPUT = (content) => {
    let elem = document.createElement("input");
    elem.value = content;
    elem.type = "text";
    return elem;
};

///////// ELEMENTS
const tabs = document.getElementById("tabs");
const tabContainer = document.getElementById("tab-container");


///////// SITE SETUP
const createTree = async (parent, data) => {
    console.log(data);
    if (data.uri) {
        let deviceData = await fetch(`/device-info/${data.uri}`);
        let deviceJson = await deviceData.json();
        const container = document.createElement("div");
        container.classList.add("device");
        parent.appendChild(container);

        let name = deviceJson.name ? deviceJson.name : "Unknown name";
        let nameElem = H2(name);
        container.appendChild(nameElem);

        if (deviceJson.actions) {
            for (key in deviceJson.actions) {
                container.appendChild(H3(key));

                let form = document.createElement("form");

                form.appendChild(TEXT_INPUT(deviceJson.actions[key].target));
                form.appendChild(TEXT_INPUT(deviceJson.actions[key].code));

                container.appendChild(form);
            }
        }
    }
};

const setupSite = async () => {
    const setup = await fetch("/get-setup");
    const body = await setup.json();

    if (body.unsorted) {
        const tab = document.createElement("button");
        tab.innerText = "Unsorted";
        tabs.appendChild(tab);

        const tabView = document.createElement("div");
        for (child of body.unsorted) {
            await createTree(tabView, child);
        }
        tabContainer.appendChild(tabView);
    }
};

setupSite();


///////// ACTIVE FORM-ATRON
const formUpdate = (form) => {
    let data = new FormData(form);

    let object = {};
    data.forEach((value, key) => object[key] = value);
    let uri = object.uri;
    delete object.uri;
    let json = JSON.stringify(object);

    fetch("publish-device", {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ uri: uri, body: json })
    })
        .then(response => console.log(response));
};

const deviceForms = document.getElementsByClassName("device-form");
for (let i = 0; i < deviceForms.length; i++) {
    deviceForms[i].addEventListener("input", () => formUpdate(deviceForms[i]));
}