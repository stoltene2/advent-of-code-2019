use std::collections::HashSet;

type Point = (usize, usize);

fn main() {
    let mut hs: HashSet<Point> = input().into_iter().collect();
    let line = (655, 0);

    hs = fold(hs, &line);
    assert_eq!(638, hs.len());

    println!("{:?}", &hs.len());

    // part 2
    let mut hs: HashSet<Point> = input().into_iter().collect();

    for line in input_folds() {
        hs = fold(hs, &line);
    }

    for y in 0..=50 {
        for x in 0..=50 {
            if hs.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    // Pt 2 solution
    //  ##    ##  ##  #  # ###   ##  ###  ###
    // #  #    # #  # # #  #  # #  # #  # #  #
    // #       # #    ##   ###  #  # #  # ###
    // #       # #    # #  #  # #### ###  #  #
    // #  # #  # #  # # #  #  # #  # #    #  #
    //  ##   ##   ##  #  # ###  #  # #    ###
}

fn reflect((x1, y1): &Point, line: &Point) -> Point {
    match line {
        // fold on vertical line
        (x, 0) => ((2 * x - x1), *y1),
        // Fold on horizontal line
        (0, y) => (*x1, 2 * y - y1),
        _ => panic!("line, {:?} is not a line", line),
    }
}

fn fold(points: HashSet<Point>, line: &Point) -> HashSet<Point> {
    // if point is right or below line
    // Remove point from points
    // Add reflected point

    let mut ps = points.clone();
    for p in points {
        match line {
            (0, y) => {
                if p.1 > *y {
                    ps.remove(&p);
                    let reflected = reflect(&p, &line);
                    ps.insert(reflected);
                }
            }
            (x, 0) => {
                if p.0 > *x {
                    ps.remove(&p);
                    let reflected = reflect(&p, &line);
                    ps.insert(reflected);
                }
            }
            _ => panic!("at the disco"),
        }
    }

    ps
}

fn test_input() -> Vec<Point> {
    vec![
        (6, 10),
        (0, 14),
        (9, 10),
        (0, 3),
        (10, 4),
        (4, 11),
        (6, 0),
        (6, 12),
        (4, 1),
        (0, 13),
        (10, 12),
        (3, 4),
        (3, 0),
        (8, 4),
        (1, 10),
        (2, 14),
        (8, 10),
        (9, 0),
    ]
}

fn input_folds() -> Vec<Point> {
    vec![
        (655, 0),
        (0, 447),
        (327, 0),
        (0, 223),
        (163, 0),
        (0, 111),
        (81, 0),
        (0, 55),
        (40, 0),
        (0, 27),
        (0, 13),
        (0, 6),
    ]
}

fn input() -> Vec<Point> {
    vec![
        (103, 224),
        (624, 491),
        (808, 688),
        (1076, 130),
        (700, 26),
        (55, 794),
        (119, 724),
        (773, 809),
        (875, 33),
        (922, 135),
        (509, 260),
        (801, 176),
        (1143, 85),
        (619, 526),
        (1250, 138),
        (753, 431),
        (1260, 654),
        (276, 457),
        (637, 718),
        (1183, 115),
        (284, 137),
        (539, 757),
        (279, 85),
        (1128, 474),
        (406, 469),
        (1086, 170),
        (927, 673),
        (310, 702),
        (35, 796),
        (268, 892),
        (202, 249),
        (820, 878),
        (992, 121),
        (339, 649),
        (1275, 796),
        (1113, 290),
        (336, 616),
        (522, 471),
        (755, 764),
        (631, 204),
        (241, 393),
        (455, 546),
        (1165, 176),
        (304, 276),
        (855, 859),
        (582, 380),
        (437, 9),
        (944, 400),
        (199, 770),
        (124, 473),
        (522, 423),
        (167, 85),
        (606, 493),
        (1158, 688),
        (1186, 473),
        (981, 54),
        (790, 507),
        (704, 493),
        (788, 362),
        (971, 201),
        (512, 889),
        (758, 810),
        (1101, 770),
        (325, 840),
        (734, 873),
        (725, 586),
        (416, 105),
        (1183, 365),
        (1233, 725),
        (698, 256),
        (418, 112),
        (520, 507),
        (276, 773),
        (634, 185),
        (483, 693),
        (612, 256),
        (651, 603),
        (1225, 53),
        (1268, 759),
        (35, 339),
        (311, 474),
        (984, 592),
        (1237, 770),
        (1093, 129),
        (755, 639),
        (671, 278),
        (788, 471),
        (430, 810),
        (693, 231),
        (281, 227),
        (454, 178),
        (1, 54),
        (1290, 499),
        (619, 407),
        (1081, 575),
        (1252, 266),
        (1186, 421),
        (698, 638),
        (1218, 561),
        (954, 763),
        (692, 402),
        (923, 532),
        (281, 631),
        (222, 777),
        (1047, 316),
        (1109, 215),
        (513, 824),
        (771, 645),
        (802, 640),
        (769, 471),
        (42, 256),
        (619, 80),
        (913, 425),
        (340, 68),
        (397, 833),
        (539, 137),
        (1136, 654),
        (515, 18),
        (507, 247),
        (1274, 402),
        (970, 364),
        (913, 362),
        (328, 410),
        (775, 404),
        (164, 16),
        (356, 763),
        (545, 353),
        (790, 506),
        (780, 46),
        (378, 569),
        (276, 437),
        (878, 777),
        (159, 613),
        (313, 137),
        (377, 418),
        (134, 22),
        (483, 14),
        (1207, 670),
        (1238, 738),
        (528, 268),
        (913, 761),
        (1009, 501),
        (1217, 805),
        (865, 361),
        (460, 801),
        (358, 715),
        (857, 775),
        (552, 842),
        (771, 197),
        (627, 40),
        (765, 577),
        (1071, 704),
        (862, 638),
        (811, 686),
        (219, 413),
        (904, 425),
        (728, 738),
        (1020, 588),
        (1240, 225),
        (619, 78),
        (611, 180),
        (610, 474),
        (388, 135),
        (957, 582),
        (194, 43),
        (1027, 842),
        (917, 864),
        (537, 770),
        (831, 26),
        (1240, 669),
        (1034, 582),
        (1210, 49),
        (1036, 282),
        (1217, 537),
        (271, 100),
        (537, 691),
        (584, 290),
        (224, 618),
        (1111, 546),
        (610, 26),
        (726, 221),
        (1053, 509),
        (1295, 278),
        (1094, 859),
        (725, 245),
        (552, 52),
        (981, 168),
        (448, 190),
        (905, 64),
        (700, 126),
        (132, 179),
        (930, 648),
        (692, 189),
        (659, 291),
        (1027, 852),
        (984, 78),
        (318, 121),
        (1026, 757),
        (889, 385),
        (184, 596),
        (746, 50),
        (850, 93),
        (233, 173),
        (234, 130),
        (201, 679),
        (290, 588),
        (510, 0),
        (284, 869),
        (939, 628),
        (147, 303),
        (1265, 78),
        (445, 361),
        (763, 477),
        (1250, 586),
        (1178, 715),
        (282, 196),
        (1290, 617),
        (719, 191),
        (460, 93),
        (932, 325),
        (529, 343),
        (112, 829),
        (967, 567),
        (43, 714),
        (545, 541),
        (541, 423),
        (612, 759),
        (1262, 196),
        (473, 522),
        (878, 117),
        (512, 596),
        (182, 446),
        (783, 529),
        (797, 70),
        (768, 65),
        (477, 270),
        (552, 810),
        (373, 227),
        (502, 682),
        (378, 121),
        (430, 724),
        (592, 565),
        (180, 254),
        (145, 718),
        (184, 298),
        (748, 354),
        (127, 147),
        (845, 400),
        (951, 485),
        (393, 260),
        (898, 357),
        (480, 667),
        (541, 871),
        (507, 199),
        (1267, 714),
        (407, 406),
        (288, 494),
        (152, 654),
        (827, 14),
        (1028, 698),
        (1000, 192),
        (549, 169),
        (490, 211),
        (1111, 592),
        (535, 404),
        (26, 854),
        (372, 126),
        (455, 859),
        (236, 340),
        (716, 808),
        (387, 532),
        (1101, 361),
        (857, 119),
        (999, 420),
        (276, 878),
        (373, 417),
        (923, 670),
        (92, 729),
        (179, 703),
        (619, 555),
        (455, 348),
        (668, 266),
        (674, 217),
        (1139, 880),
        (442, 530),
        (1042, 2),
        (594, 586),
        (890, 354),
        (13, 757),
        (698, 759),
        (268, 677),
        (1125, 276),
        (314, 826),
        (1210, 889),
        (207, 42),
        (691, 78),
        (241, 277),
        (42, 638),
        (176, 82),
        (445, 891),
        (315, 649),
        (1081, 95),
        (880, 810),
        (782, 268),
        (472, 229),
        (2, 826),
        (769, 23),
        (974, 616),
        (898, 537),
        (102, 674),
        (604, 374),
        (415, 480),
        (1154, 453),
        (1110, 14),
        (798, 695),
        (562, 333),
        (336, 809),
        (254, 508),
        (412, 313),
        (271, 221),
        (763, 171),
        (883, 255),
        (992, 773),
        (999, 327),
        (818, 36),
        (1081, 215),
        (974, 278),
        (1274, 588),
        (808, 234),
        (373, 477),
        (43, 42),
        (552, 674),
        (609, 816),
        (440, 486),
        (1081, 679),
        (105, 723),
        (281, 667),
        (865, 96),
        (574, 442),
        (290, 140),
        (1289, 294),
        (338, 682),
        (299, 264),
        (1268, 638),
        (1154, 5),
        (42, 759),
        (999, 474),
        (502, 688),
        (35, 787),
        (452, 400),
        (1116, 679),
        (571, 684),
        (420, 781),
        (1056, 396),
        (1309, 467),
        (1238, 193),
        (1287, 204),
        (448, 638),
        (418, 844),
        (832, 82),
        (1053, 385),
        (1290, 395),
        (1101, 768),
        (1027, 42),
        (1146, 858),
        (768, 787),
        (716, 674),
        (765, 541),
        (504, 82),
        (512, 471),
        (1206, 565),
        (691, 80),
        (391, 764),
        (1208, 514),
        (1, 427),
        (23, 690),
        (994, 212),
        (837, 522),
        (1265, 750),
        (209, 320),
        (952, 715),
        (999, 687),
        (329, 726),
        (1233, 85),
        (126, 626),
        (117, 462),
        (803, 143),
        (652, 712),
        (142, 298),
        (582, 50),
        (370, 309),
        (20, 838),
        (903, 488),
        (380, 515),
        (681, 536),
        (1128, 446),
        (291, 75),
        (502, 206),
        (70, 673),
        (233, 721),
        (127, 81),
        (99, 255),
        (765, 765),
        (102, 871),
        (388, 51),
        (612, 135),
        (763, 723),
        (387, 425),
        (639, 278),
        (1191, 180),
        (728, 514),
        (607, 869),
        (199, 302),
        (699, 282),
        (420, 354),
        (823, 64),
        (527, 747),
        (405, 64),
        (564, 50),
        (35, 526),
        (524, 334),
        (913, 61),
        (542, 667),
        (582, 722),
        (492, 36),
        (765, 353),
        (1074, 340),
        (840, 450),
        (875, 400),
        (937, 477),
        (2, 264),
        (1146, 260),
        (1027, 714),
        (1101, 320),
        (182, 474),
        (801, 767),
        (1260, 381),
        (72, 253),
        (195, 73),
        (390, 497),
        (1056, 508),
        (107, 275),
        (753, 504),
        (699, 180),
        (890, 540),
        (894, 105),
        (509, 270),
        (530, 249),
        (378, 94),
        (50, 65),
        (870, 486),
        (1029, 667),
        (728, 840),
        (974, 809),
        (803, 751),
        (92, 354),
        (749, 509),
        (1198, 206),
        (144, 0),
        (290, 208),
        (1195, 249),
        (470, 647),
        (1233, 809),
        (801, 270),
        (874, 793),
        (903, 406),
        (310, 192),
        (1109, 551),
        (806, 812),
        (1069, 841),
        (1161, 158),
        (1044, 311),
        (1168, 23),
        (542, 227),
        (1268, 361),
        (1166, 446),
        (421, 385),
        (782, 432),
        (478, 82),
        (1034, 885),
        (1154, 217),
        (216, 859),
        (677, 497),
        (372, 660),
        (806, 82),
        (612, 704),
        (483, 201),
        (820, 260),
        (933, 418),
        (873, 885),
        (744, 789),
        (147, 143),
        (594, 220),
        (45, 144),
        (1268, 135),
        (699, 119),
        (383, 221),
        (27, 854),
        (328, 352),
        (499, 880),
        (1298, 75),
        (437, 317),
        (102, 52),
        (1184, 432),
        (996, 826),
        (448, 25),
        (1211, 764),
        (987, 165),
        (913, 133),
        (35, 595),
        (271, 436),
        (1036, 155),
        (1255, 624),
        (1287, 690),
        (1298, 714),
        (492, 688),
        (827, 693),
        (1129, 521),
        (1028, 196),
        (1176, 388),
        (654, 190),
        (967, 106),
        (340, 364),
        (201, 215),
        (59, 670),
        (495, 606),
        (768, 227),
        (576, 873),
        (746, 844),
        (618, 82),
        (1238, 253),
        (387, 84),
        (50, 325),
        (631, 690),
        (705, 418),
        (937, 711),
        (1207, 152),
        (152, 682),
        (768, 829),
        (528, 686),
        (716, 220),
        (1183, 141),
        (277, 866),
        (35, 487),
        (44, 749),
        (798, 889),
        (705, 476),
        (326, 816),
        (1251, 224),
        (1310, 381),
        (1109, 343),
        (1211, 255),
        (60, 138),
        (1081, 103),
        (659, 603),
        (214, 65),
        (835, 703),
        (1290, 838),
        (534, 645),
        (539, 249),
        (1074, 50),
        (406, 425),
        (209, 768),
        (652, 182),
        (512, 838),
        (316, 212),
        (820, 16),
        (35, 147),
        (985, 691),
        (604, 65),
        (407, 488),
        (811, 880),
        (1193, 432),
        (937, 675),
        (344, 416),
        (400, 325),
        (1145, 168),
        (868, 530),
        (769, 871),
        (1298, 404),
        (134, 388),
        (1039, 436),
        (1033, 812),
        (681, 393),
        (517, 337),
        (582, 844),
        (725, 201),
        (1012, 187),
        (480, 675),
        (654, 704),
        (1275, 144),
        (115, 249),
        (12, 588),
        (1028, 644),
        (306, 220),
        (326, 592),
        (671, 431),
        (97, 431),
        (1146, 16),
        (380, 648),
        (955, 686),
        (604, 325),
        (119, 842),
        (1086, 618),
        (1309, 54),
        (562, 540),
        (1146, 464),
        (996, 523),
        (311, 327),
        (758, 471),
        (92, 281),
        (671, 655),
        (550, 565),
        (125, 103),
        (699, 404),
        (228, 752),
        (1298, 371),
        (576, 649),
        (552, 380),
        (1260, 513),
        (821, 427),
        (783, 141),
        (0, 78),
        (683, 40),
        (806, 826),
        (552, 276),
        (371, 567),
        (431, 0),
        (267, 532),
        (1086, 808),
        (336, 278),
        (372, 234),
        (530, 46),
        (58, 628),
        (12, 371),
        (855, 546),
        (282, 644),
        (938, 660),
        (1176, 22),
        (995, 649),
        (49, 476),
        (1238, 514),
        (1279, 485),
        (539, 697),
        (50, 513),
        (1190, 256),
        (490, 235),
        (430, 84),
        (18, 217),
        (453, 215),
        (1103, 42),
        (274, 164),
        (242, 537),
        (884, 752),
        (550, 329),
        (855, 299),
        (100, 845),
        (328, 526),
        (910, 514),
        (933, 476),
        (100, 49),
        (2, 378),
        (301, 501),
        (1298, 180),
        (939, 842),
        (470, 298),
        (35, 528),
        (746, 498),
        (156, 453),
        (857, 439),
        (564, 554),
        (144, 777),
        (1165, 718),
        (840, 695),
        (546, 793),
        (749, 385),
        (576, 201),
        (529, 103),
        (835, 751),
        (1266, 26),
        (1093, 577),
        (207, 852),
        (1297, 757),
        (933, 866),
        (841, 724),
        (1201, 129),
        (199, 859),
        (1074, 560),
        (335, 649),
        (271, 794),
        (1125, 724),
        (838, 229),
        (1183, 779),
        (782, 686),
        (413, 393),
        (765, 129),
        (77, 725),
        (1146, 430),
        (59, 224),
        (870, 107),
        (656, 704),
        (164, 270),
        (865, 320),
        (181, 521),
        (1063, 40),
        (229, 95),
        (142, 392),
        (987, 515),
        (838, 644),
        (276, 9),
        (103, 670),
        (790, 58),
        (174, 240),
        (1298, 819),
        (1131, 703),
        (435, 861),
        (1308, 712),
        (782, 626),
        (806, 147),
        (523, 261),
        (979, 379),
        (1026, 137),
        (591, 191),
        (726, 540),
        (92, 561),
        (400, 392),
        (1309, 427),
        (728, 380),
        (937, 417),
        (171, 880),
        (1308, 182),
        (1218, 540),
        (311, 119),
        (104, 565),
        (201, 343),
        (266, 583),
        (363, 371),
        (397, 89),
        (119, 714),
        (919, 130),
        (142, 502),
        (773, 691),
        (995, 245),
        (1265, 816),
        (748, 540),
        (1185, 103),
        (1072, 789),
        (185, 170),
        (1020, 754),
        (946, 626),
        (677, 600),
        (93, 805),
        (1081, 791),
        (633, 294),
        (512, 840),
        (728, 50),
        (771, 137),
        (520, 506),
        (897, 393),
        (1154, 565),
        (541, 471),
        (1297, 137),
        (967, 474),
        (499, 208),
        (639, 879),
        (318, 269),
        (629, 281),
        (975, 649),
        (490, 260),
        (545, 577),
        (156, 5),
        (383, 669),
        (913, 532),
        (119, 164),
        (162, 688),
        (1092, 793),
    ]
}
