use std::sync::Arc;

use arrow::{
    array::{
        Array, ArrayBuilder, ArrayRef, BooleanBuilder, NullBufferBuilder, StringBuilder,
        StructArray, UInt64Builder,
    },
    datatypes::{DataType, Field},
};

pub struct Card {
    pub last4: String,
    pub added_at: u64,
    pub active: bool,
}

#[derive(Debug)]
pub struct CardBuilder {
    pub last4: StringBuilder,
    pub added_at: UInt64Builder,
    pub active: BooleanBuilder,

    _nulls: ::arrow::array::NullBufferBuilder,
}

impl CardBuilder {
    pub fn append_value(&mut self, card: Card) {
        self.last4.append_value(card.last4);
        self.added_at.append_value(card.added_at);
        self.active.append_value(card.active);

        self._nulls.append_non_null();
    }

    pub fn append_null(&mut self) {
        self.last4.append_null();
        self.added_at.append_null();
        self.active.append_null();

        self._nulls.append_null();
    }

    pub fn append_option(&mut self, card: Option<Card>) {
        match card {
            Some(card) => self.append_value(card),
            None => self.append_null(),
        }
    }

    pub fn finish(&mut self) -> StructArray {
        let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
        let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();

        {
            let last4_array = Arc::new(self.last4.finish());
            let last4_field = Field::new("last4", last4_array.data_type().clone(), false);
            arrays.push(last4_array);
            fields.push(last4_field);
        };

        let added_at_array = Arc::new(self.added_at.finish());
        let added_at_field = Field::new("added_at", added_at_array.data_type().clone(), false);
        arrays.push(added_at_array);
        fields.push(added_at_field);

        let active_array = Arc::new(self.active.finish());
        let active_field = Field::new("active", active_array.data_type().clone(), false);
        arrays.push(active_array);
        fields.push(active_field);

        StructArray::new(
            ::arrow::datatypes::Fields::from(fields),
            arrays,
            self._nulls.finish(),
        )
    }

    pub fn finish_cloned(&self) -> StructArray {
        let arrays: Vec<::arrow::array::ArrayRef> = vec![
            Arc::new(self.last4.finish_cloned()),
            Arc::new(self.added_at.finish_cloned()),
            Arc::new(self.active.finish_cloned()),
        ];

        let fields: ::arrow::datatypes::Fields = ::arrow::datatypes::Fields::from(vec![
            Field::new("last4", DataType::Utf8, false),
            Field::new("added_at", DataType::UInt64, false),
            Field::new("active", DataType::Boolean, false),
        ]);

        let nulls = self._nulls.finish_cloned();

        StructArray::new(fields, arrays, nulls)
    }
}

impl Default for CardBuilder {
    fn default() -> Self {
        CardBuilder {
            last4: Default::default(),
            added_at: Default::default(),
            active: Default::default(),
            _nulls: NullBufferBuilder::new(0),
        }
    }
}

impl ArrayBuilder for CardBuilder {
    fn len(&self) -> usize {
        self._nulls.len()
    }

    fn finish(&mut self) -> ArrayRef {
        Arc::new(self.finish())
    }

    fn finish_cloned(&self) -> ArrayRef {
        Arc::new(self.finish_cloned())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn into_box_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl Extend<Option<Card>> for CardBuilder {
    fn extend<T: IntoIterator<Item = Option<Card>>>(&mut self, iter: T) {
        iter.into_iter().for_each(|p| self.append_option(p));
    }
}
