let overlay;
let sizer;
let positioner;

let headerContainer;
let dismissButton;

let bodyContainer;
let bodyTitle;
let bodyContent;

let footerContainer;
let cancelBtn;
let okBtn;

let success, info, warning, danger;

function awake(): void {
    overlay = createElement('z-[9999] absolute w-full top-0 bottom-0 right-0 backdrop-blur-lg')
    sizer = createElement('ease-out transition-all sm:mx-auto sm:w-full m-3');
    positioner = createElement('relative flex flex-col bg-white shadow-lg rounded-xl dark:bg-gray-800');

    headerContainer = createElement('absolute top-2 right-2');
    dismissButton = createElement('inline-flex flex-shrink-0 justify-center items-center h-8 w-8 rounded-md text-gray-500 hover:text-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white transition-all text-sm dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800', 'button');
    dismissButton.innerHTML = `<svg class="w-3.5 h-3.5" width="8" height="8" viewBox="0 0 8 8" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M0.258206 1.00652C0.351976 0.912791 0.479126 0.860131 0.611706 0.860131C0.744296 0.860131 0.871447 0.912791 0.965207 1.00652L3.61171 3.65302L6.25822 1.00652C6.30432 0.958771 6.35952 0.920671 6.42052 0.894471C6.48152 0.868271 6.54712 0.854471 6.61352 0.853901C6.67992 0.853321 6.74572 0.865971 6.80722 0.891111C6.86862 0.916251 6.92442 0.953381 6.97142 1.00032C7.01832 1.04727 7.05552 1.1031 7.08062 1.16454C7.10572 1.22599 7.11842 1.29183 7.11782 1.35822C7.11722 1.42461 7.10342 1.49022 7.07722 1.55122C7.05102 1.61222 7.01292 1.6674 6.96522 1.71352L4.31871 4.36002L6.96522 7.00648C7.05632 7.10078 7.10672 7.22708 7.10552 7.35818C7.10442 7.48928 7.05182 7.61468 6.95912 7.70738C6.86642 7.80018 6.74102 7.85268 6.60992 7.85388C6.47882 7.85498 6.35252 7.80458 6.25822 7.71348L3.61171 5.06702L0.965207 7.71348C0.870907 7.80458 0.744606 7.85498 0.613506 7.85388C0.482406 7.85268 0.357007 7.80018 0.264297 7.70738C0.171597 7.61468 0.119017 7.48928 0.117877 7.35818C0.116737 7.22708 0.167126 7.10078 0.258206 7.00648L2.90471 4.36002L0.258206 1.71352C0.164476 1.61976 0.111816 1.4926 0.111816 1.36002C0.111816 1.22744 0.164476 1.10028 0.258206 1.00652Z" fill="currentColor"/>
            </svg>`;

    bodyContainer = createElement('p-4 sm:p-8 text-center overflow-y-auto');
    bodyTitle = createElement('mb-2 text-2xl font-bold text-gray-800 dark:text-gray-200', 'h3');
    bodyContent = createElement('text-gray-500', 'p');

    footerContainer = createElement('flex items-center');
    cancelBtn = createElement('p-4 w-full font-bold inline-flex justify-center items-center gap-2 rounded-bl-xl bg-gray-100 border border-transparent font-semibold text-gray-800 hover:text-blue-600 focus:outline-none focus:ring-2 ring-offset-white focus:ring-gray-100 focus:ring-offset-2 transition-all text-sm dark:bg-gray-700 dark:hover:bg-gray-600 dark:focus:ring-gray-600 dark:text-white dark:focus:ring-offset-gray-800', 'button');
    okBtn = createElement('', 'button');



    const alert = createElement('mb-4 inline-flex justify-center items-center w-[62px] h-[62px] rounded-full border-4', 'span')
    function makeAlert(color: string, icon: string): HTMLElement {
        const element = alert.cloneNode();
        element.classList.add(`border-${color}-50`, `bg-${color}-100`, `text-${color}-500`, `dark:bg-${color}-700`, `dark:border-${color}-600`, `dark:text-${color}-100`);
        element.innerHTML = icon;

        return element;
    }

    success = makeAlert('green', `<svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
              <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-3.97-3.03a.75.75 0 0 0-1.08.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-.01-1.05z"/>
      </svg>`);

    info = makeAlert('blue', `<svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
            <path d="M8 16A8 8 0 1 0 8 0a8 8 0 0 0 0 16zm.93-9.412-1 4.705c-.07.34.029.533.304.533.194 0 .487-.07.686-.246l-.088.416c-.287.346-.92.598-1.465.598-.703 0-1.002-.422-.808-1.319l.738-3.468c.064-.293.006-.399-.287-.47l-.451-.081.082-.381 2.29-.287zM8 5.5a1 1 0 1 1 0-2 1 1 0 0 1 0 2z"/>
     </svg>`);

    warning = makeAlert('yellow', `<svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
            <path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/>
      </svg>`);

    danger = makeAlert('red', `<svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
            <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zM5.354 4.646a.5.5 0 1 0-.708.708L7.293 8l-2.647 2.646a.5.5 0 0 0 .708.708L8 8.707l2.646 2.647a.5.5 0 0 0 .708-.708L8.707 8l2.647-2.646a.5.5 0 0 0-.708-.708L8 7.293 5.354 4.646z"/>
     </svg>`);
}


