use crate::{generated::css_classes::C, twisk::*, Msg};
use comp_state::{topo, use_state};
use seed::{prelude::*, *};
use seed_comp_helpers::on_click;

#[topo::nested]
pub fn view() -> Node<Msg> {
    div![
        menus::pink_menu(
            "Tailwind Starter Kit in Seed",
            &[
                ("Sample item 1", "#one"),
                ("Sample item 2", "#two"),
                ("Sample item 3", "#three")
            ]
        ),
        div![
            class![C.flex C.flex_row C.mb_4],
            div![class![C.w_1of6  C.text_right C.pr_1], "Circular Image:"],
            div![
                class![C.w_5of6],
                images::circle_raised_image("static/images/back2.jpg", "..."),
            ]
        ],
        div![
            class![C.flex C.flex_row C.mb_4],
            div![class![C.w_1of6 C.text_right C.pr_1], "Tabs Example:"],
            div![class![C.w_5of6], tabs::tabs_3(tab_a(), tab_b(), tab_c()),]
        ],
        div![
            class![C.flex C.flex_row C.mb_4],
            div![class![C.w_1of6 C.text_right C.pr_1], "Dropdown:"],
            div![
                class![C.w_5of6],
                dropdowns::dropdown(
                    "pink",
                    "white",
                    "A dropdown",
                    &[("one", "#one"), ("two", "#two"), ("three", "#three")]
                ),
            ]
        ],
        div![
            class![C.flex C.flex_row C.mb_4],
            div![class![C.w_1of6 C.text_right C.pr_1], "Alert:"],
            div![
                class![C.w_5of6],
                alerts::alert("pink", "This is an a alert - check it out!"),
            ]
        ],
        div![
            class![C.flex C.flex_row],
            div![class![C.w_1of6 C.text_right C.pr_1], "Other Image Styles:"],
            div![
                class![C.flex C.flex_row C.w_5of6],
                images::simple_image("static/images/front.jpg", "..."),
                images::circle_image("static/images/back2.jpg", "..."),
                images::simple_raised_image("static/images/front.jpg", "..."),
            ]
        ],
        div![
            class![C.flex C.flex_row C.mb_4],
            div![class![C.w_1of6 C.text_right C.pr_1], "Small Modal Example:"],
            div![class![C.w_5of6], modals::modal(div!["This is a modal"]),]
        ],
        div![
            class![C.flex C.flex_row C.mb_4],
            div![class![C.w_1of6 C.text_right C.pr_1], "Popover Example::"],
            div![class![C.w_5of6], popovers::popover(),]
        ],
    ]
}

#[topo::nested]
fn tab_a() -> (String, Node<Msg>) {
    let (count, count_access) = use_state(|| 0);
    let content = div![
        button![
            class![C.p_1, C.m_2 C.block C.rounded, C.bg_pink_600, C.text_white],
            format!("Clicked {} times", count),on_click(move |_| count_access.set(count+1))],
            "Tempor nulla duis non nisi tempor in cillum magna pariatur magna irure. Exercitation eu sit culpa exercitation cillum veniam proident nulla duis incididunt sit exercitation officia Lorem. Dolor irure deserunt anim mollit eu ipsum elit cillum sunt consequat ullamco labore occaecat. In ex sunt incididunt ullamco commodo. Reprehenderit qui aliquip ex anim in amet veniam reprehenderit occaecat veniam enim deserunt id labore.",
    ];
    ("Title 1".to_string(), content)
}

fn tab_b() -> (String, Node<Msg>) {
    ("Title 2".to_string(), div!["Nulla cillume ex dolore est sunt fugiat. Proident magna mollit duis voluptate eu laboris ipsum adipisicing est pariatur sint officia. Ad do et incididunt aliquip est ut incididunt cillum incididunt ea reprehenderit reprehenderit. Officia sunt enim mollit nisi anim occaecat mollit."])
}

#[topo::nested]
fn tab_c() -> (String, Node<Msg>) {
    let (count, count_access) = use_state(|| 0);
    let content = div![
        button![
            class![C.p_1, C.m_2 C.block C.rounded, C.bg_pink_600, C.text_white],
            format!("Clicked {} times", count),on_click(move |_| count_access.set(count+1))],
        "Dolor ea dolor tempor laborum et do reprehenderit sit incididunt minim duis et ex Lorem. Consectetur irure dolor qui fugiat quis ex culpa officia sunt nulla velit id minim laboris. Anim sit ipsum culpa laboris veniam elit irure occaecat ea id ipsum.",
    ];

    ("Title 3".to_string(), content)
}
