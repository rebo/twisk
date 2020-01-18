use crate::{generated::css_classes::C, Msg};
use comp_state::{topo, use_state};
use enclose::enclose as e;
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

#[topo::nested]
pub fn modal(contents: Node<Msg>) -> Node<Msg> {
    let (show_modal, show_modal_access) = use_state(|| false);

    div![
        button![
            class![C.bg_pink_500 C.text_white C.font_bold C.uppercase C.text_sm C.px_6 C.py_3 C.rounded C.shadow C.hover__shadow_lg C.outline_none C.focus__outline_none C.mr_1 C.mb_1],
            attrs![At::Type=>"button"],
            style![St::Transition =>"all .15s ease"],
            on_click(
                e!((show_modal_access) move |_| show_modal_access.set(true))
            ),
            "Open regular modal"
        ],
        if !show_modal {
            vec![empty![]]
        } else {
            vec![
          div![
            class![C.justify_center C.items_center C.flex C.overflow_x_hidden C.overflow_y_auto C.fixed C.inset_0 C.z_50 C.outline_none C.focus__outline_none],
            on_click(e!((show_modal_access) move |_| show_modal_access.set(false))),
            div![
                class!["relative w-auto my-6 mx-auto max-w-3xl"],
              //content
              div![
                    class!["border-0 rounded-lg shadow-lg relative flex flex-col w-full bg-white outline-none focus:outline-none"],
                    // header
                    div![ class!["flex items-start justify-between p-5 border-b border-solid border-gray-300 rounded-t"],
                  h3![class!["text-3xl font-semibold"],
                    "Modal Title"
                  ],
                  button![
                    class!["p-1 ml-auto bg-transparent border-0 text-black opacity-5 float-right text-3xl leading-none font-semibold outline-none focus:outline-none"],
                    on_click(e!((show_modal_access) move |_| show_modal_access.set(false))),
                    span![
                        class!["bg-transparent text-black opacity-5 h-6 w-6 text-2xl block outline-none focus:outline-none"],
                      "×"
                    ]
                  ],
                    ],
                div![ class!["relative p-6 flex-auto"],
                  p![class!["my-4 text-gray-600 text-lg leading-relaxed"],
                   contents
                  ],
                ],
                //footer
                div![
                    class!["flex items-center justify-end p-6 border-t border-solid border-gray-300 rounded-b"],
                  button![
                    class!["text-red-500 background-transparent font-bold uppercase px-6 py-2 text-sm outline-none focus:outline-none mr-1 mb-1"],
                    attrs![At::Type => "button"],
                    style![St::Transition =>"all .15s ease"],
                    on_click(e!((show_modal_access) move |_| show_modal_access.set(false))),
                    "Close"],
                    button![
                    class![C.bg_green_500 C.text_white C.font_bold C.uppercase C.text_sm C.px_6 C.py_3 C.rounded C.shadow C.hover__shadow_lg C.outline_none C.focus__outline_none C.mr_1 C.mb_1],
                    attrs![At::Type => "button"],
                    style![St::Transition =>"all .15s ease"],
                    on_click(e!((show_modal_access) move |_| show_modal_access.set(false))),
                    "Save Changes"
                  ]
                ]
              ]
          ]
        ],
        div![ class!["opacity-20 fixed inset-0 z-40 bg-black"]]]
        }
    ]
}
