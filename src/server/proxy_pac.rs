fn gen_js_host_if_condition(tgt_host: &[&str]) -> String {
    let mut conditions = Vec::new();

    for ip in tgt_host.iter() {
        conditions.push(format!("host === '{ip}'", ip = ip))
    }

    conditions.join(" || ")
}

pub fn gen_proxy_pac(url: &str, tgt_host: &[&str]) -> String {
    let mut buf = String::new();
    let if_cond = gen_js_host_if_condition(tgt_host);

    // SECTION: function FindProxyForURL(url, host) {
    buf.push_str("function FindProxyForURL(url, host) {\n");
    // SECTION:   if (<&if_cond>) {
    buf.push_str("\tif (");
    buf.push_str(&if_cond);
    buf.push_str(") {\n");
    // SECTION:     return 'PROXY <url>';
    buf.push_str("\t\treturn 'PROXY ");
    buf.push_str(url);
    buf.push_str("';\n");
    // SECTION:   }
    buf.push_str("\t}\n");
    // SECTION:   return 'DIRECT';
    buf.push_str("\treturn 'DIRECT';\n");
    // SECTION: }
    buf.push_str("}\n");

    buf
}

#[cfg(test)]
mod tests {
    mod gen_js_host_if_condition_test {
        use super::super::gen_js_host_if_condition;

        /// No any `||` with only 1 entry.
        #[test]
        fn no_any_or_with_only_one_entry() {
            let tgt_host = ["1.2.3.4"];
            assert_eq!(gen_js_host_if_condition(&tgt_host), "host === '1.2.3.4'");
        }

        /// Include `||` with more than 1 entries.
        #[test]
        fn include_or_with_more_than_one_entries() {
            let tgt_host = ["1.2.3.4", "5.6.7.8"];
            assert_eq!(
                gen_js_host_if_condition(&tgt_host),
                "host === '1.2.3.4' || host === '5.6.7.8'"
            );
        }
    }

    mod gen_proxy_pac_test {
        use super::super::gen_proxy_pac;

        #[test]
        fn gen_proxy_pac_test() {
            let tgt_host = ["1.2.3.4", "5.6.7.8"];
            assert_eq!(
                gen_proxy_pac("localhost:8080", &tgt_host),
                concat!(
                    "function FindProxyForURL(url, host) {\n",
                    "\tif (host === '1.2.3.4' || host === '5.6.7.8') {\n",
                    "\t\treturn 'PROXY localhost:8080';\n",
                    "\t}\n",
                    "\treturn 'DIRECT';\n",
                    "}\n",
                )
            );
        }
    }
}
