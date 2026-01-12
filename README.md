Kent beck 의 Augmented conding 을 실습해 본다.
[Kent beck's BPlusTree3](https://github.com/KentBeck/BPlusTree3.git)

cursor 로 변경해서 실습해 본다.

## 환경 설정
rust 와 cargo 설치
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

## prompt 로 augmented coding 진행

1. Prompt 로 cursor custom commands 를 만든다.
```
CURSOR.md 파일을 활용하여 Kent Beck 의 TDD 원칙을 따르는 Custom Slash Commands 를 만들어줘.
```

2. 초기 기본 코드를 만든다.
```
@CURSOR.md 를 참조하여 코드를 만들어줘.

[ouput]
- plan.md
- src/lib.rs
- tests/bplus_tree.rs
- Cargo.toml
```

3. plan.md 의 첫번재 코드를 만든다.
```
prompt> @plan.md 의 첫 번째 항목을 위해 /red 해줘
$ cargo test

prompt> /green
$ cargo test

prompt> /refactor (지금은 생략, 하려면 명령을 추가함... 예: 이 변수명은 어색해, 이 로직은 나중에 복잡해질 것 같으니 분리하자)

prompt> @plan.md 에서 완료된 항목에 [x] 표시를 해줘
```
