trait Button {
    fn press(&self);
    fn render(&self);
}

trait TextInput {
    fn set_text(&self, text: &str);
    fn get_text(&self) -> String;
}

struct WindowsButton;
impl Button for WindowsButton {
    fn press(&self) {
        println!("WindowsButton pressed");
    }

    fn render(&self) {
        println!("WindowsButton rendered");
    }
}

struct MacButton;
impl Button for MacButton {
    fn press(&self) {
        println!("MacButton pressed");
    }

    fn render(&self) {
        println!("MacButton rendered");
    }
}

struct WindowsTextInput;
impl TextInput for WindowsTextInput {
    fn set_text(&self, text: &str) {
        println!("WindowsTextInput set_text: {}", text);
    }

    fn get_text(&self) -> String {
        "WindowsTextInput".to_string()
    }
}

struct MacTextInput;
impl TextInput for MacTextInput {
    fn set_text(&self, text: &str) {
        println!("MacTextInput set_text: {}", text);
    }

    fn get_text(&self) -> String {
        "MacTextInput".to_string()
    }
}

trait GuiFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_text_input(&self) -> Box<dyn TextInput>;
}

struct WindowsFactory;
impl GuiFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_text_input(&self) -> Box<dyn TextInput> {
        Box::new(WindowsTextInput)
    }
}

struct MacFactory;
impl GuiFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_text_input(&self) -> Box<dyn TextInput> {
        Box::new(MacTextInput)
    }
}

fn main() {
    let factory: Box<dyn GuiFactory> = if cfg!(target_os = "windows") {
        Box::new(WindowsFactory)
    } else {
        Box::new(MacFactory)
    };

    factory.create_button().render();
    factory.create_button().press();

    factory.create_text_input().set_text("Hello, world!");
    println!("{}", factory.create_text_input().get_text());
}
