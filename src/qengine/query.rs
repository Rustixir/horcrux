use bson::Bson;



pub struct Query<'a> {
    select: Vec<&'a str>,
    conds: Vec<Condition<'a>> ,
    distinct: Vec<&'a str>
}

impl<'a> Query<'a> {
    
    pub fn Select(fields: Vec<&'a str>) -> Self {
        Query { select: fields, conds: vec![], distinct: vec![] }
    }

    pub fn Where(mut self, conds: Vec<Condition<'a>>) -> Self {
        self.conds = conds;
        return self
    }

    pub fn Distinct(mut self, fields: Vec<&'a str>) -> Self {
        self.distinct = fields
    }
}


pub struct Condition<'a> {
    Field: &'a str,
    Value: &'a Bson
}