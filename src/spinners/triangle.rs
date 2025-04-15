/*
 * Copyright (c) David Lin
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use crate::types::Validation;
use leptos::prelude::*;
use std::process;

/// Renders a square and rotates it immediately on
/// the screen.
///
/// Takes in required props for both color (#hex) and size (by px)
///
/// # Example Usage
/// ```
/// use leptos::prelude::*;
/// use spux::spinners::Triangle;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Triangle color="#000000" size=10 />
///     }
/// }
/// ```

#[component]
pub fn Triangle(color: &'static str, size: u32) -> impl IntoView {
    // we intentionally create a compile time process exit to inform users
    // that an incorrect #hex color code was used
    let Ok(_) = color.validate_color_code() else {
        eprintln!("Spux Error - Color should be in the #hex format (e.g #000000) for black");
        process::exit(1);
    };

    let border_color = format!("{}px solid {}", &size * 2, color);
    let half_size = size / 2;
    let style = format!(
        "
            .spux-spinner-triangle {{
                background: transparent;
                border: {size}px solid transparent;
                border-top: 0;
                display:flex;
                width: 0px;
                height: 0px;
                transform: rotate(45deg);
                animation: spux-spinner-triangle 1400ms ease-in-out forwards;
                animation-iteration-count: infinite;
            }}

            @keyframes spux-spinner-triangle  {{
                0% {{
                    transform: rotate(45deg);
                    margin-top:0px;
                    margin-right:0px;
                }}
                50% {{
                    transform: rotate(270deg);
                    margin-top:0px;
                    margin-right:{half_size}px;
                }}
                100% {{
                    transform: rotate(405deg);
                    margin-top:0px;
                    margin-right:0px;
                }}
            }}
        "
    );
    view! {
        <style>
            {style}
        </style>
        <div class="spux-spinner-triangle"
            style:border-bottom=border_color
        ></div>
    }
}
