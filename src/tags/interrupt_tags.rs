use error::{Error, Result};
use context::{Context, Interrupt};
use Token;
use LiquidOptions;
use Renderable;

struct Break;

impl Renderable for Break {
    fn render(&self, context: &mut Context) -> Result<Option<String>> {
        context.set_interrupt(Interrupt::Break);
        Ok(None)
    }
}

pub fn break_tag(_tag_name: &str,
                 arguments: &[Token],
                 _options: &LiquidOptions)
                 -> Result<Box<Renderable>> {

    // no arguments should be supplied, trying to supply them is an error
    if !arguments.is_empty() {
        return Error::parser("%}", arguments.first());
    }
    Ok(Box::new(Break))
}

struct Continue;

impl Renderable for Continue {
    fn render(&self, context: &mut Context) -> Result<Option<String>> {
        context.set_interrupt(Interrupt::Continue);
        Ok(None)
    }
}

pub fn continue_tag(_tag_name: &str,
                    arguments: &[Token],
                    _options: &LiquidOptions)
                    -> Result<Box<Renderable>> {
    // no arguments should be supplied, trying to supply them is an error
    if !arguments.is_empty() {
        return Error::parser("%}", arguments.first());
    }
    Ok(Box::new(Continue))
}


#[cfg(test)]
mod test {
    use Context;
    use LiquidOptions;
    use Renderable;
    use parse;

    #[test]
    fn test_simple_break() {
        let text = concat!("{% for i in (0..10) %}",
                           "enter-{{i}};",
                           "{% if i == 2 %}break-{{i}}\n{% break %}{% endif %}",
                           "exit-{{i}}\n",
                           "{% endfor %}");
        let template = parse(text, LiquidOptions::default()).unwrap();

        let mut ctx = Context::new();
        let output = template.render(&mut ctx);
        assert_eq!(output.unwrap(),
                   Some(concat!("enter-0;exit-0\n", "enter-1;exit-1\n", "enter-2;break-2\n")
                            .to_owned()));
    }

    #[test]
    fn test_nested_break() {
        // assert that a {% break %} only breaks out of the innermost loop
        let text = concat!("{% for outer in (0..3) %}",
                           "enter-{{outer}}; ",
                           "{% for inner in (6..10) %}",
                           "{% if inner == 8 %}break, {% break %}{% endif %}",
                           "{{ inner }}, ",
                           "{% endfor %}",
                           "exit-{{outer}}\n",
                           "{% endfor %}");
        let template = parse(text, LiquidOptions::default()).unwrap();

        let mut ctx = Context::new();
        let output = template.render(&mut ctx);
        assert_eq!(output.unwrap(),
                   Some(concat!("enter-0; 6, 7, break, exit-0\n",
                                "enter-1; 6, 7, break, exit-1\n",
                                "enter-2; 6, 7, break, exit-2\n")
                                .to_owned()));
    }

    #[test]
    fn test_simple_continue() {
        let text = concat!("{% for i in (0..5) %}",
                           "enter-{{i}};",
                           "{% if i == 2 %}continue-{{i}}\n{% continue %}{% endif %}",
                           "exit-{{i}}\n",
                           "{% endfor %}");
        let template = parse(text, LiquidOptions::default()).unwrap();

        let mut ctx = Context::new();
        let output = template.render(&mut ctx);
        assert_eq!(output.unwrap(),
                   Some(concat!("enter-0;exit-0\n",
                                "enter-1;exit-1\n",
                                "enter-2;continue-2\n",
                                "enter-3;exit-3\n",
                                "enter-4;exit-4\n")
                                .to_owned()));
    }

    #[test]
    fn test_nested_continue() {
        // assert that a {% continue %} only jumps out of the innermost loop
        let text = concat!("{% for outer in (0..3) %}",
                           "enter-{{outer}}; ",
                           "{% for inner in (6..10) %}",
                           "{% if inner == 8 %}continue, {% continue %}{% endif %}",
                           "{{ inner }}, ",
                           "{% endfor %}",
                           "exit-{{outer}}\n",
                           "{% endfor %}");
        let template = parse(text, LiquidOptions::default()).unwrap();

        let mut ctx = Context::new();
        let output = template.render(&mut ctx);
        assert_eq!(output.unwrap(),
                   Some(concat!("enter-0; 6, 7, continue, 9, exit-0\n",
                                "enter-1; 6, 7, continue, 9, exit-1\n",
                                "enter-2; 6, 7, continue, 9, exit-2\n")
                                .to_owned()));
    }


}
