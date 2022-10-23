#[forbid(unsafe_code)]
pub mod model;

// todo: mj-include

#[cfg(test)]
mod test {
    use crate::model::mj_accordion::{
        MjAccordion, MjAccordionElement, MjAccordionElementText, MjAccordionElementTitle,
    };
    use crate::model::mj_body::{MjBody, MjBodyAttributes};
    use crate::model::mj_column::MjColumn;
    use crate::model::mj_section::MjSection;
    use crate::model::mjml::Mjml;
    use crate::model::SizePx;

    #[test]
    fn basic_test() {
        let mjml = Mjml {
            mj_raw_file_start: None,
            mj_body: MjBody {
                content: vec![
                    Box::new(MjSection{
                        attributes: Default::default(),
                        content: vec![
                            Box::new(MjColumn{
                                content: vec![
                                    Box::new(MjAccordion{
                                        elements: vec![
                                            MjAccordionElement{
                                                title: MjAccordionElementTitle {
                                                    content: "Why use an accordion?".to_string(),
                                                    attributes: Default::default()
                                                },
                                                text: MjAccordionElementText {
                                                    content: "<span style=\"line-height:20px\">Because emails with a lot of content are most of the time a very bad experience on mobile, mj-accordion comes handy when you want to deliver a lot of information in a concise way.</span>".to_string(),
                                                    attributes: Default::default()
                                                },
                                                attributes: Default::default()
                                            },
                                            MjAccordionElement{
                                                title: MjAccordionElementTitle {
                                                    content: "How it works".to_string(),
                                                    attributes: Default::default()
                                                },
                                                text: MjAccordionElementText {
                                                    content: "<span style=\"line-height:20px\">Content is stacked into tabs and users can expand them at will. If responsive styles are not supported (mostly on desktop clients), tabs are then expanded and your content is readable at once.</span>".to_string(),
                                                    attributes: Default::default()
                                                },
                                                attributes: Default::default()
                                            },
                                        ],
                                        attributes: Default::default()
                                    })
                                ],
                                attributes: Default::default()
                            })
                        ]
                    })
                ],
                attributes: MjBodyAttributes{
                    background_color: None,
                    width: SizePx::new(600),
                    css_class: vec!["test_body".to_string()]
                }
            },
            mj_head: None,
            attributes: Default::default()
        };

        let rendered = mjml.render_to_string().unwrap();

        println!("{}", rendered);

        assert_eq!(rendered, "");
    }
}
