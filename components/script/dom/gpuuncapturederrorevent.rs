/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use js::rust::HandleObject;
use servo_atoms::Atom;

use crate::dom::bindings::codegen::Bindings::EventBinding::EventBinding::EventMethods;
use crate::dom::bindings::codegen::Bindings::GPUUncapturedErrorEventBinding::{
    GPUUncapturedErrorEventInit, GPUUncapturedErrorEventMethods,
};
use crate::dom::bindings::codegen::Bindings::GPUValidationErrorBinding::GPUError;
use crate::dom::bindings::reflector::reflect_dom_object_with_proto;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::event::Event;
use crate::dom::globalscope::GlobalScope;

#[dom_struct]
pub struct GPUUncapturedErrorEvent {
    event: Event,
    #[ignore_malloc_size_of = "Because it is non-owning"]
    gpu_error: GPUError,
}

impl GPUUncapturedErrorEvent {
    fn new_inherited(init: &GPUUncapturedErrorEventInit) -> Self {
        Self {
            gpu_error: clone_gpu_error(&init.error),
            event: Event::new_inherited(),
        }
    }

    pub fn new(
        global: &GlobalScope,
        type_: DOMString,
        init: &GPUUncapturedErrorEventInit,
    ) -> DomRoot<Self> {
        Self::new_with_proto(global, None, type_, init)
    }

    fn new_with_proto(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        type_: DOMString,
        init: &GPUUncapturedErrorEventInit,
    ) -> DomRoot<Self> {
        let ev = reflect_dom_object_with_proto(
            Box::new(GPUUncapturedErrorEvent::new_inherited(init)),
            global,
            proto,
        );
        ev.event.init_event(
            Atom::from(type_),
            init.parent.bubbles,
            init.parent.cancelable,
        );
        ev
    }

    /// https://gpuweb.github.io/gpuweb/#dom-gpuuncapturederrorevent-gpuuncapturederrorevent
    #[allow(non_snake_case)]
    pub fn Constructor(
        global: &GlobalScope,
        proto: Option<HandleObject>,
        type_: DOMString,
        init: &GPUUncapturedErrorEventInit,
    ) -> DomRoot<Self> {
        GPUUncapturedErrorEvent::new_with_proto(global, proto, type_, init)
    }
}

impl GPUUncapturedErrorEvent {
    pub fn event(&self) -> &Event {
        &self.event
    }
}

impl GPUUncapturedErrorEventMethods for GPUUncapturedErrorEvent {
    /// https://gpuweb.github.io/gpuweb/#dom-gpuuncapturederrorevent-error
    fn Error(&self) -> GPUError {
        clone_gpu_error(&self.gpu_error)
    }

    /// https://dom.spec.whatwg.org/#dom-event-istrusted
    fn IsTrusted(&self) -> bool {
        self.event.IsTrusted()
    }
}

fn clone_gpu_error(error: &GPUError) -> GPUError {
    match *error {
        GPUError::GPUValidationError(ref v) => GPUError::GPUValidationError(v.clone()),
        GPUError::GPUOutOfMemoryError(ref w) => GPUError::GPUOutOfMemoryError(w.clone()),
    }
}
