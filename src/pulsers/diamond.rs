/*
 * Copyright (c) David Lin
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
use crate::types::Validation;
use leptos::prelude::*;
use std::process;

/// Renders a diamond and pulsates it immediately on
/// the screen.
///
/// Takes in required props for both color (#hex) and size (by px)
///
/// # Example Usage
/// ```
/// use leptos::prelude::*;
/// use spux::pulsers::Diamond;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Diamond color="#000000" size=10 />
///     }
/// }
/// ```

#[component]
pub fn Diamond(color: &'static str, size: u32) -> impl IntoView {
    // we intentionally create a compile time process exit to inform users
    // that an incorrect #hex color code was used
    let Ok(_) = color.validate_color_code() else {
        eprintln!("Spux Error - Color should be in the #hex format (e.g #000000) for black");
        process::exit(1);
    };

    let half_size = size / 2;
    let margin_size = size / 8;

    let style = format!(
        "
        .spux-diamond {{
            background: transparent;
            display:flex;
            width: {size}px;
            height: {size}px;
            margin-top:0px;
            margin-left:0px;
            transform: rotate(45deg);
            animation: spux-diamond 1400ms ease-in-out forwards;
            animation-iteration-count: infinite;
        }}

        @keyframes spux-diamond {{
            0% {{
                width:{size}px;
                height:{size}px;
                margin-top:0px;
                margin-left:0px;
            }}
            50% {{
                width:{half_size}px;
                height:{half_size}px;
                margin-top:{margin_size}px;
                margin-left:{margin_size}px;
            }}
            100% {{
                width:{size}px;
                height:{size}px;
                margin-top:0px;
                margin-left:0px;
            }}
        }}
    "
    );
    view! {
        <style>
            {style}
        </style>
        <div class="spux-diamond"
            style:border="1px solid"
            style:border-color=color
        ></div>
    }
}
