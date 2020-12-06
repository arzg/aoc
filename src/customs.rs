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
                    .map(|q| &q.questions)
                    .all(|questions| questions.contains(question))
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
