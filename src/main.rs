use leptos::prelude::*;

mod components;

fn main() {
    mount_to_body(components::counter::Counter);
}
