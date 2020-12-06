pub struct Group {
    person_questions: Vec<PersonQuestions>,
}

impl Group {
    pub fn num_questions(&self) -> usize {
        self.unique_questions().len()
    }

    pub fn num_questions_all_have(&self) -> usize {
        self.unique_questions()
            .iter()
            .filter(|question| {
                self.person_questions
                    .iter()
                    .all(|pq| pq.questions.contains(question))
            })
            .count()
    }

    fn unique_questions(&self) -> Vec<&Question> {
        let mut seen = Vec::new();

        for person_questions in &self.person_questions {
            for question in &person_questions.questions {
                if !seen.contains(&question) {
                    seen.push(question);
                }
            }
        }

        seen
    }
}

impl From<&str> for Group {
    fn from(s: &str) -> Self {
        Self {
            person_questions: s.lines().map(PersonQuestions::from).collect(),
        }
    }
}

struct PersonQuestions {
    questions: Vec<Question>,
}

impl From<&str> for PersonQuestions {
    fn from(s: &str) -> Self {
        PersonQuestions {
            questions: s.chars().map(Question).collect(),
        }
    }
}

#[derive(PartialEq)]
struct Question(char);

#[cfg(test)]
mod num_questions_tests {
    use super::*;

    #[test]
    fn abc() {
        assert_eq!(Group::from("abc").num_questions(), 3);
    }

    #[test]
    fn a_b_c() {
        assert_eq!(Group::from("a\nb\nc").num_questions(), 3);
    }

    #[test]
    fn ab_ac() {
        assert_eq!(Group::from("ab\nac").num_questions(), 3);
    }

    #[test]
    fn a_a_a_a() {
        assert_eq!(Group::from("a\na\na\na").num_questions(), 1);
    }

    #[test]
    fn b() {
        assert_eq!(Group::from("b").num_questions(), 1);
    }
}

#[cfg(test)]
mod num_questions_all_have_tests {
    use super::*;

    #[test]
    fn abc() {
        assert_eq!(Group::from("abc").num_questions_all_have(), 3);
    }

    #[test]
    fn a_b_c() {
        assert_eq!(Group::from("a\nb\nc").num_questions_all_have(), 0);
    }

    #[test]
    fn ab_ac() {
        assert_eq!(Group::from("ab\nac").num_questions_all_have(), 1);
    }

    #[test]
    fn a_a_a_a() {
        assert_eq!(Group::from("a\na\na\na").num_questions_all_have(), 1);
    }

    #[test]
    fn b() {
        assert_eq!(Group::from("b").num_questions_all_have(), 1);
    }
}
