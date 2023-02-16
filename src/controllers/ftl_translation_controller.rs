#[cfg(feature = "ipadic")]
use std::path::PathBuf;

use std::{
    fs,
    env,
    error::Error,
    result::Result,
};
use lindera::LinderaResult;

#[cfg(feature = "ipadic")]
use lindera::{
    mode::Mode,
    tokenizer::{DictionaryConfig, Tokenizer, TokenizerConfig, UserDictionaryConfig},
    DictionaryKind,
};

use fluent::{FluentBundle, FluentResource};
use unic_langid::LanguageIdentifier;

pub struct QueryConfig {
    /// we can't have &str or &'static str because it has to outlive-
    /// - the given scenario
    query: String,
    filepath: String,
    ignored: bool,
}

impl QueryConfig {
    pub fn build(args: &[String]) -> Result<QueryConfig, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();

        let ignored = env::var("IGNORE_CASE").is_ok();

        Ok(QueryConfig {
            query: query,
            filepath: filepath,
            ignored: ignored,
        })
    }
}

fn assert_ftl_string(lhs_ftl_string: String, rhs_ftl_string: &str) -> Result<&'static str, Box<dyn Error>> {
    let rhs_ftl_string = rhs_ftl_string.to_owned();

    let li: LanguageIdentifier = "en-US".parse().expect("Failed to parse");

    let resulting_string = FluentResource::try_new(lhs_ftl_string).expect("Failed to parse an FTL string.");
    let mut bundle_vector = FluentBundle::new(vec![li]);

    bundle_vector.add_resource(resulting_string).expect("Failed to add FTL resources to the bundle.");

    let message = &bundle_vector.get_message(&rhs_ftl_string).expect("Message doesn't exist!");
    let mut errors = vec![];
    let pattern = message.value().expect("Message has no value.");
    let value = bundle_vector.format_pattern(&pattern, None, &mut errors);

    assert_eq!(&rhs_ftl_string, &value);

    return Ok("Ok");
}
/// "&String" type is used because of the T: Copy trait which acts as an ownership's lifetime
/// Using String.clone() or &str.to_owned() would not satisfy the compiler because the value loses its integrity when its data is moved
fn read_ipadic_data<T>(token_inputs: &String) -> LinderaResult<()> where T: Copy {
    let original_token_inputs = token_inputs;

    #[cfg(feature = "ipadic")]
    {
        let dictionary = DictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: None,
        };

        let user_dictionary = Some(UserDictionaryConfig {
            kind: Some(DictionaryKind::IPADIC),
            path: PathBuf::from(env!("CARGO_MANIFEST_DIR")) // use file i/o over path buffering
                .join("./public/resources/user/dict") // secure a filesystem directory in 'routes'
                .join("ipadic_userdic.csv"), // take these 'CSV' files from SvelteKit
        }); // make this take from an continuous updating file buffer from Model-View-Controller

        let config = TokenizerConfig {
            dictionary,
            user_dictionary,
            mode: Mode::Normal,
        };

        #[allow(unused_variables)]
        let tokenizer = Tokenizer::with_config(config).unwrap();

        // tokenize the text
        let tokens = tokenizer.tokenize(token_inputs)?;
        // tokenise variable instead of String double-quotes

        for token in tokens {
            println!("{}", token.text);
        }
    }

    /// We can clone and change ownership with the values here-
    /// -because we are outside of an inline statement
    assert_ftl_string(token_inputs.clone(), &original_token_inputs.to_owned());

    return Ok(());
}

fn run_user_data_input(query_config: QueryConfig) -> Result<&'static str, Box<dyn Error>> {
    let contents = fs::read_to_string(query_config.filepath);
    let results = if query_config.ignored {
        read_ipadic_data::<&String>(&query_config.query);
    };

    println!("\n\t{:?}\n\t\t$=>{:?}", contents, results);

    return Ok("Ok");
}