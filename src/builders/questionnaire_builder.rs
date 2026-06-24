use crate::builders::questionnaire_item_builder::QuestionnaireItemBuilder;
use crate::condition::Condition;
use crate::error::RorschachError;
use crate::questionnaire::Questionnaire;
use crate::questionnaire_item::QuestionnaireItem;
use crate::traits::CalculateScore;
use chrono::Duration;
use std::collections::BTreeMap;

pub struct QuestionnaireBuilder {
    name: String,
    items: Vec<QuestionnaireItem>,
    interpretation: BTreeMap<i32, Option<Condition>>,
    score_calculator: Box<dyn CalculateScore>,
    recall_period: Option<Duration>,
}

impl QuestionnaireBuilder {
    pub fn new(name: impl Into<String>, score_calculator: Box<dyn CalculateScore>) -> Self {
        Self {
            name: name.into(),
            items: Vec::new(),
            interpretation: BTreeMap::new(),
            score_calculator,
            recall_period: None,
        }
    }

    pub fn item(mut self, item: impl Into<QuestionnaireItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn items(
        mut self,
        items: impl IntoIterator<Item = impl TryInto<QuestionnaireItem, Error = RorschachError>>,
    ) -> Result<Self, RorschachError> {
        let qi: Result<Vec<QuestionnaireItem>, RorschachError> =
            items.into_iter().map(TryInto::try_into).collect();
        self.items.extend(qi?);
        Ok(self)
    }

    pub fn interpretation(mut self, score: i32, condition: impl Into<Condition>) -> Self {
        self.interpretation.insert(score, Some(condition.into()));
        self
    }

    pub fn interpretation_none(mut self, score: i32) -> Self {
        self.interpretation.insert(score, None);
        self
    }

    pub fn interpretations(
        mut self,
        entries: impl IntoIterator<Item = (i32, Option<Condition>)>,
    ) -> Self {
        self.interpretation.extend(entries);
        self
    }

    pub fn recall_period(mut self, period: Duration) -> Self {
        self.recall_period = Some(period);
        self
    }

    pub fn build(self) -> Questionnaire {
        Questionnaire::new(
            self.name,
            self.items,
            self.interpretation,
            self.score_calculator,
            self.recall_period,
        )
    }
}

impl Questionnaire {
    pub fn builder(
        name: impl Into<String>,
        score_calculator: Box<dyn CalculateScore>,
    ) -> QuestionnaireBuilder {
        QuestionnaireBuilder::new(name, score_calculator)
    }
}

impl TryFrom<QuestionnaireItemBuilder> for QuestionnaireItem {
    type Error = RorschachError;

    fn try_from(builder: QuestionnaireItemBuilder) -> Result<Self, Self::Error> {
        builder.build()
    }
}
