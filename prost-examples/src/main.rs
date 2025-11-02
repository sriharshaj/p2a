fn main() {}

#[cfg(test)]
mod proto2arrow_tests {
    use arrow::array::Array;
    use pretty_assertions::assert_eq;
    use prost_examples::{example, example_builders};

    fn create_test_users() -> Vec<example::User> {
        let user1 = example::User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            age: 25,
            is_active: true,
            r#type: example::user::UserType::Reader.into(),
            payment_method: None,
            addresses: vec![
                example::user::Address {
                    street: "123 Main St".to_string(),
                    city: "SunValley".to_string(),
                    state: "California".to_string(),
                    zip_code: "12345".to_string(),
                    country: "USA".to_string(),
                },
                example::user::Address {
                    street: "789 Second Ave".to_string(),
                    city: "Springfield".to_string(),
                    state: "California".to_string(),
                    zip_code: "12346".to_string(),
                    country: "USA".to_string(),
                },
            ],
            transactions: vec![example::Transaction {
                amount: 12.34,
                timestamp: 1758574905936,
            }],
            posts: ::std::collections::HashMap::new(),
        };

        let user2 = example::User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            age: 30,
            is_active: false,
            r#type: example::user::UserType::Author.into(),
            payment_method: Some(example::user::PaymentMethod::CreditCard(
                example::CreditCard {
                    card_number: "1234-5678-9012-3456".to_string(),
                    expiry_month: 12,
                    expiry_year: 2029,
                    cvv: "123".to_string(),
                },
            )),
            addresses: vec![example::user::Address {
                street: "456 Oak Ave".to_string(),
                city: "Portland".to_string(),
                state: "Oregon".to_string(),
                zip_code: "97201".to_string(),
                country: "USA".to_string(),
            }],
            transactions: vec![],
            posts: {
                let mut posts = ::std::collections::HashMap::new();
                posts.insert(
                    12,
                    example::Post {
                        title: "My First Post".into(),
                        body: "This is the content of my first post.".into(),
                    },
                );
                posts.insert(
                    23,
                    example::Post {
                        title: "Another Post".into(),
                        body: "More content here.".into(),
                    },
                );
                posts
            },
        };

        let user3 = example::User {
            id: 3,
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
            age: 28,
            is_active: true,
            r#type: example::user::UserType::Editor.into(),
            payment_method: Some(example::user::PaymentMethod::BankAccount(
                example::BankAccount {
                    account_number: "12345667".to_string(),
                    routing_number: "435626512436".to_string(),
                    bank_name: "DCU".to_string(),
                },
            )),
            addresses: vec![],
            transactions: vec![
                example::Transaction {
                    amount: 17.24,
                    timestamp: 1757574905936,
                },
                example::Transaction {
                    amount: 45.24,
                    timestamp: 1757577905936,
                },
            ],
            posts: {
                let mut posts = ::std::collections::HashMap::new();
                posts.insert(
                    3,
                    example::Post {
                        title: "Charlie's Post".into(),
                        body: "This is Charlie's post content.".into(),
                    },
                );
                posts
            },
        };

        vec![user1, user2, user3]
    }

    #[test]
    fn builder_default() {
        let _ = example_builders::UserBuilder::default();
    }

    #[test]
    fn append_value_to_builder() {
        let users = create_test_users();
        let mut ub = example_builders::UserBuilder::default();
        ub.append_value(users.into_iter().next().unwrap());
        let user_array = ub.finish();

        assert_eq!(user_array.len(), 1);
        assert_eq!(user_array.null_count(), 0);
    }

    #[test]
    fn append_null_to_builder() {
        let mut ub = example_builders::UserBuilder::default();
        ub.append_null();
        let user_array = ub.finish();

        assert_eq!(user_array.len(), 1);
        assert_eq!(user_array.null_count(), 1);
    }

    #[test]
    fn extend_builder_with_multiple_values() {
        let users = create_test_users();
        let mut ub = example_builders::UserBuilder::default();
        ub.extend(users.into_iter().map(Some));
        let user_array = ub.finish();

        assert_eq!(user_array.len(), 3);
        assert_eq!(user_array.null_count(), 0);
    }

    #[test]
    fn finish_empties_builder_state() {
        let users = create_test_users();
        let mut ub = example_builders::UserBuilder::default();
        ub.extend(users.into_iter().map(Some));
        let user_array = ub.finish();

        assert_eq!(user_array.len(), 3);
        assert_eq!(user_array.null_count(), 0);

        ub.append_null();
        let user_array = ub.finish();
        assert_eq!(user_array.len(), 1);
        assert_eq!(user_array.null_count(), 1);
    }

    #[test]
    fn finish_cloned_preserves_builder_state() {
        let users = create_test_users();
        let mut ub = example_builders::UserBuilder::default();
        ub.extend(users.into_iter().map(Some));
        let user_array = ub.finish_cloned();

        assert_eq!(user_array.len(), 3);
        assert_eq!(user_array.null_count(), 0);

        ub.append_null();
        let user_array = ub.finish_cloned();
        assert_eq!(user_array.len(), 4);
        assert_eq!(user_array.null_count(), 1);
    }

    #[test]
    fn builder_arrow_schema() {
        use arrow::datatypes::{DataType, Field, Fields};
        use std::sync::Arc;

        let mut ub = example_builders::UserBuilder::default();
        let user_array = ub.finish();
        let schema = user_array.fields();

        let post_schema = DataType::Struct(Fields::from(vec![
            Field::new("title", DataType::Utf8, false),
            Field::new("body", DataType::Binary, false),
        ]));
        let posts_map = Field::new_map(
            "posts",
            "entries",
            Arc::new(Field::new("keys", DataType::Int32, false)),
            Arc::new(Field::new("values", post_schema, true)),
            false,
            true,
        );

        let transaction_schema = DataType::Struct(Fields::from(vec![
            Field::new("amount", DataType::Float32, false),
            Field::new("timestamp", DataType::UInt64, false),
        ]));
        let transactions_list = Field::new_list(
            "transactions",
            Arc::new(Field::new_list_field(transaction_schema, true)),
            true,
        );

        let address_schema = DataType::Struct(Fields::from(vec![
            Field::new("street", DataType::Utf8, false),
            Field::new("city", DataType::Utf8, false),
            Field::new("state", DataType::Utf8, false),
            Field::new("zip_code", DataType::Utf8, false),
            Field::new("country", DataType::Utf8, false),
        ]));
        let addresses_list = Field::new_list(
            "addresses",
            Arc::new(Field::new_list_field(address_schema, true)),
            true,
        );

        let credit_card_field = Field::new_struct(
            "credit_card",
            Fields::from(vec![
                Field::new("card_number", DataType::Utf8, false),
                Field::new("expiry_month", DataType::UInt32, false),
                Field::new("expiry_year", DataType::UInt32, false),
                Field::new("cvv", DataType::Utf8, false),
            ]),
            true,
        );

        let bank_account_field = Field::new_struct(
            "bank_account",
            Fields::from(vec![
                Field::new("account_number", DataType::Utf8, false),
                Field::new("routing_number", DataType::Utf8, false),
                Field::new("bank_name", DataType::Utf8, false),
            ]),
            true,
        );

        let payment_method_field = Field::new_struct(
            "payment_method",
            Fields::from(vec![credit_card_field, bank_account_field]),
            true,
        );

        let expected_schema = Fields::from(vec![
            Field::new("id", DataType::UInt32, false),
            Field::new("name", DataType::Utf8, false),
            Field::new("email", DataType::Utf8, false),
            Field::new("age", DataType::UInt32, false),
            Field::new("is_active", DataType::Boolean, false),
            Field::new("type", DataType::Int32, false),
            addresses_list,
            transactions_list,
            posts_map,
            payment_method_field,
        ]);

        assert_eq!(schema, &expected_schema);
    }

    #[test]
    fn builder_arrow_data() {
        use arrow::array::{Array, AsArray};

        let users = create_test_users();
        let mut ub = example_builders::UserBuilder::default();
        ub.extend(users.clone().into_iter().map(Some));
        let user_array = ub.finish();

        let id_array = user_array
            .column_by_name("id")
            .unwrap()
            .as_primitive::<arrow::datatypes::UInt32Type>();
        let name_array = user_array
            .column_by_name("name")
            .unwrap()
            .as_string::<i32>();
        let email_array = user_array
            .column_by_name("email")
            .unwrap()
            .as_string::<i32>();
        let age_array = user_array
            .column_by_name("age")
            .unwrap()
            .as_primitive::<arrow::datatypes::UInt32Type>();
        let is_active_array = user_array.column_by_name("is_active").unwrap().as_boolean();
        let type_array = user_array
            .column_by_name("type")
            .unwrap()
            .as_primitive::<arrow::datatypes::Int32Type>();
        let addresses_array = user_array
            .column_by_name("addresses")
            .unwrap()
            .as_list::<i32>();
        let payment_method_array = user_array
            .column_by_name("payment_method")
            .unwrap()
            .as_struct();
        let credit_card_array = payment_method_array
            .column_by_name("credit_card")
            .unwrap()
            .as_struct();
        let bank_account_array = payment_method_array
            .column_by_name("bank_account")
            .unwrap()
            .as_struct();
        let transactions_array = user_array
            .column_by_name("transactions")
            .unwrap()
            .as_list::<i32>();
        let posts_array = user_array.column_by_name("posts").unwrap().as_map();

        // Validate User 1 (Alice)
        assert_eq!(id_array.value(0), 1);
        assert_eq!(name_array.value(0), "Alice");
        assert_eq!(email_array.value(0), "alice@example.com");
        assert_eq!(age_array.value(0), 25);
        assert!(is_active_array.value(0));
        assert_eq!(type_array.value(0), 0);

        let alice_addresses = addresses_array.value(0);
        assert_eq!(alice_addresses.len(), 2);
        let alice_addresses_struct = alice_addresses.as_struct();

        let street_array = alice_addresses_struct
            .column_by_name("street")
            .unwrap()
            .as_string::<i32>();
        let city_array = alice_addresses_struct
            .column_by_name("city")
            .unwrap()
            .as_string::<i32>();
        let state_array = alice_addresses_struct
            .column_by_name("state")
            .unwrap()
            .as_string::<i32>();
        let zip_code_array = alice_addresses_struct
            .column_by_name("zip_code")
            .unwrap()
            .as_string::<i32>();
        let country_array = alice_addresses_struct
            .column_by_name("country")
            .unwrap()
            .as_string::<i32>();

        assert_eq!(street_array.value(0), "123 Main St");
        assert_eq!(city_array.value(0), "SunValley");
        assert_eq!(state_array.value(0), "California");
        assert_eq!(zip_code_array.value(0), "12345");
        assert_eq!(country_array.value(0), "USA");

        assert_eq!(street_array.value(1), "789 Second Ave");
        assert_eq!(city_array.value(1), "Springfield");
        assert_eq!(state_array.value(1), "California");
        assert_eq!(zip_code_array.value(1), "12346");
        assert_eq!(country_array.value(1), "USA");

        assert!(payment_method_array.is_null(0));

        let alice_transactions = transactions_array.value(0);
        assert_eq!(alice_transactions.len(), 1);
        let alice_transactions_struct = alice_transactions.as_struct();

        let amount_array = alice_transactions_struct
            .column_by_name("amount")
            .unwrap()
            .as_primitive::<arrow::datatypes::Float32Type>();
        let timestamp_array = alice_transactions_struct
            .column_by_name("timestamp")
            .unwrap()
            .as_primitive::<arrow::datatypes::UInt64Type>();

        assert_eq!(amount_array.value(0), 12.34);
        assert_eq!(timestamp_array.value(0), 1758574905936);

        assert!(posts_array.is_null(0));

        // Validate User 2 (Bob)
        assert_eq!(id_array.value(1), 2);
        assert_eq!(name_array.value(1), "Bob");
        assert_eq!(email_array.value(1), "bob@example.com");
        assert_eq!(age_array.value(1), 30);
        assert!(!is_active_array.value(1));
        assert_eq!(type_array.value(1), 1);

        let bob_addresses = addresses_array.value(1);
        assert_eq!(bob_addresses.len(), 1);
        let bob_addresses_struct = bob_addresses.as_struct();

        let street_array = bob_addresses_struct
            .column_by_name("street")
            .unwrap()
            .as_string::<i32>();
        let city_array = bob_addresses_struct
            .column_by_name("city")
            .unwrap()
            .as_string::<i32>();
        let state_array = bob_addresses_struct
            .column_by_name("state")
            .unwrap()
            .as_string::<i32>();
        let zip_code_array = bob_addresses_struct
            .column_by_name("zip_code")
            .unwrap()
            .as_string::<i32>();
        let country_array = bob_addresses_struct
            .column_by_name("country")
            .unwrap()
            .as_string::<i32>();

        assert_eq!(street_array.value(0), "456 Oak Ave");
        assert_eq!(city_array.value(0), "Portland");
        assert_eq!(state_array.value(0), "Oregon");
        assert_eq!(zip_code_array.value(0), "97201");
        assert_eq!(country_array.value(0), "USA");

        assert!(!payment_method_array.is_null(1));
        assert!(!credit_card_array.is_null(1));
        assert!(bank_account_array.is_null(1));

        let card_number_array = credit_card_array
            .column_by_name("card_number")
            .unwrap()
            .as_string::<i32>();
        let expiry_month_array = credit_card_array
            .column_by_name("expiry_month")
            .unwrap()
            .as_primitive::<arrow::datatypes::UInt32Type>();
        let expiry_year_array = credit_card_array
            .column_by_name("expiry_year")
            .unwrap()
            .as_primitive::<arrow::datatypes::UInt32Type>();
        let cvv_array = credit_card_array
            .column_by_name("cvv")
            .unwrap()
            .as_string::<i32>();

        assert_eq!(card_number_array.value(1), "1234-5678-9012-3456");
        assert_eq!(expiry_month_array.value(1), 12);
        assert_eq!(expiry_year_array.value(1), 2029);
        assert_eq!(cvv_array.value(1), "123");
        assert!(transactions_array.is_null(1));

        let bob_posts = posts_array.value(1);
        let keys_array = bob_posts
            .column_by_name("keys")
            .unwrap()
            .as_primitive::<arrow::datatypes::Int32Type>();
        let values_array = bob_posts.column_by_name("values").unwrap().as_struct();

        let (first, second) = if keys_array.value(0) != 12 {
            (1, 0)
        } else {
            (0, 1)
        };
        assert_eq!(keys_array.value(first), 12);
        assert_eq!(keys_array.value(second), 23);

        let title_array = values_array
            .column_by_name("title")
            .unwrap()
            .as_string::<i32>();
        let body_array = values_array
            .column_by_name("body")
            .unwrap()
            .as_binary::<i32>();

        assert_eq!(title_array.value(first), "My First Post");
        assert_eq!(title_array.value(second), "Another Post");

        let first_body = body_array.value(first).into();
        // let first_body_bytes = first_body.as_primitive::<arrow::datatypes::BinaryType>();
        let first_body_string = String::from_utf8(first_body).unwrap();
        assert_eq!(first_body_string, "This is the content of my first post.");

        let second_body = body_array.value(second).into();
        let second_body_string = String::from_utf8(second_body).unwrap();
        assert_eq!(second_body_string, "More content here.");

        // Validate User 3 (Charlie)
        assert_eq!(id_array.value(2), 3);
        assert_eq!(name_array.value(2), "Charlie");
        assert_eq!(email_array.value(2), "charlie@example.com");
        assert_eq!(age_array.value(2), 28);
        assert!(is_active_array.value(2));
        assert_eq!(type_array.value(2), 2);

        assert!(addresses_array.is_null(2));

        assert!(!payment_method_array.is_null(2));
        assert!(credit_card_array.is_null(2));
        assert!(!bank_account_array.is_null(2));

        let account_number_array = bank_account_array
            .column_by_name("account_number")
            .unwrap()
            .as_string::<i32>();
        let routing_number_array = bank_account_array
            .column_by_name("routing_number")
            .unwrap()
            .as_string::<i32>();
        let bank_name_array = bank_account_array
            .column_by_name("bank_name")
            .unwrap()
            .as_string::<i32>();

        assert_eq!(account_number_array.value(2), "12345667");
        assert_eq!(routing_number_array.value(2), "435626512436");
        assert_eq!(bank_name_array.value(2), "DCU");

        let charlie_transactions = transactions_array.value(2);
        assert_eq!(charlie_transactions.len(), 2);
        let charlie_transactions_struct = charlie_transactions.as_struct();

        let amount_array = charlie_transactions_struct
            .column_by_name("amount")
            .unwrap()
            .as_primitive::<arrow::datatypes::Float32Type>();
        let timestamp_array = charlie_transactions_struct
            .column_by_name("timestamp")
            .unwrap()
            .as_primitive::<arrow::datatypes::UInt64Type>();

        assert_eq!(amount_array.value(0), 17.24);
        assert_eq!(timestamp_array.value(0), 1757574905936);
        assert_eq!(amount_array.value(1), 45.24);
        assert_eq!(timestamp_array.value(1), 1757577905936);

        let charlie_posts = posts_array.value(2);

        let keys_array = charlie_posts
            .column_by_name("keys")
            .unwrap()
            .as_primitive::<arrow::datatypes::Int32Type>();
        let values_array = charlie_posts.column_by_name("values").unwrap().as_struct();

        assert_eq!(keys_array.value(0), 3);
        let title_array = values_array
            .column_by_name("title")
            .unwrap()
            .as_string::<i32>();
        let body_array = values_array
            .column_by_name("body")
            .unwrap()
            .as_binary::<i32>();

        assert_eq!(title_array.value(0), "Charlie's Post");

        let charlie_body = body_array.value(0).into();
        let charlie_body_string = String::from_utf8(charlie_body).unwrap();
        assert_eq!(charlie_body_string, "This is Charlie's post content.");
    }
}