window.addEventListener('load', awake);

function createElement(className: string, tag: string = 'div'): HTMLElement {
    const element = document.createElement(tag);
    element.className = className;
    if (tag === 'button') {
        element.setAttribute('type', 'button');
    }

    return element;
}

interface DialogConfig {
    title?: string; // dialog title
    body?: string; // dialog body
    type?: string; // dialog type. success | info | warning | danger (changes action btn color to red)
    closable?: boolean; // show close (X) icon at top right
    size?: string; // dialog size. sm | undefined (default md)
    isCancellable?: boolean; // show cancel action button
    actionRequired?: boolean; // hide action keys
    backdropBackground?: number; // dialog overlay background color
    onCancel?: () => void;  // cancel action click callback handler
    onOk?: () => void|boolean; // ok action click callback handler
    disableBackDropInteraction?: boolean; // dont close dialog on backdrop click.
    okText?: string; // text displayed on okay action button (default: Ok)
    cancelText?: string; // text displayed on cancel action button (default: Cancel)
}

export default function dialog(config: DialogConfig) {
    if (overlay === undefined) {
        awake(); // fail safe | vue hot reload.
    }

    function close(triggerEvent: boolean = true) {
        overlay.parentNode.removeChild(overlay);
        if (triggerEvent) {
            config.onCancel();
        }
    }


    positioner.textContent = '';
    if (config.disableBackDropInteraction !== true) {
        overlay.onclick = close;
    }

    // add header
    {
        headerContainer.textContent = '';
        if (config.closable !== false) {
            dismissButton.onclick = close;
            headerContainer.appendChild(dismissButton)
        }

        positioner.appendChild(headerContainer)
    }

    let confirmBtnCls = ['bg-indigo-500', 'hover:bg-indigo-600', 'focus:ring-indigo-500'];

    // add body
    {
        bodyContainer.textContent = '';
        if (config.title !== undefined) {
            bodyTitle.innerHTML = config.title;
        }

        if (config.body !== undefined) {
            bodyContent.innerHTML = config.body;
        }

        if (config.type === 'info') {
            bodyContainer.appendChild(info);
        } else if (config.type === 'warning') {
            bodyContainer.appendChild(warning);
        } else if (config.type === 'success') {
            bodyContainer.appendChild(success);
        } else if (config.type === 'danger') {
            bodyContainer.appendChild(danger);
            confirmBtnCls = ['bg-red-500', 'hover:bg-red-600', 'focus:ring-red-500'];
        }

        bodyContainer.appendChild(bodyTitle);
        bodyContainer.appendChild(bodyContent);
        positioner.appendChild(bodyContainer);
    }

    // add footer
    if (config.actionRequired !== false) {
        footerContainer.textContent = '';
        if (config.isCancellable !== true) {
            cancelBtn.innerHTML = config.cancelText || 'Cancel';
            cancelBtn.onclick = close;
            footerContainer.appendChild(cancelBtn);
        }

        okBtn.onclick = () => {
            if (config.onOk() !== false) {
                close(false);
            }
        }
        okBtn.className = 'p-4 w-full font-bold inline-flex justify-center items-center gap-2 rounded-br-xl border border-transparent font-semibold text-white focus:outline-none focus:ring-2 focus:ring-offset-2 transition-all text-sm dark:focus:ring-offset-gray-800';
        okBtn.classList.add(...confirmBtnCls)
        okBtn.innerHTML = config.okText || 'Ok'
        footerContainer.appendChild(okBtn);
        positioner.appendChild(footerContainer);
    }

    if (config.backdropOpacity !== undefined) {
        overlay.style.opacity = config.backdropOpacity.toString();
    }

    if (config.size === 'sm') {
        sizer.classList.add('sm:max-w-xs')
    } else {
        sizer.classList.add('sm:max-w-lg')
    }

    overlay.style.background = config.backdropBackground || 'rgb(0 0 0 / .6)';
    sizer.appendChild(positioner)
    overlay.appendChild(sizer)
    document.body.appendChild(overlay);
}