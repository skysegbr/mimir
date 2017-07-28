//! `mimiron` macros

macro_rules! or_none {
    ($st:expr) => {
        $st.unwrap_or_else(|| "none".to_string())
    };
    (b => $st:expr) => {
        $st.unwrap_or_else(|| false)
    }
}

macro_rules! try_join {
    ($st:expr, $optlist:expr) => {
        if let Some(list) = $optlist {
            if !list.is_empty() {
                $st = String::from("[ ");
                $st.push_str(&list.join(","));
                $st.push_str(" ]");
            }
        }
    };
}
