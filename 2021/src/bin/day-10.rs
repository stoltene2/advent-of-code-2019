fn main() {
    let test_input = vec![
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    let test_result = test_input
        .iter()
        .fold(0, |total, input| total + score_syntax(input));

    assert_eq!(26397, test_result);

    let result = input()
        .iter()
        .fold(0, |total, input| total + score_syntax(input));

    println!("result: {}", result);
}

fn score_syntax(input: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();
    for bracket in input.chars() {
        match bracket {
            '{' | '[' | '(' | '<' => {
                stack.push(bracket);
            }
            '}' | ']' | ')' | '>' => {
                if let Some(last_bracket) = stack.last() {
                    if *last_bracket == is_pair(bracket) {
                        stack.pop();
                    } else {
                        return score(bracket);
                        // println!(
                        //     "Oops, expected: {} but found {}",
                        //     is_pair(*last_bracket),
                        //     bracket
                        // );
                    }
                }
            }
            _ => panic!("Unexected {}", bracket),
        }
    }

    // incomplete input
    if stack.len() > 0 {
        return 0;
    }

    0
}

fn score(bracket: char) -> usize {
    match bracket {
        ']' => 57,
        '>' => 25137,
        ')' => 3,
        '}' => 1197,
        _ => 0,
    }
}

fn is_pair(bracket: char) -> char {
    match bracket {
        '{' => '}',
        '(' => ')',
        '<' => '>',
        '[' => ']',
        '}' => '{',
        ')' => '(',
        '>' => '<',
        ']' => '[',
        _ => panic!("didn't expect {}", bracket),
    }
}

fn input<'a>() -> Vec<&'a str> {
    vec![
"{[<({(<(<{((<(()())[()<>]>({[][]}{<>{}}))[[({}[]){()<>}]<{{}<>}(<><>)>]){[{{<>()}[{}<>]}({()[]}([]))]{{[(",
"<{<({([{{<[[{({})({}<>)}{<<><>><<>())}](<<[]()>(<>{})>[[()()]{{}{}}])]{{[[()<>](()[])]}[<(()",
"{(({<[[{({[<<({}<>)<{}[]>>>{(<()[]>[[][]])<<{}()>((){})>}]})}{{<{({({}<>){<>}}<({}())>)}<(({[]()}",
"[((([{({[{[<<((){})[[]()]><([]){()()}>><{[(){}]>{(<>{}){(){}}}>]}{<<{[<>()]}((<>[]){[]()})>([[<>{}]{(",
"<{[<[(<[[{{{({()()}[<>[]])<[<>{}][[]()]]}}([((()())(<>())){{()<>}[<>{}]}]{[[<>[]]{[]<>}]([{}()](()<>))})}((",
"<{(<(<[<<<{<([{}()]({}()))[({}())]>[((()())({}<>))(<[]()>[(){}])]}(({<(){}>}<[{}()][()()]>)",
"{{[(((<{[{(({{{}}[[][]]}[[{}{}]]))(((([]<>)[{}{}>)[({}{})<[][]>]))}{({({<>()}{()[]}){<<>()><<",
"{(<([{[(({(<({()[]})[{{}()}[()[]]]>[<{()<>}{()}>{[{}[]]}])[[<{{}{}}[{}]>[{[]<>}([]<>)]]{([[]][<>]){{()<",
"[<[([<{{(({(<{[]{}}([][]>>({[]<>}{(){}}))}))[{({[<<>{}>]([<>()]<<><>>)}[(([]<>){<>{}}){[()()]([][])}]",
"[[<([{{<[[<<<<[]<>>[[]<>]>[([]()}[{}[]]]>><(<[<>[]]{[]<>}><[()][<>{}]>)[[([]<>)<()[]>][[{}<>]({}{})",
"{<({<{{(((<{{[[][]]<<>[]>}}([{<>()}[[]{})])>(<<{[]()}[<>]>>)){{{{(<>{})([]{})}[<[][]><<><>>]}<<(<>{})<()()>>{",
"<[({([({{<[{{[()[]][()<>]}}<([[]]<()>)({()}<[]()>)>]><[{([()<>]{{}[]})<{[]{}}>}((<[]{}>[()()]){{{}<>}",
"([<{[<{[<{{<{[(){}]((){})>([{}()](<><>))>[{{(){}}}[(<>()){[]{}}]]}}>][{<(({[[][]]}{{[]<>}([][])",
"{[[{(<[(({{[<[()[]](<>[])>({{}<>}[[]()])](<{()()}{{}<>}>({{}[]}<[][]>))}})<<{{[<[]<>>{()<>}]}<(<[]()>{<>",
"(<[{[(<<[[{[({(){}}[[]<>])([<>()]({}[]))]}]]{[<{[<<><>>[[]()]][[[]{}][{}()]]}>{({<()<>>([]())}([<><>][()[]]",
"{{{[[[<{<[<[<{()<>}<()<>>>]{{({}<>)<()()>}}>(({{[]<>}}{{[]()}[{}()]}))]{{[([<>]{{}<>})<<<>",
"[[[{([<[[(((<<[]()>(()())>[{{}[]}])<{[(){}](()())}(<[]()><<><>>)>){{<<{}>({}())}{[[]<>]<[][]>}}<",
"[{{{{(<([<<[{[<>{}]}{<{}<>><<>()>}][<[<>()}{()[]}>(<[]<>>[<>{}])]>>[[[(<()()><{}<>>){<()<>>{[",
"<[[{[[{[(<<{{{{}}}({(){}}[[][]])}><([<[][]>{()()}]<[<>[]](<>[])>){(({}{})<[][]>)<[<>[]]>}>>)({{<[(<>[]){[]}]",
"[{<[<<([(<[[[(()<>){<>()}}]{({<><>}{[]()}){{{}<>}}}]{<<<<>{}>(<>{})>({(){}}((){}))>(<[<><>][<>]><([]<>){[]}>",
"{{({[<(<{<{{({<><>})<{{}[]}{[]{}}>}<({<>{}}[[]()])<(<>{})>>}>{[<(([]())[[]()])[{{}()}]>({({}[])[[]()]}{(",
"([<[[({(<<({{<{}[]>[<><>]}})[<[({}[]){{}{}}][{<>()}([])]>]>>{[[[<({}{})[<>()]>{<()>}](<{[]<>}(()<>)>",
"<[((({<(<{{[({[]{}}{[]{}})({{}<>})](([[][]}[[]()])({[]<>}{()<>}))}<{<[{}<>][{}()]>[(()[])([]())]}<[[<>{}]][(",
"{{[<{[(({{([<[()()]{<>()}><[<>[]][[]<>]>]{{({}())([]())}[[[]<>]{<><>}]})}}({((<{{}[]}[<>{}]>{(",
"({[[[<([(<{([[<>[]]][{(){}}({}{})])}<(<{<><>}(()<>)>{{{}<>}})(({()}[{}{}])[(()())[()()]])>><(<{{{}{}}({}<>",
"([[[<((<{({([{()[]}{()[]}](<()()>(())))<{[()[]][[]{}]}[{(){})([]())]>}(<({<>()}<[]()>)>{<<()<>>(()())>})",
"[({((([<((([[{<>{}}<[]<>>]<[[]<>][<>[]]>]<[(<>{})<{}<>>]<<<>[]>[<>()]>>))[[{[[{}[]]{()<>}]}]<[[(<>[]",
"{<([({<[(({<(([]()){<>()})<([]{})(<><>)>>[[[[][]]<[]{}>]{([]())[<>()]}]}(<{[<>]<<>()>}{<()[]>",
"{[[{[[<[[[{{{([][])[<>()]}}<(([]{})[[][]])[{[]{}}}>}(({<()<>>[(){}]}){[(<>())(<><>)][{()}[[]]]}",
"{<({({<{(([[({[]()}(()())){({}{}){<>{}}}]]([[{<>{}}[[]]]((<>[])[<><>])]<{({}[])<<><>>}{{<><>",
"{([[((<<[[(<[((){})]({<>{}}(<>{}))>)[({[{}[]]}){<([]<>){[][]}>}]][[[{<<>[]>[<>{}]}[{()()}<{}[]>]",
"([[<[[[({[<[[[{}[]]{<>()}]]{<{()<>}{{}[]}><[{}[]]{(){}}>}>]})]([({[([{()[]}])][{<(()<>)[[][",
"[<{<{[[[<{[{<({}[])>[{[]}{<>[]}]}<[{<>[]}<()[]>]>]<([{<>{}}[{}()]]{(<>{})[<>[]]})[<<<><>>[{}]><<()<>>>]>}>",
"<[{<{[{{<<{<[[(){}]([]())][{[][]}(<>{})]>([([][])<[]()>][<{}{}>([]<>)])}[[[(<>())<()[]>]]]><({(({}())[[]",
"([<<{{{<([<<{[()[]]<(){}>}[[()()](<>[])]>([<[][]><{}{}>])>[[{{[]<>}[<><>]}]<([<><>][()()]>(({}[])[[]<>])>]]{",
"[(({{[{{[<<[<((){})([]<>)>({{}()}([][]))]([[{}<>]{[]()}]<<[]())<(){}>>)><{<{()[]}({}())>{{()[]}({})}}<(<{}()>",
"<{(<<(<[(<<{[{(){}>({}())]{[<><>][[]()]}}[<[(){}]{<>{}}>]>[<[(()[])[<>[]]][{()[]}{<><>}]>[([[]]){<()<>><(",
"[{{[(<((({((({<><>}([]))(<[]><<><>>)))}{({[<{}{}>({}<>)]({()})}(<({})[<>[]]>((()[]){{}})))})){{((<[{(",
"<[([(<[([[([<{{}{}}[[]<>]>[[()[]]({}())]][{{{}{}><<>()>}])<[{[[]<>]<[][]>}<{[]()}((){})>]{[{{}<>}](<[]<>>{()[",
"<[{[({<([[{{{([][])<[]{}>}((<>{}))}({([])<[]()>})]([[<()[]>(()<>)][<()[]>]]{<<<>>[<>{}]>[<<><",
"[[[[[{{[[{[[<<()()>{[]{}}>{{[]{}}}}]}<(({[{}<>][()()]})(<{[]<>}((){})>[{<>()}]))>]{<{((([]()){{}<>}))(",
"([[[[[{[(<[[<<()[]>{()}>[[<>[]][()()]]]([{{}{}}[<><>]][[<>()]<{}{}]])]><<[[<[][]>[{}[]]]([<>[]]({}<>))][[[(",
"<[{<[(<<<[{({<[]()><[]{}>})<<{()<>}(<><>)><<<>())[<>{}]>>}<<{<[]<>><<><>>}[({}[])<[][]>]>{[[{}{}]({}<>)]<(",
"{[<[{{({<{[[((<><>){<><>})<<[]>{[]<>}>]{((()()))(<()[]><[]<>])}](<(<{}<>>[<>{}]){[[]{}][<>[]]}>{",
"<[[<<<[(<<<{{[[]<>][{}<>]}}[[{[][]}[<>{}]]<([]){()[]}>]>[{<[()[]]{{}[]}><<{}<>>[()()]>}(<<<>{}>>)]>{[{",
"{<{{{{{[<<[(<([]{}){()<>}>)]>[{{<{<>()}>{{{}{}}}}}]>]}<[[<{({{[][]}[[]{}]}<({}<>)<[]()>>)[<([]()){<><>}><",
"{<(<(<<[<[(<[{()()}<<><>>][(<>[]){()[]}]><{(<><>){[][]}}>)<(<<(){}><()<>)>{([]{})})[<[{}{}]{[][]}>{([]",
"([[<[[((([(<<<<>{}>[<>[]]>>[((<>))<({}()}<()>>])[{({()}[<><>])({<>{}}(<><>))}[{{()[]}}[(())]]]](({{{(){}}<[][",
"<<[({(({([[<[{<>{}}{<>[]}]<([]{})>>]<(<{()[]}{<>}>(<[][]>{[]{}}))<{[[]()](()())}{<[]<>>(()<>)}>>])}){(<{{<{[",
"((<(({[([{{({([]<>){[]{}}}<((){})([][])>)[({[]{}}[(){}]){<<>()>[{}{}]}])}{<([<{}[]>[[]()]])<{[{}(",
"(([[<[(([{{{{{<><>}(()())}{(<>[])(()<>)}}<{<()<>>(<>[])}{[[]<>]{[]{}}}>}({[[{}<>>[(){}]]}(<<",
"((<[{([{[{<(<{(){}}<()()>>[{<>{}}<{}[]>])<{<[]()><()<>>}[({}())]>}}][[<[[{<>}][((){})<<>>]]<[",
"(<({([([(<{<{({}[])[{}[]]}<{<>()}{<>()}>>}>{[(<(<>{})(<><>)>{<<><>>{()<>}})<[{[][]}<[]()>}>]",
"{({((<{<(<<{{<()[]>[()()]}<{()[]}{()<>}>}<([[]()](<>[])>[{<>()}]>>({{({}())[[]{}]}{<<>><[]<",
"{{<{({[{(<([{[[]<>][{}[]]}{<[]{}>(())}])>(<<<[{}<>]<<>[]>>>[<<<><>>{[]()}>]><{{[()]}}<<<{}<>>[()",
"[<[[<({((({({([]())<()()>}<{{}()}(<>{})>)[({{}{}}{{}{}}){[[]<>][[][]]}]}({([(){}][<>{}]){[<>[]][[",
"[[<<[[[{({<[({<>[]}{<>[]})]({((){})}({{}[]}))>}[[(({[]<>}<<>{}>)({()[]}<()<>>))]<<([[]<>])[[[]{}]({}{})]",
"[<[[{(<{<<{{(<()[]><(){}>)<[()[]]([][])>}}({<{{}}[[]()]>[{<>{}}]})>[[<{<[]()>{[]{}}}<<{}[]>[{}[]",
"(<<{<<({{{[[[([]()){[]<>}]](({[][]})<((){})[<>{}]>)][(<({}())>[({}{})<<><>>])]}(<{([<>()](()()))}{{[<>]{()",
"[{([<<[{{[({([()()]<[]<>>)<<()<>><[][]>>}{<[()[]]<[][]>>([()[]]{(){}}>})[(({[]()}[[][]])[({}{})[{}{}]])[({[]",
"<({(([[<({<<[<{}()>{[]()}]><<<<>[]>{{}[]}><(()())[[]{}]>>>})>{<{[({<[][]>(()<>)}[[{}{}]([]<>)])<{(())}{{[",
"({(<[[<([{{[<{{}<>}{<>{}}><([]{})>]<{[[]{}]}>}(<({{}[]}{()[]})((()<>)({}()))>({{<>{}}<<>{}>}([{}]<",
"[<[[[{({[{<{{[{}[]}[<>{}]}[[()<>]([]<>)]}>}[{[[{(){}}<()<>>][{()[]}]]<(([]{})<(){}>)([()<>][<>{}])>}<([[",
"<{{[{(<<(<[<({()[]}((){}))([()]<[]<>>)>([<{}()>({}())]([[]{}]<[]{}>))]({([()]<(){}>)[(<>{}){<>()}]}<([[]<>]<{",
"({{[([[{((<(<[()][()[]]><[{}<>]{{}()}>)<{<[]()>([][])}([<>()]<{}<>>)>>([[<[]()>([]<>)][{{}()}{<>[]}]][({()()",
"[({<({([[{[[<[(){}][[]{}]>]<[((){})(<>)]>]{{{{()()}[[]()]}{<{}>[<>{}]}}<[<<>[]><<>()>][<()[]>[<>{}]]>}}<[(",
"[{([<[((([{[({[]()}{()()})({<>()}<[][]>)]<<{[]{}}>>}([{({}<>)}[{[][]}<<>()>]])]{<[(((){}){{}{}})]({(()[",
"(<({{(({(<({(({}())[(){}])<({}[])[{}{}>>}((([]())<[][]>){(<><>){{}()}}))[{({()<>}{[][]})(<()<>>)}]>)}))",
"(([(({{(<{[[[{[]<>}<<>()>]<{<><>}([]())>]([([]<>)<()<>>])]}<[(<<<><>>}{{<>{}}[(){}]})[(<[]<>>{<>}){{[]()}",
"{{{[<<([(<(<[{{}()}[(){}]][{[]()}{<>[]}]>([[<>[]]{[][]}](<<><>>{[]{}})))<[[[()()]<<>{}>](<(",
"({([[[[(<[{([[{}{}](()())])}]<<<[{<><>}<[]{}>]<({}<>)>>[[{()}[()[]]]{([]<>){()<>}}]>>><<<[{[[",
"<{[(<{({<[({{{<>{}}<[]{}>}<((){})>}({[{}[]]<<>{}>}(<[]>{{}[]}))){<[(()<>)[[]{}]]>}][[<[{[]<>",
"[{[[[<<{<<[[{<<><>>(()<>)}<[(){}][[][]])]]>>}{<{((<[{}<>](<>{})>))}>(<[[[<<><>>{<>[]}][{()()}[[][]",
"<(<{{<<<{([({<{}()>({}{})}([{}[]]<{}[]>))<{[()<>]}[<()()>(()<>)]>]<(<{[]{}}{<>{}}><[[]<>](()[])>)<{<{}{}",
"[((([<<{<{[<{(()<>)<()()>}[[{}[]][{}]]>(<[[]()]>{<()<>>(<>())})]}<(<[<()[]>]>([[[][]}({})]<[<>{}]{[][]}>",
"<<[{[(<({<{<<{{}<>}[{}{}]>>([{[]()}{<>[]}]<<{}{}>{()()}>)}<<<<<>[]><[][]>>[{[]{}}(<><>)]>[(<[]{}>[[]()])(",
"[([<<{<<{<{{{(()()}{{}[]}}([()()](()[]))}{[([]{})[[][]]](([][]){()()})}}>}<[<[({(){}}{{}()})(",
"[<{(<({({{(({[<>{}]<[]{}>}({()<>}<[][]>))[(<(){}>)[{{}<>}[{}()]]])}<<<[{[]{}}[<>[]]][<<>><<><>>]><([{",
"{<<(<<{<{(({<{(){}}>((<><>)<()()>)})<[[[()<>]<{}<>>]]{({{}[]}<[]<>>)[{()()}({}())]}>)<<<[({}[])<<>[]>]{<",
"({[[({{{([<{({[]{}}<[][]>)<{[]{}}{{}()}>}{{{{}<>}}<<<>()>[[]()]>}><[<([]{})[<>{}]>]<<{<><>}<()<>>>[{[]}<",
"<({([[{[(([[(([][]){[]()}){(<>())}][([[][]])(<()>}]](([<()()><{}[]>]{<[][]><<>[]>})<{<[]()>{[]{}}}(<()<>",
"[{({(<<({[[(([{}<>]{()[]})[[[]<>]{(){}}])][<[{{}()}(()[])]{{(){}}[()()]}>{[[[]()]<[]<>>](<<>{}>{",
"{[<([[[{[([{([{}()]<()()>)}<<[[]]([]())><<<>[]>>>][{[{()[]}]({{}{}}{[]<>}]}])]{<({{{[][]}{<><>}}<<<>[]",
"(<{((({<[<<({({}[])([]{})}<{{}()}<<><>>>)<{(()[])(<>{})}>>{[[([]())(<>[])][<()>{<><>}]]}><[[([{}[]])(<[]()>((",
"{(([{[<[{<({[[<>[]][<><>]]<([][])[(){}]>}{{{[][]}<()[]>}}}<(([[]<>])[(<><>)<<>[]>])>>{[<[({})[[]<>]]{[{}[]",
"[(([(<([((([[{[][]}((){})]{[[]{}]{<>()}}]({[[][]](()<>)}{[{}<>]})]){[([[[]{}](<>[])]{((){})<{}<>>}",
"{{{({<[(<(<([(()[]){<>{}}])<({{}<>})>>[(<(<>{})<{}[]>>{([]<>)[[]{}]})<[(()<>)]({[]()})>])[[[({()[]}){({}<>",
"[[([[[(<<(<<(<<><>>({}()))<<[]{}>[()<>]>><{[{}[]]}[(<>)([][])]>>)([<{<()>(<>{})}>[(<()[]>[<>[]])(",
"<<(<[[({{<<({<<><>><[][]>}<<<>{}>(<>[])>)[{{[][]}{[]{}}}(<<>()>[()()])]>{(<[{}<>][{}()]](<(",
"({<{([[{[{[<{<[]{}>[{}()]}[<[]<>><<>[]>]>[<{[]()}[[]{}]>[({}[])<()()>]]]<(({{}<>}[{}()))[(<>{})(<><>)])<{",
"{({[<<([<<[[[{()()}{<>[]}]({<><>}<(){}>)]]>[{{<[<>{}](<>[])>}(((<>()){()[]})[<()<>>[{}{}]])}<(<",
"<[{[{<(<({{((<[][]><<>{}>)([<><>]<<>[]>})<[{[]()}{{}{}}](<[]()>[(){}])>}{{[<<>()>{[][]}]{{()[]}",
"(<[((<{([({{<{()}{{}{}}><{[]()}[<><>]>}[<[(){}]{{}()}>[<{}[]>[{}[]]]]})])((([([{<>{}}<[]{}>]<(<>[]}>)",
"[[((<({(({(<<[<>][(){}]>[{<>{}}[<><>]]>([{<>()}(<>())][[{}[]]])){[{[{}{}]{<>()}}[(<>())]]}}[<{[({}())<",
"((<[[<({<<{[<{{}[]}<()<>>>[(<>{}){{}[]}]](((<>)({})))}{<[[{}<>][{}{}]]{(()())([][]))>{[[{}{}]]{{{}[]}[[]()]}",
"{{[(<({{{[[<<{()<>}({}())>({{}<>}<{}{}>)>([[[][]]{{}()}]{[<><>]})}(((([]<>)({}()))(<[][]>))<(<<>[]>({}{})",
"[[{<[(<[[([[[{()()}{<><>}]<<{}[]][<>[]]>]{{[<>{}]}[[{}[]]]}](<<<[]()>[[]{}]><{{}()}<{}()>>>[[{(",
"[<{{<<<<{({<{<()<>>((){})}({{}<>}(()[]])>}{[({[]()}[()()])]})[{{<{()()}<{}[]>>}<[[(){}]{<><>}]{[[]<>]{{}<>",
    ]
}