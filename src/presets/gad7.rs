use crate::condition::Condition;
use crate::questionnaire::Questionnaire;
use crate::questionnaire_item::QuestionnaireItem;
use crate::score_calculations::sum_score::SumScore;
use crate::term::{PhenotypeTerms, SeverityTerms};
use chrono::Duration;
use maplit::btreemap;

pub(crate) fn gad7() -> Questionnaire {
    Questionnaire::new(
        "GAD-7",
        vec![
            QuestionnaireItem::new(
                Some("Feeling nervous, anxious, or on edge".to_string()),
                PhenotypeTerms::Anxiety,
                4.0,
            ),
            QuestionnaireItem::new(
                Some("Not being able to stop or control worrying".to_string()), // TODO: Needs to get its own phenotype, current one is wrong
                PhenotypeTerms::Anxiety,
                4.0,
            ),
            QuestionnaireItem::new(
                Some("Worrying too much about different things".to_string()),
                PhenotypeTerms::Ruminations,
                4.0,
            ),
            QuestionnaireItem::new(
                Some("Trouble relaxing".to_string()),
                PhenotypeTerms::Agitation,
                4.0,
            ),
            QuestionnaireItem::new(
                Some("Being so restless that it is hard to sit still".to_string()),
                PhenotypeTerms::Restlessness,
                4.0,
            ),
            QuestionnaireItem::new(
                Some("Becoming easily annoyed or irritable".to_string()),
                PhenotypeTerms::Irritability,
                4.0,
            ),
            QuestionnaireItem::new(
                Some("Feeling afraid, as if something awful might happen".to_string()),
                PhenotypeTerms::AnticipatoryAnxiety,
                4.0,
            ),
        ],
        btreemap! {
                0 => Some(Condition::new_excluded(PhenotypeTerms::Anxiety)),
                1 => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Borderline)),
                5 => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Mild)),
                10 => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Moderate)),
                15 => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Severe)),
                20 => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Profound)),
        },
        SumScore,
        Some(Duration::weeks(2)),
    )
}
