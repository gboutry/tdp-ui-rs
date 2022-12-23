use std::rc::Rc;
use std::str::FromStr;

use js_sys::{JsString, Reflect};
use wasm_bindgen::JsValue;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use tree_sitter::{Language, Parser, Tree, TreeSitter};

#[derive(Properties, PartialEq)]
pub struct EditorProps {
    #[prop_or_default]
    pub content: AttrValue,
}

fn into_value<T>(err: T) -> JsValue
where
    T: Into<JsValue>,
{
    <T as Into<JsValue>>::into(err)
}

async fn initialize_parser() -> Result<Parser, JsValue> {
    log::debug!("TreeSitter initialization");
    TreeSitter::init().await.map_err(into_value)?;

    log::debug!("Language initialization");
    let language = Language::load_path("/assets/tree-sitter-yaml.wasm")
        .await
        .map_err(into_value)?;

    log::debug!("Parser initialization");
    let parser = Parser::new().map_err(into_value)?;

    log::debug!("Setting parser's language");
    parser
        .set_language(Some(language.as_ref()))
        .map_err(into_value)?;

    log::debug!("All good");
    Ok(parser)
}

pub enum EditorMessage {
    Initialize,
    Initialized(Parser),
    Error(JsValue),
    UpdateInput(String),
    UpdateContent(AttrValue),
    ContentUpdated,
    TreeUpdate,
    Nothing,
}

pub enum ParserState {
    NotInitialized,
    Initialized(Parser),
    Error(JsValue),
}

pub struct Editor {
    parser: ParserState,
    tree: Option<Tree>,
    content: AttrValue,
    parse_tree: Html,
}

impl Component for Editor {
    type Message = EditorMessage;

