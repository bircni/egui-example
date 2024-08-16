use egui::{DragValue, TextEdit, Ui};
use egui_form::garde::{field_path, GardeReport};
use egui_form::{Form, FormField};
use garde::Validate;

#[derive(Debug, Default, Validate)]
struct Fields {
    #[garde(length(min = 2, max = 50))]
    name: String,
    #[garde(skip)]
    age: i32,
}

pub struct EguiForm {
    fields: Fields,
}

impl EguiForm {
    pub fn new() -> Self {
        Self {
            fields: Fields {
                name: "Name".to_owned(),
                age: 33,
            },
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let mut form = Form::new().add_report(GardeReport::new(self.fields.validate(&())));
        FormField::new(&mut form, field_path!("name"))
            .label("User Name")
            .ui(ui, TextEdit::singleline(&mut self.fields.name));

        FormField::new(&mut form, field_path!("age"))
            .label("Age")
            .ui(ui, DragValue::new(&mut self.fields.age));

        if form.handle_submit(&ui.button("Submit"), ui) == Some(Ok(())) {
            println!("Submitted: {:?}", self.fields);
        }
    }
}
