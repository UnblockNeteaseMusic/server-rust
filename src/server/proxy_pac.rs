fn gen_js_host_if_condition(tgt_host: &[&str]) -> String {
    let mut conditions = Vec::new();

    for ip in tgt_host {
        conditions.push(format!("host === '{ip}'", ip = ip))
    }

    conditions.join(" || ")
}

pub fn gen_proxy_pac(url: &str, tgt_host: &[&str]) -> String {
    let if_cond = gen_js_host_if_condition(tgt_host);

    format!(
        "\
function FindProxyForURL(url, host) {{
    if ({cond}) {{
        return 'PROXY {url}';
    }}
    
    return 'DIRECT';
}}
",
        cond = if_cond,
        url = url,
    )
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
                    "    if (host === '1.2.3.4' || host === '5.6.7.8') {\n",
                    "        return 'PROXY localhost:8080';\n",
                    "    }\n",
                    "    \n",
                    "    return 'DIRECT';\n",
                    "}\n",
                )
            );
        }
    }
}
