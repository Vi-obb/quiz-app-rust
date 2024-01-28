struct Question {
    title: String,
    answer: bool,
    user_answer: Option<bool>,
}

impl Question {
    fn is_answered(&self) -> bool {
        self.user_answer.is_some()
    }

    fn is_correct(&self) -> bool {
        self.user_answer == Some(self.answer)
    }
}

struct Quiz {
    questions: Vec<Question>,
    current_index: usize,
}

impl Quiz {
    fn current_question(&self) -> &Question {
        &self.questions[self.current_index]
    }

    fn current_question_mut(&mut self) -> Option<&mut Question> {
        self.questions.get_mut(self.current_index)
    }

    fn next_question(&mut self) -> &Question {
        let count = self.questions.len() - 1;

        if self.current_question().is_answered() && self.current_index < count {
            self.current_index += 1;
        }

        self.current_question()
    }

    fn prev_question(&mut self) -> &Question {
        let count = self.questions.len() - 1;

        if self.current_index > count {
            self.current_index -= 1;
        }

        self.current_question()
    }

    fn answer(&mut self, answer: bool) {
    if let Some(question) = self.current_question_mut() {
        question.user_answer = Some(answer);
    }
}

    fn calculate_score(&self) -> usize {
        let mut score = 0;

        for question in &self.questions {
            if question.is_correct() {
                score += 1;
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
      user_answer: Some(true)
    };

    assert_eq!(q.is_answered(), true);
  }

  #[test]
  fn test_question_is_correct() {
    let q = Question {
      title: String::from("Is this a question?"),
      answer: true,
      user_answer: Some(true)
    };

    assert_eq!(q.is_correct(), true);
  }
}