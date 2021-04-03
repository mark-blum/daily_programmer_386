// https://old.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/
// Partition Counts

use std::time::Instant;

use daily_programmer_386::*;

fn main() {
    let target_n = 666;

    let width = 12;

    let prefix = "serial";
    let start = Instant::now();
    let answer = calc_partition_count(target_n);
    let elapsed = start.elapsed();
    println!("{:width$} {:?}", prefix, elapsed, width = width);
    println!("{:width$} {:?}", prefix, answer, width = width);
    println!();

    let num_threads = 24;

    let prefix = "parallel";
    let start = Instant::now();
    let answer_parallel = calc_partition_count_parallel(target_n, num_threads);
    let elapsed = start.elapsed();
    println!("{:width$} {:?}", prefix, elapsed, width = width);
    println!("{:width$} {:?}", prefix, answer_parallel, width = width);
    println!();

    let prefix = "threadpool";
    let start = Instant::now();
    let answer_parallel = calc_partition_count_parallel_threadpool(target_n, num_threads);
    let elapsed = start.elapsed();
    println!("{:width$} {:?}", prefix, elapsed, width = width);
    println!("{:width$} {:?}", prefix, answer_parallel, width = width);
}

#[cfg(test)]
mod tests {
    // const ANSWER_666666: &str = "829882047250572684700899902613243782763602762816201701722599315815312910790359761069230836156205082863018110775536469855308986200073144248662915902110787189076754874498156375781560473819383193570267234294008114407862435374738896137895011798602712056367666855560838392848713564675054729329398073507378373208972509842880751022273604950120819819461244250221006793015786720300981470607613047369007107554702116361432490562419340585594835559930063181308823544907938556335147860188606415089685992917539117106588219848248270148792532079530603636993578091236835691954161244027792120896238596848636567612717269000784250428006924746617450033567240084513811817484845287957454044679781070379504435782073968802016327182672402147816498886658350521297949309218478570934795197523632953503835428280916586305632528116828229355871664575877094278615695592039186556142662054263695788587794970386821424021653983372333685780633";
    // const ANSWER_666: &str = "11956824258286445517629485";
}
