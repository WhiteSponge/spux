/*
 * Copyright (c) David Lin
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use crate::types::Validation;
use leptos::prelude::*;
use std::process;

/// Renders a filled square and rotates it immediately on
/// the screen.
///
/// Takes in required props for both color (#hex) and size (by px)
///
/// # Example Usage
/// ```
/// use leptos::prelude::*;
/// use spux::spinners::FilledSquare;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <FilledSquare color="#000000" size=10 />
///     }
/// }
/// ```

#[component]
pub fn FilledSquare(color: &'static str, size: u32) -> impl IntoView {
    // we intentionally create a compile time process exit to inform users
    // that an incorrect #hex color code was used
    let Ok(_) = color.validate_color_code() else {
        eprintln!("Spux Error - Color should be in the #hex format (e.g #000000) for black");
        process::exit(1);
    };

    let style = format!(
        "
        .spux-spinner-filled-square {{
            display:flex;
            width: {size}px;
            height: {size}px;
            transform: rotate(45deg);
            animation: spux-spinner-filled-square 1400ms ease-in-out forwards;
            animation-iteration-count: infinite;
        }}

        @keyframes spux-spinner-filled-square  {{
            0% {{
                transform: rotate(45deg);
            }}
            50% {{
                transform: rotate(90deg);
            }}
            100% {{
                transform: rotate(135deg);
            }}
        }}

    "
    );

    view! {
        <style>
            {style}
        </style>
        <div class="spux-spinner-filled-square"
            style:background-color=color
        ></div>
    }
}
