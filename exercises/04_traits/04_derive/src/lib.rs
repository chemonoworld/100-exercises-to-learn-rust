// TODO: 이 연습문제가 성공적으로 컴파일되기 위해서는 (파생 가능한) 트레이트 구현이 필요합니다.
//   수정하세요!
//
// # `Debug` 기초
//
// `Debug`는 Rust 타입의 디버깅에 적합한 표현을 반환합니다 (이름에서 알 수 있듯이).
// `assert_eq!`는 `Ticket`이 `Debug`를 구현해야 합니다. 왜냐하면, 단언이 실패할 때, 비교의 양쪽을 터미널에 출력하려고 하기 때문입니다.
// 비교된 타입이 `Debug`를 구현하지 않으면, 어떻게 표현해야 할지 알 수 없습니다!

use std::fmt::Debug;

#[derive(PartialEq, Debug, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl std::fmt::Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            std::format!(
                "title: {}, status: {}, desc: {}",
                self.title,
                self.status,
                self.description
            )
            .as_str(),
        )
    }

    // fn assert_eq!(t1: &Ticket, t2: &Ticket) -> bool {
    //     t1.title == t2.title && t1.description == t2.description && t1.status == t2.status
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let title = "title";
        let description = "description";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = ticket1.clone();
        print!("{}", ticket1);

        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_partial_eq() {
        let title = "title";
        let description = "description";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = "title";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: "description".to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: "description2".to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let status = "To-Do";
        let description = "description";
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = "title";
        let description = "description";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status".to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status2".to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }
}
