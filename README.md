こんな感じで BrainF\*ck ベースの言語を簡単に作れるやつ

example code

```
use brain_fu_k::{CustomInstruction, Machine};

fn main() {
    let custom = CustomInstruction {
        pointer_increment: "オラ".into(),
        pointer_desrement: "無駄".into(),
        increment: "スターフィンガー！".into(),
        decrement: "ロードローラーだ！".into(),
        put: "ハーミットパープル".into(),
        get: "あら手のスタンド使いか！".into(),
        begin: "あ・・・ありのまま今起こったことを話すぜ".into(),
        end: "ザ・ワールドッッ".into(),
    };
    let input = "";
    Machine::custom(input, &custom).run();
}
```
