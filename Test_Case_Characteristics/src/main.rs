use syn::{Item, Type};
use syn::visit::{self, Visit};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use csv::Writer;
use walkdir::WalkDir;

struct NodeCollector {
    types: HashMap<String, usize>,
    items: HashMap<String, usize>,
}

impl NodeCollector {
    fn new() -> Self {
        NodeCollector {
            types: HashMap::new(),
            items: HashMap::new(),
        }
    }

    fn get_types(&self) -> HashMap<String, usize> {
        self.types.clone()
    }

    fn get_items(&self) -> HashMap<String, usize> {
        self.items.clone()
    }
}

impl<'ast> Visit<'ast> for NodeCollector {
    fn visit_type(&mut self, ty: &'ast Type) {
        let type_name = get_type_kind(&ty); // get type information
        *self.types.entry(type_name).or_insert(0) += 1;
        visit::visit_type(self, ty);
    }

    fn visit_item(&mut self, item: &'ast syn::Item) {
        let item_name = get_item_kind(&item); // get item information
        *self.items.entry(item_name).or_insert(0) += 1;
        visit::visit_item(self, item);
    }
}

fn get_type_kind(ty: &Type) -> String{
    match ty {
        Type::Array(_) => return "Array".to_string(),
        Type::BareFn(_) => return "BareFn".to_string(),
        Type::Group(_) => return "Group".to_string(),
        Type::ImplTrait(_) => return "ImplTrait".to_string(),
        Type::Infer(_) => return "Infer".to_string(),
        Type::Macro(_) => return "Macro".to_string(),
        Type::Never(_) => return "Never".to_string(),
        Type::Paren(_) => return "Paren".to_string(),
        Type::Path(_) => return "Path".to_string(),
        Type::Ptr(_) => return "Ptr".to_string(),
        Type::Reference(_) => return "Reference".to_string(),
        Type::Slice(_) => return "Slice".to_string(),
        Type::TraitObject(_) => return "TraitObject".to_string(),
        Type::Tuple(_) => return "Tuple".to_string(),
        Type::Verbatim(_) => return "Verbatim".to_string(),
        _ => return "Other".to_string(),
    }
}


fn get_item_kind(item: &Item) -> String {
    match item {
        Item::Fn(_) => return "Function".to_string(),
        Item::Struct(_) => return "Struct".to_string(),
        Item::Enum(_) => return "Enum".to_string(),
        Item::Trait(_) => return "Trait".to_string(),
        Item::Impl(_) => return "Impl".to_string(),
        Item::Mod(_) => return "Mod".to_string(),
        Item::Static(_) => return "Static".to_string(),
        Item::Const(_) => return "Const".to_string(),
        Item::Type(_) => return "Type".to_string(),
        Item::ExternCrate(_) => return "ExternCrate".to_string(),
        Item::ForeignMod(_) => return "ForeignMod".to_string(),
        Item::Macro(_) => return "Macro".to_string(),
        Item::TraitAlias(_) => return "TraitAlias".to_string(),
        Item::Union(_) => return "Union".to_string(),
        Item::Use(_) => return "Use".to_string(),
        Item::Verbatim(_) => return "Verbatim".to_string(),
        _ => return "Other".to_string(),
    }
}

// Process a single file, collect its type and project information, and write to a CSV file
fn process_file(path: &Path, item_wtr: &mut Writer<File>, item_headers: &Vec<String>, type_wtr: &mut Writer<File>, type_headers: &Vec<String>) {
    let path_str = path.to_str().unwrap_or("");

    // Open the file
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {:?}: {}", path, err);
            return;
        }
    };

    // Read the file content.
    let mut content = String::new();
    if let Err(err) = file.read_to_string(&mut content) {
        eprintln!("Error reading file {:?}: {}", path, err);
        return;
    }

    // Read the file content
    if let Ok(ast) = syn::parse_file(&content) {
        let mut collector = NodeCollector::new();
        collector.visit_file(&ast);

        // Prepare the row to be written
        let mut item_row = Vec::new();
        item_row.push(path_str.to_string());
        let mut type_row = Vec::new();
        type_row.push(path_str.to_string());

        let item_counts = collector.get_items();
        let type_counts = collector.get_types();
        
        // Write count data for each header node type
        for header in item_headers {
            let count = item_counts.get(header).unwrap_or(&0); // If the key is missing, default to 0
            item_row.push(count.to_string());
        }
        // Write the row of data to the CSV
        item_wtr.write_record(item_row).expect("Unable to write record");

        // Write count data for each header node type
        for header in type_headers {
            let count = type_counts.get(header).unwrap_or(&0); // If the key is missing, default to 0
            type_row.push(count.to_string());
        }
        // Write the row of data to the CSV
        type_wtr.write_record(type_row).expect("Unable to write record");
    } else {
        eprintln!("Failed to parse file {:?}", path);
    }

}

fn main() {
    let dir_path = "/home/cici/rustc_bug_study/test_cases"; 
    let item_csv_path = "item_node_counts.csv";
    let type_csv_path = "type_node_counts.csv";

    let item_headers: Vec<String> = vec![
        "Function", "Struct", "Enum", "Trait", "Impl", "Mod", "Static", 
        "Const", "Type", "ExternCrate", "ForeignMod", "Macro", "TraitAlias", 
        "Union", "Use", "Verbatim", "Other"
    ].into_iter().map(String::from).collect();

    let type_headers: Vec<String> = vec![
        "Array", "BareFn", "Group", "ImplTrait", "Infer", "Macro", "Never", 
        "Paren", "Path", "Ptr", "Reference", "Slice", "TraitObject", 
        "Tuple", "Verbatim", "Other"
    ].into_iter().map(String::from).collect();

    // Open CSV file for writing
    let item_file = File::create(item_csv_path).expect("Unable to create file");
    let mut item_writer = Writer::from_writer(item_file);
    let type_file = File::create(type_csv_path).expect("Unable to create file");
    let mut type_writer = Writer::from_writer(type_file);

    // Write the headers as the first row
    let mut item_full_headers = vec!["File".to_string()];
    item_full_headers.extend(item_headers.clone());
    item_writer.write_record(&item_full_headers).expect("Unable to write header");
    let mut type_full_headers = vec!["File".to_string()];
    type_full_headers.extend(type_headers.clone());
    type_writer.write_record(&type_full_headers).expect("Unable to write header");


    for entry in WalkDir::new(dir_path).into_iter().filter_map(Result::ok) {
        if entry.path().extension().map(|e| e == "rs").unwrap_or(false) {
            println!("Parsing file: {}", entry.path().display());
            process_file(entry.path(), &mut item_writer, &item_headers, &mut type_writer, &type_headers);
        }
    }

}
