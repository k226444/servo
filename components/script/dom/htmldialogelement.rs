/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use html5ever::{local_name, namespace_url, ns, LocalName, Prefix};
use js::rust::HandleObject;

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::HTMLDialogElementBinding::HTMLDialogElementMethods;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::document::Document;
use crate::dom::element::Element;
use crate::dom::eventtarget::EventTarget;
use crate::dom::htmlelement::HTMLElement;
use crate::dom::node::{window_from_node, Node};

#[dom_struct]
pub struct HTMLDialogElement {
    htmlelement: HTMLElement,
    return_value: DomRefCell<DOMString>,
}

impl HTMLDialogElement {
    fn new_inherited(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> HTMLDialogElement {
        HTMLDialogElement {
            htmlelement: HTMLElement::new_inherited(local_name, prefix, document),
            return_value: DomRefCell::new(DOMString::new()),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
        proto: Option<HandleObject>,
    ) -> DomRoot<HTMLDialogElement> {
        Node::reflect_node_with_proto(
            Box::new(HTMLDialogElement::new_inherited(
                local_name, prefix, document,
            )),
            document,
            proto,
        )
    }
}

impl HTMLDialogElementMethods for HTMLDialogElement {
    // https://html.spec.whatwg.org/multipage/#dom-dialog-open
    make_bool_getter!(Open, "open");

    // https://html.spec.whatwg.org/multipage/#dom-dialog-open
    make_bool_setter!(SetOpen, "open");

    // https://html.spec.whatwg.org/multipage/#dom-dialog-returnvalue
    fn ReturnValue(&self) -> DOMString {
        let return_value = self.return_value.borrow();
        return_value.clone()
    }

    // https://html.spec.whatwg.org/multipage/#dom-dialog-returnvalue
    fn SetReturnValue(&self, return_value: DOMString) {
        *self.return_value.borrow_mut() = return_value;
    }

    // https://html.spec.whatwg.org/multipage/#dom-dialog-close
    fn Close(&self, return_value: Option<DOMString>) {
        let element = self.upcast::<Element>();
        let target = self.upcast::<EventTarget>();
        let win = window_from_node(self);

        // Step 1 & 2
        if element
            .remove_attribute(&ns!(), &local_name!("open"))
            .is_none()
        {
            return;
        }

        // Step 3
        if let Some(new_value) = return_value {
            *self.return_value.borrow_mut() = new_value;
        }

        // TODO: Step 4 implement pending dialog stack removal

        // Step 5
        win.task_manager()
            .dom_manipulation_task_source()
            .queue_simple_event(target, atom!("close"), &win);
    }
}
