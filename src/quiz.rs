pub struct Question {
    pub title: String,
    pub answer: bool,
    pub user_answer: Option<bool>,
}

impl Question {
    fn is_answered(&self) -> bool {
        self.user_answer.is_some()
    }

    pub fn is_correct(&self) -> bool {
        self.user_answer == Some(self.answer)
    }
}

pub struct Quiz {
    pub questions: Vec<Question>,
    pub current_index: usize,
}

impl Quiz {
    pub fn sample() -> Self {
    Self {
      questions: vec![
        Question {
          title: "Is the sky blue?".to_string(),
          answer: true,
          user_answer: None
        },
        Question {
          title: "Is the grass green?".to_string(),
          answer: true,
          user_answer: None
        },
        Question {
          title: "Is the sun yellow?".to_string(),
          answer: false,
          user_answer: None
        },
      ],
      current_index: 0
    }
  }

    pub fn current_question(&self) -> &Question {
        &self.questions[self.current_index]
    }

    fn current_question_mut(&mut self) -> Option<&mut Question> {
        self.questions.get_mut(self.current_index)
    }

    pub fn next_question(&mut self) -> &Question {
        let count = self.questions.len() - 1;

        if self.current_question().is_answered() && self.current_index < count {
            self.current_index += 1;
        }

        self.current_question()
    }

    pub fn prev_question(&mut self) -> &Question {
        if self.current_index > 0 {
            self.current_index -= 1;
        }

        self.current_question()
    }

    pub fn answer(&mut self, answer: bool) {
        if let Some(question) = self.current_question_mut() {
            question.user_answer = Some(answer);
        }
    }

    pub fn calculate_score(&self) -> i32 {
        let mut score = 0;

        for question in &self.questions {
            if let Some(_user_answer) = question.user_answer {
                if question.is_correct() {
                    score += 1;
                } else {
                    score -= 1;
                }
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_is_answered() {
        let q = Question {
            title: String::from("Is this a question?"),
            answer: true,
            user_answer: Some(true),
        };

        assert_eq!(q.is_answered(), true);
    }

    #[test]
    fn test_question_is_correct() {
        let q = Question {
            title: String::from("Is this a question?"),
            answer: true,
            user_answer: Some(true),
        };

        assert_eq!(q.is_correct(), true);
    }
}
