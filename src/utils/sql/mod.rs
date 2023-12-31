use crate::{
    prelude::{SortingDirection, ToSQL},
    types::Limit,
};

pub struct SelectRequestBuilder<OrderingType, Query>
where
    OrderingType: ToSQL,
    Query: ToSQL,
{
    main_query: String,
    group_by: Option<String>,
    order_by: Option<SortingDirection<OrderingType>>,
    limit: Option<Limit>,
    query: Vec<Query>,
}

impl<OrderingType, Query> SelectRequestBuilder<OrderingType, Query>
where
    OrderingType: ToSQL,
    Query: ToSQL,
{
    pub fn new(root_sql: String, query: Vec<Query>) -> SelectRequestBuilder<OrderingType, Query> {
        SelectRequestBuilder {
            main_query: root_sql,
            group_by: None,
            order_by: None,
            query,
            limit: None,
        }
    }
    pub fn order_by(
        self,
        order_by: SortingDirection<OrderingType>,
    ) -> SelectRequestBuilder<OrderingType, Query> {
        SelectRequestBuilder {
            main_query: self.main_query,
            group_by: self.group_by,
            order_by: Some(order_by),
            query: self.query,
            limit: self.limit,
        }
    }

    pub fn group_by(self, group_by: String) -> SelectRequestBuilder<OrderingType, Query> {
        SelectRequestBuilder {
            main_query: self.main_query,
            group_by: Some(group_by),
            order_by: self.order_by,
            query: self.query,
            limit: self.limit,
        }
    }

    pub fn limit(self, limit: Limit) -> SelectRequestBuilder<OrderingType, Query> {
        SelectRequestBuilder {
            main_query: self.main_query,
            group_by: self.group_by,
            order_by: self.order_by,
            limit: Some(limit),
            query: self.query,
        }
    }

    pub fn build(self) -> String {
        format!(
            "{} {} {} {} {} {};",
            self.main_query,
            match self.query.first() {
                Some(query) => format!("where {}", query.to_sql()),
                None => String::new(),
            },
            self.query[1..self.query.len()]
                .iter()
                .map(|query| format!(" and {}", query.to_sql()))
                .collect::<String>(),
            match self.group_by {
                Some(group_by) => format!("group by {}", group_by),
                None => String::new(),
            },
            match self.order_by {
                Some(order) => format!("order by {}", order.to_sql()),
                None => String::new(),
            },
            match self.limit {
                Some(limit) => format!(
                    "limit {} {}",
                    limit.limit,
                    match limit.offset {
                        Some(offset) => format!("offset {}", offset),
                        None => String::new(),
                    }
                ),
                None => String::new(),
            },
        )
    }
}
