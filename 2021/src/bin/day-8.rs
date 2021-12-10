use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn solve(data: [&str; 10], display: [&str; 4]) -> i32 {
    // todo: Maybe data should be appended with display

    let i: Vec<HashSet<char>> = data.iter().map(|s| HashSet::from_iter(s.chars())).collect();

    let display: Vec<HashSet<char>> = display
        .iter()
        .map(|s| HashSet::from_iter(s.chars()))
        .collect();

    let mut h = HashMap::new();

    for segments in &i {
        match segments.len() {
            2 => h.insert(1, segments),
            3 => h.insert(7, segments),
            4 => h.insert(4, segments),
            7 => h.insert(8, segments),
            _ => None,
        };
    }

    // Six, Nine, and Zero have 6 segments and their intersection with
    // 1 and 4 uniquely identifies the number by how many segments
    // remain.
    let six = &i
        .iter()
        .find(|x| x.len() == 6 && (x.intersection(&h[&1]).collect::<Vec<_>>().len() == 1))
        .unwrap();
    h.insert(6, six);

    let nine = &i
        .iter()
        .find(|x| x.len() == 6 && (x.difference(&h[&4]).collect::<Vec<_>>().len() == 2))
        .unwrap();

    h.insert(9, nine);

    // There are only 3 numbers of size 6, we've found two so set zero as the third
    let zero = &i
        .iter()
        .find(|x| x.len() == 6 && !x.eq(six) && !x.eq(nine))
        .unwrap();

    h.insert(0, zero);

    // Three, two, and five have 5 segments and we can intersect or
    // difference and check remaining segments to identify three and
    // two. Five is a little tricker (because I'm tired.) I ended up
    // fighting the borrow checker while attempting to us a difference
    // and chain with another intersection.

    // My error was
    /*
    error[E0308]: mismatched types
      --> src/bin/day-8.rs:67:47
       |
    67 |             x.len() == 5 && diff.intersection(*one).collect::<Vec<_>>().len() == 1
       |                                               ^^^^ expected `&char`, found `char`
       |
       = note: expected reference `&HashSet<&char>`
                  found reference `&HashSet<char>`

    For more information about this error, try `rustc --explain E0308`.

    12/09 - I found out that I can use `.copied()` here.
     */
    let three = &i
        .iter()
        .find(|x| x.len() == 5 && x.intersection(&h[&1]).collect::<Vec<_>>().len() == 2)
        .unwrap();

    h.insert(3, three);

    // I messed with two and 5
    let two = &i
        .iter()
        .find(|x| x.len() == 5 && x.difference(nine).collect::<Vec<_>>().len() == 1)
        .unwrap();

    h.insert(2, two);

    let five = &i
        .iter()
        .find(|x| {
            x.len() == 5
                && six.difference(x).collect::<Vec<_>>().len() == 1
                && x.difference(three).collect::<Vec<_>>().len() == 1
        })
        .unwrap();

    h.insert(5, five);

    assert_eq!(&10, &h.len());
    assert_eq!(&4, &display.len());

    display
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, v)| {
            // panic here
            let p = h.iter().find(|(_num, set)| set.eq(&&v));
            match p {
                Some((n, _)) => return n * 10_i32.pow(idx as u32),
                None => {
                    println!(
                        "\nError!!!!!\nInput:{:?}\n{:?}\nDisplay{:?}\n",
                        &data, &h, &display
                    );
                    return 0;
                }
            }
        })
        .sum::<i32>()
}

fn main() {
    let i = input();

    let result = i.iter().fold(0, |total, (_, output)| {
        total
            + output.iter().fold(0, |t, s| {
                if matches!(s.len(), 2 | 3 | 4 | 7) {
                    return t + 1;
                }

                t
            })
    });

    assert_eq!(&512, &result);

    // Part 2

    let data = test_input();
    let result = solve(data[0].0, data[0].1);
    println!("Result: {}", result);

    let data = input();

    let result: i32 = data.iter().map(|data| solve(data.0, data.1)).sum();
    println!("input: {}", result);
}

#[allow(dead_code)]
fn test_input() -> Vec<([&'static str; 10], [&'static str; 4])> {
    vec![
        (
            [
                "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb", "fabcd",
                "edb",
            ],
            ["fdgacbe", "cefdb", "cefbgd", "gcbe"],
        ),
        (
            [
                "edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde", "gfcbed",
                "gfec",
            ],
            ["fcgedb", "cgb", "dgebacf", "gc"],
        ),
        (
            [
                "fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb",
                "cdgabef",
            ],
            ["cg", "cg", "fdcagb", "cbg"],
        ),
        (
            [
                "fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab", "fgdeca",
                "fcdbega",
            ],
            ["efabcd", "cedba", "gadfec", "cb"],
        ),
        (
            [
                "aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab",
                "fcbdga",
            ],
            ["gecf", "egdcabf", "bgf", "bfgea"],
        ),
        (
            [
                "fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg", "bafgc",
                "acf",
            ],
            ["gebdcfa", "ecba", "ca", "fadegcb"],
        ),
        (
            [
                "dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb", "gdcebf",
                "gf",
            ],
            ["cefg", "dcbef", "fcge", "gbcadfe"],
        ),
        (
            [
                "bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced", "adcbefg",
                "gebcd",
            ],
            ["ed", "bcgafe", "cdgba", "cbgef"],
        ),
        (
            [
                "egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb",
                "bfceg",
            ],
            ["gbdfcae", "bgc", "cg", "cgb"],
        ),
        (
            [
                "gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac",
                "fegbdc",
            ],
            ["fgae", "cfgab", "fg", "bagce"],
        ),
    ]
}

