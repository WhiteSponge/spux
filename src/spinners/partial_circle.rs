/*
 * Copyright (c) David Lin
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use crate::types::Validation;
use leptos::prelude::*;
use std::process;

/// Renders a partial circle and rotates it immediately on
/// the screen.
///
/// Takes in required props for both color (#hex) and size (by px)
///
/// # Example Usage
/// ```
/// use leptos::prelude::*;
/// use spux::spinners::PartialCircle;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <PartialCircle color="#000000" size=10 />
///     }
/// }
/// ```

#[component]
pub fn PartialCircle(color: &'static str, size: u32) -> impl IntoView {
    // we intentionally create a compile time process exit to inform users
    // that an incorrect #hex color code was used
    let Ok(_) = color.validate_color_code() else {
        eprintln!("Spux Error - Color should be in the #hex format (e.g #000000) for black");
        process::exit(1);
    };

    let border_color = format!("transparent {color} {color} {color}");
    let style = format!(
        "
        .spux-partial-circle {{
            background: transparent;
            display:flex;
            width: {size}px;
            height: {size}px;
            border-radius: 100%;
            transform: rotate(45deg);
            animation: spux-partial-circle 1400ms linear forwards;
            animation-iteration-count: infinite;
        }}

        @keyframes spux-partial-circle {{
            0% {{
                transform: rotate(45deg);
            }}
            50% {{
                transform: rotate(270deg);
            }}
            100% {{
                transform: rotate(405deg);
            }}
        }}
    "
    );

    view! {
        <style>
            {style}
        </style>
        <div class="spux-partial-circle"
            style:border="2px solid"
            style:border-color=border_color
        ></div>
    }
}
