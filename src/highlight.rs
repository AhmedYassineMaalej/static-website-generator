use arborium::Highlighter as ArboriumHighlighter;
use arborium::advanced::Span;
use build_html::HtmlChild;
use build_html::HtmlElement;
use build_html::HtmlTag;

#[derive(Debug)]
struct Event<'a> {
    idx: usize,
    span: &'a Span,
    is_start: bool,
}

pub struct Highlighter;

impl Highlighter {
    pub fn highlight(
        code: &str,
        lang: &str,
        (mut line, mut column): (usize, usize),
    ) -> HtmlElement {
        let mut highlighter = ArboriumHighlighter::new();
        let mut element = HtmlElement::new(HtmlTag::PreformattedText);

        let Ok(mut spans) = highlighter.highlight_spans(lang, code) else {
            element.add_child(code.into());
            return element;
        };

        // remove duplicate spans: leave the one with the highest pattern index
        spans.sort_by_key(|span| (span.start, span.end, u32::MAX - span.pattern_index));
        spans.dedup_by_key(|span| (span.start, span.end));

        // transform spans to remove overlap
        let events = Self::get_events(&spans);
        let mut last_pos = 0;
        // keep track of current span and its parents in a stack
        let mut span_stack: Vec<&Span> = Vec::new();

        for event in events {
            // span end event
            if !event.is_start {
                let span = span_stack.pop().expect("tried to pop from empty stack");
                element.add_child(get_html_span(
                    &code[last_pos..event.idx],
                    Some(&span),
                    (&mut line, &mut column),
                ));
                last_pos = event.idx;
                continue;
            }

            // span start event

            if last_pos < event.idx {
                // handle text between two span starts
                // [  [ ]    [   ] ]
                //  ^^   ^^^^     ^
                element.add_child(get_html_span(
                    &code[last_pos..event.idx],
                    span_stack.last(),
                    (&mut line, &mut column),
                ));
                last_pos = event.idx;
            }
            span_stack.push(event.span);
        }

        assert!(span_stack.is_empty());

        if last_pos < code.len() {
            element.add_child(code[last_pos..].into());
        }

        element
    }

    fn get_events(spans: &'_ Vec<Span>) -> Vec<Event<'_>> {
        let mut events = Vec::new();
        for span in spans {
            events.push(Event {
                idx: span.start as usize,
                span,
                is_start: true,
            });

            events.push(Event {
                idx: span.end as usize,
                span,
                is_start: false,
            });
        }

        events.sort_by_key(|event| event.idx);
        events
    }
}

fn get_html_span(
    code: &str,
    span: Option<&&Span>,
    (line, column): (&mut usize, &mut usize),
) -> HtmlChild {
    let mut element = HtmlElement::new(HtmlTag::Span)
        .with_attribute("data-position", format!("{line}:{column}"))
        .with_child(code.into());

    if let Some(span) = span {
        element.add_attribute("class", span.capture.replace('.', " "));
    }

    update_position(line, column, code);

    element.into()
}

fn update_position(line: &mut usize, column: &mut usize, text: &str) {
    for char in text.chars() {
        *column += 1;
        if char == '\n' {
            *column = 1;
            *line += 1;
        }
    }
}