fn input() -> Vec<([&'static str; 10], [&'static str; 4])> {
    vec![
        (
            [
                "fbead", "dcabe", "bcega", "gfbecd", "ecd", "dgac", "cd", "bedcag", "agebcfd",
                "fcagbe",
            ],
            ["ced", "cgbefad", "gbcaef", "cd"],
        ),
        (
            [
                "bdef", "fbgeadc", "gabfc", "fadegb", "adefg", "abe", "be", "decagf", "dacgbe",
                "begfa",
            ],
            ["dbfe", "gaecdf", "fbgae", "agfbe"],
        ),
        (
            [
                "gad", "cgdfab", "ag", "edacgb", "agbfedc", "facdb", "dcebaf", "adfbg", "agfc",
                "gbdef",
            ],
            ["dga", "gacfebd", "adbgcf", "adgbce"],
        ),
        (
            [
                "cebadf", "gadcf", "egabfd", "ecabdfg", "gfcbd", "dacef", "geac", "ag", "afg",
                "fgdaec",
            ],
            ["aegc", "gdeabfc", "fgedac", "eacg"],
        ),
        (
            [
                "ceafb", "gcbfea", "eacgdf", "cegb", "eag", "bgfda", "eg", "fdaegcb", "ebagf",
                "adebcf",
            ],
            ["eg", "adgfb", "ge", "ceabf"],
        ),
        (
            [
                "gcdfeab", "bfdcga", "gdacf", "fe", "fae", "bdefga", "eafdgc", "fecd", "efagc",
                "aecgb",
            ],
            ["dcef", "fea", "cgfda", "gbcae"],
        ),
        (
            [
                "gbdce", "cfdbg", "efbd", "gbaedcf", "dge", "gfcead", "bcgae", "de", "bgacdf",
                "befdcg",
            ],
            ["gde", "fcbadg", "egdafc", "dbaefcg"],
        ),
        (
            [
                "adgecf", "agecdb", "dfcag", "cea", "dafgebc", "egbfc", "fdea", "ea", "agbfdc",
                "caefg",
            ],
            ["gcdfae", "aefcbdg", "fdgebac", "eac"],
        ),
        (
            [
                "dgcea", "acfged", "eagfc", "abdec", "dcg", "gd", "efcgdb", "adfebcg", "bceagf",
                "adfg",
            ],
            ["dfag", "dgebcaf", "dfbcage", "dgc"],
        ),
        (
            [
                "aedgbf", "dgfabec", "aefb", "edbgf", "gabdc", "ea", "gedfbc", "cgdefa", "dea",
                "baedg",
            ],
            ["ae", "ea", "eadbgfc", "fdabge"],
        ),
        (
            [
                "afgbde", "agbef", "df", "fda", "caegfb", "efdb", "fgdbeca", "fcdgab", "agedf",
                "cgead",
            ],
            ["fad", "df", "fd", "bedgcfa"],
        ),
        (
            [
                "bacdge", "gdecbaf", "bdfcge", "fabec", "dc", "dec", "cfbed", "gedbf", "dcgf",
                "agbefd",
            ],
            ["bcedf", "fgbed", "bcafe", "cfbedag"],
        ),
        (
            [
                "gbcafd", "bcd", "db", "begdc", "gedcfb", "bgdfeac", "cgdef", "dbfe", "gceab",
                "adecgf",
            ],
            ["dbc", "bd", "ebgdc", "bgcfda"],
        ),
        (
            [
                "ebafgd", "dcafeb", "ag", "gfadb", "aegb", "agd", "edbaf", "adcefg", "gcdabfe",
                "gdfcb",
            ],
            ["bgafd", "ga", "ga", "ag"],
        ),
        (
            [
                "gcbfda", "acd", "facdb", "aecbgf", "gafcb", "ad", "agbfced", "bacdge", "efdcb",
                "agdf",
            ],
            ["gbdeacf", "ad", "bgedca", "dcbagfe"],
        ),
        (
            [
                "gfae", "dae", "ea", "ecabfd", "edbgc", "aegdc", "cgfdae", "fcadgb", "facbedg",
                "gdfca",
            ],
            ["dfeagbc", "cdeabf", "dea", "ea"],
        ),
        (
            [
                "egafb", "ec", "dcegafb", "fcdag", "bcef", "cae", "cdgbae", "egbcfa", "abfdeg",
                "eafcg",
            ],
            ["bfacedg", "ace", "eca", "afgdc"],
        ),
        (
            [
                "bfgadc", "fabgd", "fcgb", "gecadf", "adceb", "gc", "cgd", "dabcg", "gadfcbe",
                "eadfbg",
            ],
            ["gbcf", "cebda", "cgd", "cg"],
        ),
        (
            [
                "abfdg", "dgabe", "cgbead", "efdb", "abfgced", "gefbda", "daf", "cfadeg", "cbgaf",
                "df",
            ],
            ["gfadb", "egbda", "bgdea", "bcadgef"],
        ),
        (
            [
                "aebcf", "agdbf", "adgbfce", "agbfec", "egcf", "acg", "bgacde", "cgbaf", "dfeabc",
                "cg",
            ],
            ["aefdbgc", "gc", "efadbc", "gc"],
        ),
        (
            [
                "bacdfe", "baefc", "edacb", "abcdg", "bafdge", "cgfbdea", "de", "cfebga", "cfde",
                "bde",
            ],
            ["ecfd", "bde", "cfbea", "fagebc"],
        ),
        (
            [
                "cefd", "cf", "afbegcd", "gfdcab", "adegfc", "baegf", "dgcea", "fcg", "cgebad",
                "gfeca",
            ],
            ["gdeafc", "fc", "decbgaf", "gedca"],
        ),
        (
            [
                "dgcae", "eadgbc", "eafdgcb", "ad", "badg", "deabcf", "gefdbc", "ade", "bdceg",
                "fcega",
            ],
            ["ade", "acedfgb", "acegfdb", "becfda"],
        ),
        (
            [
                "bfadecg", "cfadge", "bcgfd", "bceagf", "deabfg", "edg", "edca", "ecafg", "cfedg",
                "ed",
            ],
            ["cead", "deacfg", "ged", "gbfaedc"],
        ),
        (
            [
                "egafd", "dfbgcae", "bdf", "bfade", "dgbcfe", "bagf", "beacd", "fgbdae", "cefgda",
                "fb",
            ],
            ["fb", "gabf", "bfd", "cefagd"],
        ),
        (
            [
                "afedbcg", "eadcgf", "gebdcf", "agdfb", "ceagf", "baecfg", "de", "gaefd", "cead",
                "egd",
            ],
            ["bdeagfc", "ecda", "agbfd", "dgafce"],
        ),
        (
            [
                "bcde", "bc", "fgdceb", "bfc", "faecdbg", "adgfb", "cdfgb", "fbegca", "dgafce",
                "fdegc",
            ],
            ["cdgfbe", "fcgde", "cfb", "fcedg"],
        ),
        (
            [
                "bgacd", "cbf", "fceda", "fb", "gacbdf", "cadfb", "bdagce", "fegcbad", "beacfg",
                "fbgd",
            ],
            ["ecafbgd", "bfdg", "gcbdefa", "eacdbg"],
        ),
        (
            [
                "dgbec", "afecgdb", "fbgce", "dbgcaf", "becdfa", "bagcf", "efb", "ecfabg", "geaf",
                "fe",
            ],
            ["fegacbd", "ef", "fcbge", "abgfc"],
        ),
        (
            [
                "cedfg", "cgfaedb", "fgcdea", "gacdf", "gcae", "begdaf", "cdbaf", "fbcdeg", "ag",
                "gad",
            ],
            ["gcfebd", "cadfeg", "adfcb", "afcbd"],
        ),
        (
            [
                "edgcfba", "cdafb", "agd", "cbegfa", "dfagc", "acefg", "cgdbae", "dg", "dgceaf",
                "egfd",
            ],
            ["fgecba", "gad", "aebgcfd", "agfcd"],
        ),
        (
            [
                "afecdg", "bce", "gdeca", "cfdgbe", "bgca", "cfabegd", "daecb", "bfeda", "gabcde",
                "bc",
            ],
            ["bgca", "bec", "gabc", "bc"],
        ),
        (
            [
                "fgbdc", "gad", "ad", "eabdcgf", "gdfeac", "ebdfag", "baegfc", "gadfb", "ebad",
                "begaf",
            ],
            ["ebda", "dfabg", "adg", "bdae"],
        ),
        (
            [
                "bgec", "dbfga", "degfc", "eb", "deabfc", "fcdage", "bfe", "gfbced", "gfedb",
                "gfdcabe",
            ],
            ["bcge", "begc", "efb", "bcgdef"],
        ),
        (
            [
                "efbga", "bfeacg", "dg", "dfg", "afdce", "eafbgd", "adbg", "fdbcgae", "fcdebg",
                "eafdg",
            ],
            ["dbag", "fdg", "fgd", "gbad"],
        ),
        (
            [
                "cdabe", "agcbdef", "fgaed", "dbcgea", "fbce", "bfacdg", "fb", "faebd", "dbf",
                "fadcbe",
            ],
            ["bcgfda", "bf", "fb", "fb"],
        ),
        (
            [
                "cdafbe", "cbdagf", "fcgade", "fbage", "fbdae", "fdbca", "ed", "efd", "adfgceb",
                "bcde",
            ],
            ["ed", "adcbf", "bgdcfea", "ecbd"],
        ),
        (
            [
                "fgab", "gfacbd", "gfdcb", "fgdecb", "eagbdcf", "bdacg", "ag", "gad", "edgcfa",
                "eacdb",
            ],
            ["gcfdba", "fgdace", "cdegbfa", "agbdc"],
        ),
        (
            [
                "egcaf", "gdecbf", "beacg", "cfba", "dgfeabc", "abfgec", "baegd", "bc", "ecfagd",
                "ecb",
            ],
            ["bcaf", "cbaf", "gdecbf", "ebc"],
        ),
        (
            [
                "agfce", "cgedfb", "gbdfe", "daf", "da", "febgda", "cgfaedb", "adgb", "dgefa",
                "ecabdf",
            ],
            ["bdfceg", "dgab", "adf", "gadb"],
        ),
        (
            [
                "begadf", "deg", "aecfgb", "aebgd", "aebfg", "ceadgfb", "fead", "ed", "dbcga",
                "egdcbf",
            ],
            ["ged", "fcgeab", "gdaeb", "de"],
        ),
        (
            [
                "ad", "debcf", "deabgc", "acegbfd", "daef", "febcda", "cgebdf", "bcafd", "cda",
                "bfcga",
            ],
            ["acfgb", "cbdaeg", "gecadb", "acd"],
        ),
        (
            [
                "fadbeg", "bagdcf", "bcda", "fceag", "cfd", "bdgfa", "dabgefc", "cd", "bdcgfe",
                "afgcd",
            ],
            ["acbefdg", "dc", "gfcea", "bcda"],
        ),
        (
            [
                "fcbge", "bg", "cgdeaf", "bdegcf", "beg", "bfeca", "gbdc", "agdefcb", "gcdef",
                "bgefda",
            ],
            ["egb", "bgdfaec", "bg", "feacbdg"],
        ),
        (
            [
                "gcfabed", "dae", "geaf", "afdgc", "dfbcag", "cgadef", "dcebg", "defbac", "edagc",
                "ae",
            ],
            ["cdgae", "ae", "eadfcb", "egaf"],
        ),
        (
            [
                "ceabf", "fabcge", "cgfe", "gecabdf", "dgafcb", "bagcf", "edfagb", "ef", "cdeab",
                "feb",
            ],
            ["acbgf", "fabecg", "bfe", "badefg"],
        ),
        (
            [
                "adg", "bdfaeg", "bacd", "gcebd", "cgadbe", "da", "caedg", "egfca", "bfgdace",
                "fdbecg",
            ],
            ["dcab", "cgeda", "agd", "ad"],
        ),
        (
            [
                "cbdafge", "ecbg", "fabec", "caefgb", "cb", "abefd", "fcdega", "bac", "adbcfg",
                "efcga",
            ],
            ["cb", "gcbe", "bac", "dbgeafc"],
        ),
        (
            [
                "gfcbad", "fbc", "cf", "ebgaf", "acdbe", "decbfa", "dcbeag", "cdfe", "efgbdac",
                "fbaec",
            ],
            ["aebcgdf", "bfc", "dcaebfg", "edfbacg"],
        ),
        (
            [
                "dgcf", "bfaedg", "bdf", "cdagbef", "cbead", "df", "cebfg", "gdcfeb", "fbdce",
                "gabecf",
            ],
            ["fd", "begcf", "df", "df"],
        ),
        (
            [
                "bc", "egcb", "efabdc", "cfb", "gefdba", "cbfgade", "gbdef", "fdcga", "gfbcd",
                "gbdecf",
            ],
            ["cfagd", "ecbdaf", "cbf", "dbfcae"],
        ),
        (
            [
                "bc", "cbe", "cfega", "gbecaf", "fdcega", "dbefa", "cbgdfea", "efacb", "cdbega",
                "gfbc",
            ],
            ["dgceabf", "ebc", "bce", "fcgb"],
        ),
        (
            [
                "cf", "gfcdea", "fdage", "gfc", "dgcfe", "debcg", "fcbdag", "gefdba", "afec",
                "cgfdbae",
            ],
            ["deagbf", "eacdgf", "dgcbefa", "dfgec"],
        ),
        (
            [
                "cdgfbae", "bc", "ecabd", "cfabdg", "dfageb", "cfeda", "debga", "abc", "begdac",
                "egbc",
            ],
            ["bc", "cb", "acb", "adegbc"],
        ),
        (
            [
                "fd", "caegfd", "fed", "acfegdb", "bcfed", "cbadfe", "abfd", "bcaedg", "egcfb",
                "eabcd",
            ],
            ["dcabegf", "cbgdfea", "bfda", "cadbe"],
        ),
        (
            [
                "dbgac", "ebd", "faebdg", "afbcdg", "efcbg", "aedc", "dbgeca", "dcafgeb", "gcedb",
                "ed",
            ],
            ["aced", "edgbc", "ebd", "ed"],
        ),
        (
            [
                "fea", "dfbga", "bcedf", "fgadbce", "ea", "aebg", "bgfaed", "gdafec", "dbefa",
                "bdgcaf",
            ],
            ["abdef", "gdcafb", "dgbaf", "ebfad"],
        ),
        (
            [
                "eac", "afdecb", "dbegca", "acdfb", "dbacfge", "ea", "bcfea", "ebgfc", "fade",
                "fadcgb",
            ],
            ["fcbad", "ebcfa", "edcgba", "dacbf"],
        ),
        (
            [
                "agfdbc", "adcbg", "fdgc", "cdfeagb", "bgd", "eadbc", "edfagb", "dg", "abcfg",
                "bfaecg",
            ],
            ["cdfg", "dg", "gbeacfd", "bdg"],
        ),
        (
            [
                "fdbag", "gafcbe", "agfdcb", "bfc", "cdgf", "bdagef", "cfbda", "bdeac", "bcfedag",
                "cf",
            ],
            ["bdeca", "adfegb", "cbfad", "cf"],
        ),
        (
            [
                "agfdce", "ecd", "cbeadg", "ce", "bdagc", "ecagfbd", "bcegd", "abce", "bfedg",
                "gcbfda",
            ],
            ["cde", "cdbgaef", "ceadfg", "eabc"],
        ),
        (
            [
                "cgbda", "bceag", "bcgfde", "gdb", "dcabf", "dg", "badefgc", "gdae", "adbcge",
                "cabgfe",
            ],
            ["adcgb", "egdbca", "dgbfaec", "gdb"],
        ),
        (
            [
                "baefc", "gecfb", "bfadc", "dabcge", "cdbfage", "aeb", "ea", "bgafce", "ebdgfc",
                "efag",
            ],
            ["bfgdaec", "gcdebfa", "dbecgf", "dagcbe"],
        ),
        (
            [
                "bgdcfe", "cafdg", "cad", "acegf", "fgcadb", "fdcbg", "faedbc", "da", "gbecfda",
                "dabg",
            ],
            ["acd", "faecg", "afcdg", "fdbgc"],
        ),
        (
            [
                "gedbcf", "gef", "cfaedbg", "egbc", "ge", "gfcaed", "dcbfea", "fgbad", "dgbfe",
                "fecbd",
            ],
            ["eg", "gfe", "gcbe", "bcge"],
        ),
        (
            [
                "fca", "cbfge", "faceg", "fbeadg", "gdefa", "ac", "adcfbeg", "ceda", "cdbgaf",
                "dgaefc",
            ],
            ["gfabcd", "afc", "degabf", "fac"],
        ),
        (
            [
                "dacfb", "edbag", "ce", "abdfgc", "dcfe", "ebcda", "gecfab", "befacgd", "feadbc",
                "aec",
            ],
            ["cea", "dabcegf", "cea", "badfcg"],
        ),
        (
            [
                "edabgcf", "eadcgf", "dbgecf", "dgeca", "dcbaf", "adcfg", "fgd", "gf", "faeg",
                "decbga",
            ],
            ["bfcdeg", "dgfca", "edcafbg", "dcafbeg"],
        ),
        (
            [
                "abegcf", "bedaf", "fc", "ebadgc", "cbf", "fbcagd", "baecg", "befac", "defgacb",
                "cgef",
            ],
            ["ecbaf", "fbcdage", "cf", "bfc"],
        ),
        (
            [
                "acg", "bafgdc", "afegb", "bfacd", "bgeacfd", "fgdc", "cg", "aecdbf", "gdaebc",
                "cbafg",
            ],
            ["afdcb", "cbfgda", "acg", "abdfc"],
        ),
        (
            [
                "ca", "ace", "afcb", "afebcg", "ecfbg", "dabeg", "gfacde", "gbace", "cfgabde",
                "cdbfge",
            ],
            ["acbf", "cfegb", "gebad", "bacf"],
        ),
        (
            [
                "dgfcba", "fabcd", "cgbfae", "dbaegcf", "eadcf", "dagbc", "edbcag", "gbfd", "fb",
                "bfc",
            ],
            ["ebagdc", "fb", "edfgabc", "gadcb"],
        ),
        (
            [
                "gdaec", "dfgb", "dcgbefa", "bde", "ebadg", "cbfage", "cabfed", "efagb", "db",
                "bdgafe",
            ],
            ["bd", "abegf", "dbe", "deb"],
        ),
        (
            [
                "caebfg", "bf", "bgead", "dgecab", "bdaf", "bgdafe", "fbe", "dfgbeac", "bfged",
                "fgedc",
            ],
            ["cbgeda", "efdbg", "edacgbf", "feb"],
        ),
        (
            [
                "begca", "fbaedgc", "adbfgc", "adbecg", "bcaefg", "af", "afb", "cdbfe", "feag",
                "beafc",
            ],
            ["fadgcbe", "af", "bgdfcea", "af"],
        ),
        (
            [
                "dceab", "be", "gcbdaf", "ebc", "abfe", "bdfac", "cdbafe", "cagdfbe", "gbfced",
                "dgcae",
            ],
            ["acfgbed", "eb", "afbe", "eabf"],
        ),
        (
            [
                "fegdbc", "acfdb", "afgce", "badecf", "badg", "ebgafdc", "gd", "cgadf", "gdf",
                "dcabfg",
            ],
            ["agcfd", "fbdac", "gfd", "egbafdc"],
        ),
        (
            [
                "beagd", "fbcg", "gbecfad", "gc", "bacge", "adbefc", "aecfb", "acgdfe", "gfbcea",
                "egc",
            ],
            ["gc", "efacbgd", "agdeb", "cfbg"],
        ),
        (
            [
                "adcebf", "fgebadc", "ga", "ega", "bdga", "edafb", "egadf", "bcgeaf", "dcefg",
                "afdbge",
            ],
            ["abdg", "dbag", "edcfg", "fbegadc"],
        ),
        (
            [
                "fe", "efd", "egfbdac", "dcagf", "cgaebd", "cbfe", "gdceb", "dfegc", "efagbd",
                "fdegcb",
            ],
            ["ef", "gdaceb", "befcgd", "gdcfbe"],
        ),
        (
            [
                "badf", "bfg", "dafgebc", "gdabe", "edfbg", "bgfeac", "bf", "bdagef", "dbaegc",
                "fecgd",
            ],
            ["gacfbe", "bdagef", "cgaebd", "adfb"],
        ),
        (
            [
                "efdabg", "bedac", "fb", "cgedab", "cefgdab", "fcdga", "adfcb", "cbfe", "dfb",
                "ecfabd",
            ],
            ["bfd", "bfec", "debafc", "acbfd"],
        ),
        (
            [
                "cbdag", "fa", "bdfacg", "dgfac", "defbag", "cbaf", "gecbda", "agf", "dbegafc",
                "fgcde",
            ],
            ["gdcef", "cdbag", "gaf", "gcfde"],
        ),
        (
            [
                "cfagd", "fbagcde", "cb", "efdabc", "dgaefb", "cfbe", "adgbec", "bdc", "acfbd",
                "edfba",
            ],
            ["befc", "cfbe", "efbc", "bcd"],
        ),
        (
            [
                "bdcgfe", "acdbeg", "ac", "caed", "gdfba", "dagbc", "abc", "gbfcea", "ecbdg",
                "gbcdaef",
            ],
            ["aced", "dgbce", "cab", "gafdb"],
        ),
        (
            [
                "bgfd", "fecdgab", "eadcf", "acbged", "gd", "acfgdb", "cafegb", "cdgfa", "cgd",
                "cfgba",
            ],
            ["fdgb", "gecadb", "gd", "gdc"],
        ),
        (
            [
                "dbcfa", "gcebda", "becg", "dgeac", "dbe", "egadcf", "gdefab", "fgaecbd", "eb",
                "acdeb",
            ],
            ["bed", "cgbe", "ebd", "efbgdca"],
        ),
        (
            [
                "ebfd", "cgfdea", "fbdagec", "afbce", "bfdeac", "geacb", "fb", "dfabgc", "fbc",
                "dacef",
            ],
            ["fcb", "abcdef", "cfb", "fb"],
        ),
        (
            [
                "gceab", "bgef", "gbdca", "abdecf", "efacg", "ecbagf", "ebc", "cgdeaf", "eb",
                "gdfbcea",
            ],
            ["ebc", "fdcbgea", "eb", "fcedbag"],
        ),
        (
            [
                "cdfeg", "de", "edg", "dbcgaf", "defbcg", "cbfgd", "gfcdabe", "bdef", "gdebca",
                "efagc",
            ],
            ["bfacegd", "fdceagb", "fbed", "gcebfad"],
        ),
        (
            [
                "fecbdg", "fcagebd", "adbgfc", "ceba", "gecad", "dgfea", "egdcba", "dac", "gcedb",
                "ca",
            ],
            ["cdabgf", "edfacgb", "acbgde", "febacdg"],
        ),
        (
            [
                "gefacd", "febcga", "fdg", "gcfdb", "ebcgf", "cgdba", "gdcefb", "fd", "bfedgac",
                "bedf",
            ],
            ["bdagfce", "gfd", "bfdcge", "geacdfb"],
        ),
        (
            [
                "bacgfd", "ec", "cgbafe", "adbgfec", "acfbe", "fbeda", "eac", "ecgdba", "acbfg",
                "fceg",
            ],
            ["dcgaeb", "fcdgabe", "aec", "afdbe"],
        ),
        (
            [
                "egcd", "dfeca", "efagd", "fdcgae", "egf", "ecbdagf", "eabdfc", "fecabg", "dfagb",
                "ge",
            ],
            ["feacd", "eg", "gcde", "feg"],
        ),
        (
            [
                "begaf", "dcfegb", "dbcge", "gadcbef", "fc", "cdabfe", "fgdc", "decbga", "cfb",
                "becgf",
            ],
            ["bfc", "fc", "cdebg", "cgfd"],
        ),
        (
            [
                "cfbdgea", "fdg", "efbdc", "fgdecb", "gd", "gbeaf", "agdfcb", "cedg", "adfebc",
                "gfdbe",
            ],
            ["cfdbe", "afgcdb", "dfcebga", "dgec"],
        ),
        (
            [
                "fdebgc", "egba", "dcaebfg", "abfgcd", "ba", "bda", "gdceb", "cefda", "degcba",
                "edcba",
            ],
            ["dba", "bdafegc", "decba", "bad"],
        ),
        (
            [
                "cgeb", "bdgcfa", "dfgcb", "efcdg", "ge", "abfged", "efdac", "eabdcfg", "ecdfgb",
                "gef",
            ],
            ["cedfg", "eg", "fcade", "eg"],
        ),
        (
            [
                "eabdcg", "gceb", "ec", "ecfadg", "baecd", "daebcgf", "afdeb", "badgc", "bafdcg",
                "ace",
            ],
            ["bcdgeaf", "ec", "cebadfg", "ce"],
        ),
        (
            [
                "ebdf", "cagfeb", "efcda", "ecbdag", "ecdbfa", "aebdc", "cgdfa", "eaf", "caefgbd",
                "ef",
            ],
            ["eaf", "dcbea", "acgdbe", "dfcbega"],
        ),
        (
            [
                "gdbfa", "gfaeb", "cafbgd", "adb", "cedbgaf", "faecdg", "edfbca", "bd", "dcgb",
                "gcadf",
            ],
            ["db", "bcdg", "dgfba", "dbgc"],
        ),
        (
            [
                "fbcdga", "cea", "gbcaef", "decgb", "gcdfa", "eafd", "afcdge", "adgec", "ea",
                "dbceagf",
            ],
            ["eac", "egbcdfa", "ae", "dfea"],
        ),
        (
            [
                "beagcd", "egcbd", "gbae", "eagcdf", "gdcefb", "cbade", "fabgdce", "ae", "fbcda",
                "ace",
            ],
            ["bage", "cae", "ace", "ageb"],
        ),
        (
            [
                "bacgf", "bdgac", "dcebfg", "eacgbd", "fdgbcea", "gfdbea", "dc", "aced", "bdc",
                "dbega",
            ],
            ["bcd", "cd", "bagde", "bafcg"],
        ),
        (
            [
                "gfabdc", "be", "gcedba", "fecabgd", "edcaf", "egbf", "fcabg", "cgbaef", "abcef",
                "bea",
            ],
            ["acegdfb", "fdbcga", "eb", "becaf"],
        ),
        (
            [
                "dgfbec", "bdecga", "dfgca", "fd", "aedcgfb", "adgec", "adef", "fcd", "fgcab",
                "fcegda",
            ],
            ["cbfgeda", "cdf", "df", "fdae"],
        ),
        (
            [
                "cgfbe", "faceg", "bacegf", "dfgbe", "aefcdg", "abdcef", "cb", "gdbefac", "bfc",
                "gbca",
            ],
            ["cdgefab", "bc", "cbfead", "cgaefd"],
        ),
        (
            [
                "gefdcab", "ebcg", "fcb", "bfdce", "afedb", "fedgc", "bedfcg", "dcafgb", "degfca",
                "cb",
            ],
            ["bc", "bc", "dbfecga", "cb"],
        ),
        (
            [
                "efagc", "cagfde", "fb", "bfe", "begfa", "gbead", "badcgfe", "bgfeac", "fbca",
                "fdgbec",
            ],
            ["eacfbg", "efb", "feb", "bf"],
        ),
        (
            [
                "gebdca", "af", "afb", "afbed", "bcafgd", "fabgde", "fbedc", "afge", "dgbae",
                "eacdbfg",
            ],
            ["agdfebc", "af", "abegcdf", "baf"],
        ),
        (
            [
                "egbad", "cfedbg", "gb", "dbg", "cfdaeg", "bdeaf", "dgabefc", "cdbega", "cgdea",
                "cabg",
            ],
            ["efdab", "fedab", "gb", "gcba"],
        ),
        (
            [
                "adfcb", "defca", "cabg", "gdefab", "afbdg", "bdgfec", "gbcedaf", "dbc", "gdacbf",
                "bc",
            ],
            ["dfeac", "bdc", "cbd", "gabc"],
        ),
        (
            [
                "db", "ebcda", "ebgfac", "fcdea", "becga", "ebgadcf", "bdc", "cfedbg", "agdb",
                "bdgeca",
            ],
            ["fecda", "bd", "agbd", "db"],
        ),
        (
            [
                "bagcef", "begdafc", "fbga", "fceab", "afcged", "ecagf", "deacgb", "acb", "ba",
                "bcefd",
            ],
            ["cfbedga", "abc", "deabgfc", "bca"],
        ),
        (
            [
                "cbf", "gdbcae", "abdgc", "fb", "dagbcf", "gabf", "cdfea", "cabdf", "dabegfc",
                "fbgcde",
            ],
            ["cbf", "fb", "dgabce", "fbc"],
        ),
        (
            [
                "dfbgec", "fcbeg", "fbeag", "afdecb", "fcdbga", "cegd", "dfbcg", "efc", "ce",
                "adebgcf",
            ],
            ["egdc", "acegbdf", "agfbcd", "cbgfe"],
        ),
        (
            [
                "abgce", "gdec", "ce", "faebg", "agcbd", "bfcadg", "acefdbg", "defcab", "egcdab",
                "cea",
            ],
            ["gdfabc", "ce", "agefdbc", "ec"],
        ),
        (
            [
                "acedg", "gbc", "geab", "ebdfc", "bg", "cbedg", "dacebg", "gdcaef", "fabgecd",
                "dagbcf",
            ],
            ["cgb", "bg", "adecg", "acged"],
        ),
        (
            [
                "cfebad", "fbgda", "edg", "ceag", "agebd", "aedgcfb", "eg", "ecbfdg", "cgabde",
                "cebda",
            ],
            ["cbafegd", "gdacebf", "cega", "eg"],
        ),
        (
            [
                "eacdf", "ea", "eadb", "cfgde", "cabfde", "bdfgac", "gacebf", "afe", "fbcda",
                "gcbafde",
            ],
            ["bagcfe", "fae", "adfcbg", "cadbf"],
        ),
        (
            [
                "abd", "fgcaebd", "dbfgc", "gdeac", "fbedag", "ceba", "bcgad", "deacgb", "ab",
                "adcfge",
            ],
            ["dba", "bcea", "ab", "cfagbed"],
        ),
        (
            [
                "afg", "bgfce", "af", "dcfa", "acbdge", "bacfg", "agbfde", "dgcfab", "dgbac",
                "defagcb",
            ],
            ["dfgcab", "cfgbad", "afg", "dceagb"],
        ),
        (
            [
                "gdacbf", "ebgacf", "edbagc", "ae", "agcbf", "aefg", "acfbe", "fgbcead", "bcefd",
                "cea",
            ],
            ["eca", "adgfcb", "eca", "eafg"],
        ),
        (
            [
                "fcbae", "bea", "bcdaf", "cadfbg", "fbgec", "ae", "eafd", "dacfeb", "bagcdef",
                "cbagde",
            ],
            ["fecbg", "cgedbaf", "gdcfba", "bae"],
        ),
        (
            [
                "baefg", "daebgc", "bdgeacf", "gaecb", "efb", "feca", "abfecg", "fadgb", "edcbgf",
                "fe",
            ],
            ["bfgdeca", "gcbea", "acedgbf", "ef"],
        ),
        (
            [
                "cd", "fdaecbg", "cdbga", "befacg", "fbdag", "decb", "dca", "gfcead", "dacebg",
                "gebac",
            ],
            ["edbc", "feabcg", "gcefdba", "adc"],
        ),
        (
            [
                "agdebf", "daeg", "eadbfc", "gef", "dgbfec", "eg", "dbeaf", "afgeb", "fedbcga",
                "gcbfa",
            ],
            ["eg", "eg", "edgfbc", "efabd"],
        ),
        (
            [
                "facd", "cabfgd", "fab", "geadbfc", "af", "gcbad", "egcdab", "becfg", "beafgd",
                "acfbg",
            ],
            ["fa", "acdfbge", "gdcaebf", "dbacg"],
        ),
        (
            [
                "bcgfda", "gbeaf", "cfgba", "fae", "bace", "cfabge", "cfdgae", "gedbf", "ea",
                "dgcfbae",
            ],
            ["ae", "ea", "fcagb", "eafdcbg"],
        ),
        (
            [
                "dbe", "gdcb", "ecafd", "edcgfba", "acedb", "gebdfa", "bagdce", "fegcba", "bd",
                "gbace",
            ],
            ["bd", "feacbdg", "bd", "gedbfa"],
        ),
        (
            [
                "cabgde", "fagecb", "efb", "adfbg", "acebdf", "fecd", "afbdcge", "cdbae", "bedfa",
                "fe",
            ],
            ["cfabge", "fe", "dfabg", "bfdea"],
        ),
        (
            [
                "bacg", "dafcg", "cdgfaeb", "fbcgda", "baefdg", "gda", "dabcef", "gfced", "cfadb",
                "ag",
            ],
            ["bgac", "dcfge", "egcbdfa", "dabfcge"],
        ),
        (
            [
                "bdc", "gdac", "ecdagbf", "geacfb", "defcbg", "fabde", "cafbg", "bcfad", "cd",
                "cadbgf",
            ],
            ["acbdgfe", "fegcabd", "dafbc", "dcgbefa"],
        ),
        (
            [
                "cedbf", "ebdgcf", "degc", "dcfgbae", "aebfg", "dfg", "bedgf", "gd", "aebcfd",
                "dbgfca",
            ],
            ["dbfcage", "cdebf", "dg", "dg"],
        ),
        (
            [
                "edcfb", "ad", "cgad", "adgbfc", "agcfb", "agfebc", "bdacf", "cafgedb", "daf",
                "dgaebf",
            ],
            ["afcdgeb", "fda", "fda", "bfagecd"],
        ),
        (
            [
                "agcbfde", "gadbcf", "bdfcge", "defgc", "dc", "afdge", "dcg", "gabfec", "ecbd",
                "bfgec",
            ],
            ["gcedf", "bcfeadg", "cdeb", "eafdg"],
        ),
        (
            [
                "afdebg", "efa", "afbdc", "ecag", "egcfd", "debgcf", "ea", "fgebadc", "fdeagc",
                "defca",
            ],
            ["aef", "afdbceg", "ea", "dgeafc"],
        ),
        (
            [
                "cgdfb", "fg", "ecdbf", "fbeacd", "bgdaef", "bcadg", "ecfgdab", "dgbfce", "gfec",
                "gfd",
            ],
            ["gf", "gf", "gfce", "febdc"],
        ),
        (
            [
                "bgcde", "fedgc", "cgf", "gabcfd", "gdbefc", "defga", "fcbe", "cdabegf", "fc",
                "acbdge",
            ],
            ["bcfe", "efbc", "fc", "efdcgb"],
        ),
        (
            [
                "cagb", "ab", "gdaecf", "cgeabd", "edabg", "fcdabe", "edbfcga", "bfedg", "cgdea",
                "eab",
            ],
            ["gadce", "gdefb", "abgc", "fedcba"],
        ),
        (
            [
                "ecdafg", "feg", "bcgfad", "egbfa", "afbgc", "ecgfdab", "gaefcb", "eg", "afdeb",
                "becg",
            ],
            ["ge", "bgfdca", "eg", "fegadc"],
        ),
        (
            [
                "fe", "adefg", "egdac", "cbagdef", "dcfeag", "adecfb", "dbfag", "edbgca", "gfce",
                "efd",
            ],
            ["fe", "efgc", "cabegd", "fe"],
        ),
        (
            [
                "geab", "fgbaec", "gce", "adefc", "gcafdb", "agcef", "abgcf", "cdgebaf", "gcedbf",
                "ge",
            ],
            ["gabe", "cfega", "fbdceg", "eg"],
        ),
        (
            [
                "eac", "dgefca", "cfgdba", "gdfeabc", "eadf", "gcfbe", "adfcg", "gcfae", "gacbed",
                "ae",
            ],
            ["agcdfeb", "ecbfg", "gedabc", "ae"],
        ),
        (
            [
                "ec", "cdagb", "cegdab", "dagbcf", "cbaed", "ebgc", "ecd", "abedf", "fadcge",
                "cebdafg",
            ],
            ["ecd", "gacfdb", "becg", "ec"],
        ),
        (
            [
                "abfge", "ce", "cgfdae", "cbed", "fecab", "fbdca", "egfacbd", "efc", "acgdbf",
                "cebdaf",
            ],
            ["dgface", "efc", "ec", "ec"],
        ),
        (
            [
                "gbc", "gcdbf", "gb", "eabdfc", "egfb", "agdcf", "egbcda", "gcdfeab", "dgefcb",
                "fcedb",
            ],
            ["bgef", "gecbfd", "gb", "bgfdce"],
        ),
        (
            [
                "ecgab", "gecabf", "cb", "gfbc", "abc", "afgeb", "daecg", "fcedab", "fcagbde",
                "efdbag",
            ],
            ["bdecagf", "gbfea", "ebgcaf", "cb"],
        ),
        (
            [
                "ecdgb", "ab", "adgb", "dbecga", "fgbcde", "cadgfbe", "aeb", "eacgbf", "cabed",
                "eacdf",
            ],
            ["abdg", "cbedg", "gdabce", "cdbeg"],
        ),
        (
            [
                "bgc", "dgeb", "bg", "fcbge", "befcgd", "ebcdfa", "cbfgda", "efcag", "cebfd",
                "fagbcde",
            ],
            ["gdeb", "fcgadeb", "bfecd", "cdagbef"],
        ),
        (
            [
                "edbcfga", "cedbga", "bagfd", "eabc", "dgbfce", "eb", "gceadf", "egb", "agdec",
                "gbade",
            ],
            ["cfgead", "gefdcab", "bdcgae", "cbae"],
        ),
        (
            [
                "bafcge", "edfg", "abegd", "deabcfg", "dfbagc", "abcde", "dg", "bdg", "gbdaef",
                "febga",
            ],
            ["bdg", "fdeg", "dg", "cafbdg"],
        ),
        (
            [
                "cgfdba", "gec", "gdeafbc", "cfgead", "ce", "fbegd", "cdae", "facdg", "afcbeg",
                "edfcg",
            ],
            ["cbgfea", "cfgadeb", "gdabfce", "egcdaf"],
        ),
        (
            [
                "cbfgad", "bceagd", "dbf", "cdbfae", "ebdcf", "cdegfab", "cbaed", "gcedf", "bf",
                "aefb",
            ],
            ["aefb", "dcfbage", "bedfcga", "fb"],
        ),
        (
            [
                "cgd", "bcdafe", "dacbegf", "adecb", "dbfeg", "bacg", "gedbc", "gc", "fcegad",
                "agbdce",
            ],
            ["cdg", "fbdge", "ecdgb", "eadgfbc"],
        ),
        (
            [
                "gfdabe", "cfbga", "ceab", "eaf", "gefcab", "gcaef", "ea", "efcdg", "dgafcb",
                "abfdegc",
            ],
            ["ea", "cafbdeg", "fgcab", "acgfe"],
        ),
        (
            [
                "dfg", "gdfbe", "gf", "ceagdf", "aefdb", "fcbg", "begdc", "cgbdef", "cabged",
                "becgafd",
            ],
            ["fdbgce", "eadbf", "cgedb", "fbcg"],
        ),
        (
            [
                "fdeba", "ecdfa", "badgef", "ebagf", "abegdc", "dgfb", "bd", "febgca", "agfedbc",
                "abd",
            ],
            ["bd", "bfega", "bedgca", "cbaegf"],
        ),
        (
            [
                "bafcdg", "cegb", "acfbg", "feagd", "fdacbge", "be", "bfegac", "abe", "bdcaef",
                "afebg",
            ],
            ["be", "bfecdga", "defcgba", "be"],
        ),
        (
            [
                "bafgdc", "dbfag", "gfecdb", "cbf", "cf", "bfadge", "acgfb", "cgfdbae", "bgeca",
                "cadf",
            ],
            ["cf", "bfgcde", "gdaefbc", "cfb"],
        ),
        (
            [
                "gcdbaf", "cgbedaf", "ad", "ebfagc", "daef", "dbafec", "caedb", "bgcde", "bfeca",
                "abd",
            ],
            ["ebdagcf", "dfea", "fcaedb", "cbfgad"],
        ),
        (
            [
                "ceagfd", "eadfb", "fdeag", "dcgfe", "gea", "ag", "dagcbe", "fcegbd", "fbdaceg",
                "fcga",
            ],
            ["gfca", "fcga", "dcgefb", "gfcaebd"],
        ),
        (
            [
                "cdfbge", "dcafe", "df", "fcd", "badce", "gdaf", "agcfe", "ecagbdf", "eafbcg",
                "fgcdea",
            ],
            ["gdfa", "fcd", "eabfcg", "fd"],
        ),
        (
            [
                "ad", "bgdfae", "eagfd", "fedcg", "dagb", "gdeabcf", "dea", "bgcefa", "abgef",
                "fdbaec",
            ],
            ["afbge", "eacfbdg", "bgecfa", "fabge"],
        ),
        (
            [
                "adfbceg", "edcgf", "gcabd", "egfcbd", "ecbf", "cebgd", "cefdga", "be", "deb",
                "fedgab",
            ],
            ["fecgd", "gcdef", "ecfdbga", "daecfg"],
        ),
        (
            [
                "gdfeac", "afedc", "agdbcfe", "ae", "begcfa", "edfbc", "bfcgda", "dgea", "efa",
                "cafdg",
            ],
            ["abdcfge", "edag", "edfbc", "cfdga"],
        ),
        (
            [
                "cbdeg", "geafbc", "fd", "gfacdb", "fgdce", "fecgad", "feda", "dfg", "aedbcfg",
                "gcafe",
            ],
            ["fdecg", "fade", "fade", "cadefbg"],
        ),
        (
            [
                "af", "bcfeg", "gbcfda", "cgabfe", "ecgfa", "abef", "cgdae", "beagdfc", "gaf",
                "gebcfd",
            ],
            ["gdfecb", "afegbdc", "gfa", "gaf"],
        ),
        (
            [
                "bgafcd", "becfag", "cbdeag", "bagdc", "eb", "bdegc", "adbe", "cdfebag", "gbe",
                "egfcd",
            ],
            ["edbgac", "adgbc", "bge", "cefgba"],
        ),
        (
            [
                "dfcebg", "cgdaef", "ga", "fcbgd", "dgabfc", "debaf", "acgb", "agfbd", "defbacg",
                "gad",
            ],
            ["bagc", "bdeaf", "bfgdc", "abcg"],
        ),
        (
            [
                "ebgfac", "agfdceb", "cdbga", "dafc", "cfabg", "dc", "gebda", "dcfbeg", "dbc",
                "gafcbd",
            ],
            ["gfbdca", "fdac", "gdfabec", "bagfcde"],
        ),
        (
            [
                "egbac", "efabgc", "efagcdb", "abedg", "ead", "da", "dbac", "bedcga", "cegdaf",
                "gdfeb",
            ],
            ["dabcfeg", "gcfeda", "adbc", "ade"],
        ),
        (
            [
                "acgbfe", "eafdg", "dbea", "bfgea", "dgbcaf", "cedbagf", "dfgabe", "cdgfe", "gda",
                "da",
            ],
            ["gfebdac", "agd", "dgaef", "dga"],
        ),
        (
            [
                "edbgf", "ced", "bcdeaf", "cdgeaf", "bcfgade", "fbcad", "eabc", "cdgfba", "ec",
                "febdc",
            ],
            ["dcbef", "faedgc", "edc", "begfd"],
        ),
        (
            [
                "ca", "bedcaf", "cbaedgf", "edabf", "cab", "dfbca", "bdfgc", "cfea", "daegcb",
                "gbafde",
            ],
            ["ac", "cdfabe", "bcgdf", "gdfceab"],
        ),
        (
            [
                "bdef", "gecbfd", "gbdcf", "cbaegf", "cgbda", "dfc", "gdefacb", "fcaegd", "fd",
                "gebcf",
            ],
            ["cfd", "dcf", "df", "fegdcab"],
        ),
        (
            [
                "bf", "adfgce", "dcbea", "cfb", "gfdaceb", "cedfbg", "cfedb", "fcedg", "acdgbf",
                "gefb",
            ],
            ["fb", "bgfe", "gebf", "dacbegf"],
        ),
        (
            [
                "defc", "edfba", "eadbg", "feb", "fbagcde", "ef", "dafcb", "gbfdca", "egcfab",
                "aecfdb",
            ],
            ["ecdf", "ef", "fcbgae", "fced"],
        ),
        (
            [
                "acdbe", "aegb", "cdabfge", "ebdcg", "fcabd", "ea", "gebdca", "bfcgde", "gcefda",
                "aed",
            ],
            ["ea", "ebcgd", "dbcegfa", "dea"],
        ),
        (
            [
                "dbcfe", "bfcda", "cabfedg", "aefc", "cdfabg", "bgedc", "ebdfac", "edf", "fe",
                "efdbga",
            ],
            ["efca", "face", "fe", "cefa"],
        ),
        (
            [
                "egdab", "bgfd", "gdeabcf", "eadbc", "abg", "bg", "bacefg", "fgade", "cadgef",
                "afbgde",
            ],
            ["bfcaegd", "dbafge", "abedg", "geabd"],
        ),
        (
            [
                "ebc", "egbfdc", "degc", "ebcagf", "gbcfd", "acbdfg", "ec", "fcdeb", "abdcgef",
                "abfde",
            ],
            ["egdc", "gcdfb", "bcgdf", "ce"],
        ),
        (
            [
                "dbfaecg", "bd", "bcegdf", "cfabg", "dcba", "abfdg", "fbeagc", "fdb", "cbdfga",
                "efgad",
            ],
            ["bgaefc", "efadg", "gbcafe", "bfd"],
        ),
        (
            [
                "gacef", "dcaegb", "egadc", "efcbadg", "eaf", "caefdg", "defc", "fe", "afgebd",
                "agcbf",
            ],
            ["cedag", "dfce", "cedfgba", "aef"],
        ),
        (
            [
                "gafcbed", "gdcef", "bcefad", "daecf", "adc", "daefb", "cbaf", "egacdb", "bgadef",
                "ac",
            ],
            ["dbceafg", "ebfcad", "bdacfe", "dca"],
        ),
        (
            [
                "ecbfa", "fae", "afgbc", "cdeafg", "dcfeb", "efgcab", "ae", "beag", "gbadecf",
                "acfbdg",
            ],
            ["bagfdc", "ae", "feabc", "edcgfa"],
        ),
        (
            [
                "egfac", "gebaf", "ac", "cfab", "gca", "gcdef", "cagedb", "bfecag", "fagdbe",
                "gacfebd",
            ],
            ["gcbaedf", "geacbdf", "gafcebd", "cgefba"],
        ),
        (
            [
                "fbdgec", "efbcag", "acbedgf", "bac", "befcg", "adfbec", "fgca", "ca", "bagce",
                "gebad",
            ],
            ["egcbf", "ca", "cfgabde", "abc"],
        ),
        (
            [
                "edafg", "cabe", "fcaed", "ce", "faegdcb", "dcefgb", "fec", "fdagcb", "cbdaf",
                "cbfaed",
            ],
            ["acbdef", "cef", "ce", "eacbfdg"],
        ),
        (
            [
                "efbcgd", "ebdgc", "egacf", "gebadc", "gba", "dabe", "fagcbde", "gbcae", "ab",
                "gfcbad",
            ],
            ["aebd", "aedb", "bead", "cdbfgae"],
        ),
        (
            [
                "becag", "cbdf", "cebda", "fgdbea", "bdgafce", "ecfda", "abd", "adefcb", "acgdef",
                "bd",
            ],
            ["adbfegc", "eadgbfc", "dcegafb", "bfdc"],
        ),
        (
            [
                "gfcadb", "fega", "acbeg", "fgcab", "ebcda", "ebcdfg", "ge", "ebcfga", "gabfdec",
                "ebg",
            ],
            ["beg", "fgedbac", "eagf", "eagdfcb"],
        ),
        (
            [
                "gecf", "cbf", "cbfga", "egfbac", "cbgaefd", "dgcbae", "fc", "aebcg", "befdca",
                "bfadg",
            ],
            ["cbf", "bgfca", "fbc", "fcb"],
        ),
        (
            [
                "cedfg", "adbgef", "cb", "bcd", "cgebd", "dageb", "cbegad", "ceab", "bgfaecd",
                "cabfdg",
            ],
            ["gbced", "bc", "dgefc", "bc"],
        ),
        (
            [
                "egba", "bdcfg", "fbe", "acegf", "ebcfg", "eacbfg", "eb", "fdebac", "dgeacf",
                "dcgbaef",
            ],
            ["efgacd", "bef", "ebcgf", "fbe"],
        ),
        (
            [
                "agfcbd", "bafcedg", "dce", "bgdfce", "efbac", "dgfe", "begdca", "de", "gdfcb",
                "dbefc",
            ],
            ["ced", "ebgcafd", "ed", "bgdaec"],
        ),
        (
            [
                "decfa", "cgbdae", "bdfce", "dgcbe", "bdfg", "bgdefc", "fbe", "bf", "bfgcae",
                "ebcgdaf",
            ],
            ["fb", "gabcef", "efcdb", "gdcebf"],
        ),
        (
            [
                "fbecga", "efcbg", "bafgd", "ac", "dcgeab", "eacf", "bafcg", "gfdbace", "gbcefd",
                "acb",
            ],
            ["bfecga", "efgbdac", "fbegc", "faec"],
        ),
        (
            [
                "ab", "ecbagdf", "fbagec", "cfedb", "eagbdc", "acb", "agdb", "ecgda", "acedb",
                "gacfde",
            ],
            ["acdbfeg", "dgab", "abc", "dgbfcea"],
        ),
        (
            [
                "gfdbc", "begdcf", "bfedg", "acgfed", "cd", "fcd", "febgda", "cfgab", "cadbgef",
                "ebdc",
            ],
            ["ebcd", "gdefab", "ebdc", "ebafdcg"],
        ),
    ]
}
