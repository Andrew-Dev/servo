/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::codegen::Bindings::StyleSheetListBinding;
use crate::dom::bindings::codegen::Bindings::StyleSheetListBinding::StyleSheetListMethods;
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::{Dom, DomRoot};
use crate::dom::document::Document;
use crate::dom::stylesheet::StyleSheet;
use crate::dom::window::Window;
use dom_struct::dom_struct;

#[dom_struct]
pub struct StyleSheetList {
    reflector_: Reflector,
    document: Dom<Document>,
}

impl StyleSheetList {
    #[allow(unrooted_must_root)]
    fn new_inherited(doc: Dom<Document>) -> StyleSheetList {
        StyleSheetList {
            reflector_: Reflector::new(),
            document: doc,
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(window: &Window, document: Dom<Document>) -> DomRoot<StyleSheetList> {
        reflect_dom_object(
            Box::new(StyleSheetList::new_inherited(document)),
            window,
            StyleSheetListBinding::Wrap,
        )
    }
}

impl StyleSheetListMethods for StyleSheetList {
    // https://drafts.csswg.org/cssom/#dom-stylesheetlist-length
    fn Length(&self) -> u32 {
        self.document.stylesheet_count() as u32
    }

    // https://drafts.csswg.org/cssom/#dom-stylesheetlist-item
    fn Item(&self, index: u32) -> Option<DomRoot<StyleSheet>> {
        // XXXManishearth this  doesn't handle the origin clean flag and is a
        // cors vulnerability
        self.document
            .stylesheet_at(index as usize)
            .map(DomRoot::upcast)
    }

    // check-tidy: no specs after this line
    fn IndexedGetter(&self, index: u32) -> Option<DomRoot<StyleSheet>> {
        self.Item(index)
    }
}
