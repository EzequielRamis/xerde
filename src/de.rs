use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct XmlParser;

#[cfg(test)]
mod parser {
    use super::*;
    use test_case::test_case;

    #[test_case(Rule::element,"<xml:a>\n    <b/>foo<bar>foo</bar>\n</a>"; "simple nested element")]
    #[test_case(Rule::element,"<a><b/><![CDATA[<greeting>Hello, world!</greeting>]]><bar>foo</bar></a>"; "nested element with cdata")]
    #[test_case(Rule::element,"<a><b/><!-- comment --><bar>foo</bar></a>"; "nested element with comment")]
    fn valid(r: Rule, s: &str) {
        let res = XmlParser::parse(r, s).expect("unsuccesful valid parse");
        println!("{}: {:?}\n", res.as_str(), res);
    }

    // fn invalid(r: Rule, s: &str) {
    //     let err = XmlParser::parse(r, s).expect_err("unsuccesful invalid parse");
    //     println!("{}", err);
    // }
}