    type Properties = EditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(EditorMessage::Initialize);
        Self {
            parser: ParserState::NotInitialized,
            tree: None,
            content: ctx.props().content.clone(),
            parse_tree: html!(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EditorMessage::Initialize => {
                ctx.link().send_future(async {
                    match initialize_parser().await {
                        Ok(parser) => EditorMessage::Initialized(parser),
                        Err(err) => EditorMessage::Error(err),
                    }
                });
                self.parser = ParserState::NotInitialized;
                true
            }
            EditorMessage::Initialized(parser) => {
                let string = JsString::from_str(&ctx.props().content).unwrap();
                self.tree = match parser
                    .parse_with_string(&string, self.tree.as_ref(), None)
                    .map_err(into_value)
                {
                    Ok(tree) => tree,
                    Err(err) => {
                        ctx.link().send_message(EditorMessage::Error(err));
                        None
                    }
                };
                ctx.link().send_message(EditorMessage::TreeUpdate);
                self.parser = ParserState::Initialized(parser);

                true
            }
            EditorMessage::Error(err) => {
                let js_error = format!(
                    "{:?}",
                    Reflect::get(&err, &JsValue::from_str("message")).unwrap()
                );
                log::error!("{}", format!("{:?}", js_error));
                self.parser = ParserState::Error(err);
                true
            }
            EditorMessage::UpdateInput(input) => {
                let content = AttrValue::Rc(Rc::from(input));
                ctx.link()
                    .send_message(EditorMessage::UpdateContent(content));
                false
            }
            EditorMessage::UpdateContent(content) => {
                self.content = content;
                ctx.link().send_message(EditorMessage::ContentUpdated);
                true
            }
            EditorMessage::ContentUpdated => {
                if let ParserState::Initialized(parser) = &self.parser {
                    let parse_result = parser
                        .parse_with_string(
                            &JsString::from(self.content.as_str()),
                            None, // Better handle change of the tree
                            None,
                        )
                        .map_err(into_value);
                    if let Err(err) = parse_result {
                        ctx.link().send_message(EditorMessage::Error(err));
                        return true;
                    };
                    self.tree = parse_result.unwrap();
                    ctx.link().send_message(EditorMessage::TreeUpdate);
                    true
                } else {
                    log::error!("Parser is not initialized, cannot handle input change");
                    ctx.link().send_future({
                        let link = ctx.link().clone();
                        async move {
                            gloo_timers::callback::Timeout::new(1_000, move || {
                                link.send_message(EditorMessage::ContentUpdated);
                            })
                            .forget();
                            EditorMessage::Nothing
                        }
                    });
                    false
                }
            }
            EditorMessage::TreeUpdate => {
                self.parse_tree = render_tree(self.tree.as_ref().unwrap());
                true
            }
            EditorMessage::Nothing => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_input = ctx.link().callback(|e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlTextAreaElement>() {
                EditorMessage::UpdateInput(input.value())
            } else {
                EditorMessage::Nothing
            }
        });

        match &self.parser {
            ParserState::NotInitialized => {
                html! { "Parser non chargé" }
            }
            ParserState::Initialized(_parser) => {
                html! {
                    <>
                        <h1>{ "Parser chargé" }</h1>
                        <div class={classes!("flex", "flex-row", "h-screen")}>
                            <textarea class={classes!("border", "border-gray-400", "grow")} oninput={on_input} value={self.content.clone()}></textarea>
                            <div class={classes!("border", "overflow-auto", "border-gray-400", "w-1/3", "m-right-4")}>{ self.parse_tree.clone() }</div>
                        </div>
                    </>
                }
            }
            ParserState::Error(_err) => {
                html! {
                <>
                    <h1>{ "Error" }</h1>
                    <p>{ "Check console for details" }</p>
                </> }
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().content != old_props.content {
            ctx.link()
                .send_message(EditorMessage::UpdateContent(ctx.props().content.clone()));
            true
        } else {
            false
        }
    }
}

fn indent(n: usize) -> String {
    "&nbsp;".repeat(n * 2)
}

fn render_tree(tree: &Tree) -> Html {
    let mut row = String::new();
    let mut rows: Vec<String> = vec![];

    let cursor = tree.walk();
    let mut visited_children = false;
    let mut finished_row = false;
    let mut indent_level = 0;

    let column_spaced = JsString::from_str(": ").unwrap();
    let empty_string = JsString::from_str("").unwrap();
    let mut i = 0;
    loop {
        let mut display_name: Option<JsString> = None;
        if cursor.current_node().is_missing() {
            display_name = Some(JsString::from(format!("MISSING {}", cursor.node_type())))
        } else if cursor.node_is_named() {
            display_name = Some(cursor.node_type());
        }
        if visited_children {
            finished_row = visited_children && display_name.is_some();
            if cursor.goto_next_sibling() {
                visited_children = false;
            } else if cursor.goto_parent() {
                visited_children = true;
                indent_level -= 1;
            }
        } else {
            if display_name.is_some() {
                if finished_row {
                    row.push_str("</div>");
                    rows.push(row.clone());
                }
                let start = cursor.start_position();
                let end = cursor.end_position();
                let id = cursor.current_field_id().unwrap();

                let field_name = if let Some(field_name) = cursor.current_field_name() {
                    field_name.concat(&column_spaced)
                } else {
                    empty_string.clone()
                };
                let start_row = start.row();
                let start_column = start.column();
                let end_row = end.row();
                let end_column = end.column();
                let anchor = format!("<a class='plain' href='#' data-id={id} data-range=\"{start_row},{start_column},{end_row},{end_column}\">{}</a>", display_name.unwrap());
                row = format!("<div>{}{field_name}{anchor} [{start_row}, {start_column}] - [{end_row}, {end_column}]", indent(indent_level));
                finished_row = true;
            }
            if cursor.goto_first_child() {
                visited_children = false;
                indent_level += 1;
            } else {
                visited_children = true;
            }
        }
        if i > 3000 {
            break;
        }
        i += 1;
    }

    if finished_row {
        row.push_str("</div>");
        rows.push(row);
    }

    cursor.delete();

    let render = rows.join("\n");
    Html::from_html_unchecked(AttrValue::from(render))
}
