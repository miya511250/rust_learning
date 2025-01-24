// 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，并利用歌曲中的重复部分（编写循环）。
fn main() {
    let days = ["第一", "第二", "第三", "第四", "第五"];
    let gifts = ["一份特别的礼物", "两个欢乐", "三声祝福", "四份温暖", "五分感恩"];
    
    for (i, day) in days.iter().enumerate() {
        println!("\n在节日的{}天,", day);
        
        // 打印当前天及之前所有的礼物(从当前天倒序打印)
        for j in (0..=i).rev() {
            println!("收到了{}", gifts[j]);
        }
    }
}
