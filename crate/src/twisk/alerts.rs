use crate::{generated::css_classes::C, Msg};
use comp_state::{topo, use_state};
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

#[topo::nested]
pub fn alert<T: Into<String>>(color: T, contents: T) -> Node<Msg> {
    let bg_col = format!("bg-{}-500", color.into());

    let (show_alert, show_alert_access) = use_state(|| true);

    if show_alert {
        div![
            class![C.text_white C.px_6 C.py_4 C.border_0 C.rounded C.relative C.mb_4 bg_col.as_ref() ],
            span![
                class!(C.text_xl C.inline_block C.mr_5 C.align_middle),
                i![class!("fas" "fa-bell")]
            ],
            span![
                class![C.inline_block C.align_middle C.mr_8],
                contents.into()
            ],
            button![
                class![C.absolute C.bg_transparent C.text_2xl C.font_semibold C.leading_none C.right_0 C.top_0 C.mt_4 C.mr_6 C.outline_none C.focus__outline_none],
                on_click(move |_| show_alert_access.set(false)),
                span!["x"]
            ]
        ]
    } else {
        empty![]
    }
}
