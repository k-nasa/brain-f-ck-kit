use brain_fu_k::{CustomInstruction, Machine};

fn main() {
    let custom = CustomInstruction {
        pointer_increment: "オラ",
        pointer_desrement: "無駄",
        increment: "スターフィンガー！",
        decrement: "ロードローラーだ！",
        put: "ハーミットパープル",
        get: "あら手のスタンド使いか！",
        begin: "あ・・・ありのまま今起こったことを話すぜ",
        end: "ザ・ワールドッッ",
    };
    let input = "";

    Machine::custom(input, &custom).run();
}
