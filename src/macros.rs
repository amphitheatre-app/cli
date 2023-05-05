// Copyrgiht 2023 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

macro_rules! display {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        println!(
            "{}",
            format!($($arg)*)
                .if_supports_color(owo_colors::Stream::Stdout, |text| text.bold())
        );
    }};
}

macro_rules! critical {
    ($($arg:tt)*) => {{
        use owo_colors::OwoColorize;
        eprintln!(
            "{}",
            format!($($arg)*)
                .if_supports_color(owo_colors::Stream::Stderr, |text| text.bright_red())
        );
    }};
}

macro_rules! define_display_macro {
    ($name:ident, $level:ident, $style:ident, $d:tt) => {
        macro_rules! $name {
                            ($d($d arg:tt)*) => {{
                                use owo_colors::OwoColorize;
                                if log::Level::$level <= *$crate::app::verbosity() {
                                    eprintln!(
                                        "{}",
                                        format!($d($d arg)*)
                                            .if_supports_color(owo_colors::Stream::Stderr, |text| text.$style())
                                    );
                                }
                            }};
                        }
    };
}

define_display_macro!(trace, Trace, underline, $);
define_display_macro!(debug, Debug, italic, $);
define_display_macro!(info, Info, bold, $);
define_display_macro!(success, Info, bright_cyan, $);
define_display_macro!(waiting, Info, bright_magenta, $);
define_display_macro!(warn, Warn, bright_yellow, $);
define_display_macro!(error, Error, bright_red, $);
