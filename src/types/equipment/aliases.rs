pub const fn canonical_item_id(id: i32) -> i32 {
    match id {
        // Abyssal tentacle
        26484 => 12006,
        // Abyssal whip
        26482 | 12774 | 12773 => 4151,
        // Adamant defender#Normal
        24141 => 8849,
        // Adamant full helm
        2613 | 2605 | 10296 | 10298 | 10300 | 10302 | 10304 => 1161,
        // Adamant kiteshield
        22127 | 22129 | 22131 | 22133 | 22135 | 22137 | 22139 | 22141 | 22143 | 22145 | 22147
        | 22149 | 22151 | 22153 | 22155 | 22157 | 2611 | 2603 | 7334 | 7340 | 7346 | 7352
        | 7358 => 1199,
        // Adamant platebody
        2607 | 23392 | 23395 | 23398 | 23401 | 23404 | 2599 => 1123,
        // Adamant platelegs
        2609 | 2601 => 1073,
        // Adamant plateskirt
        3475 | 3474 => 1091,
        // Agility cape#Trimmed
        13341 => 9772,
        // Agility cape#Untrimmed
        13340 => 9771,
        // Ahrim's hood#Undamaged
        4860 | 4856 | 4859 | 4858 | 4857 | 30543 | 30519 | 30537 | 30531 | 30525 | 30445 => 4708,
        // Ahrim's robeskirt#Undamaged
        4878 | 4874 | 4877 | 4876 | 4875 | 30547 | 30523 | 30541 | 30535 | 30529 | 30449 => 4714,
        // Ahrim's robetop#Undamaged
        4872 | 4868 | 4871 | 4870 | 4869 | 30545 | 30521 | 30539 | 30533 | 30527 | 30447 => 4712,
        // Ahrim's staff#Undamaged
        4866 | 4862 | 4865 | 4864 | 4863 | 30574 | 30570 | 30573 | 30572 | 30571 | 30568 => 4710,
        // Amulet of defence
        23309 => 1729,
        // Amulet of fury
        12436 => 6585,
        // Amulet of glory#Uncharged
        19707 | 1706 | 1708 | 1710 | 1712 | 11976 | 11978 | 10360 | 10358 | 10356 | 10354
        | 11966 | 11964 | 10362 => 1704,
        // Amulet of magic
        10366 => 1727,
        // Amulet of power
        23354 => 1731,
        // Amulet of rancour
        29804 => 29801,
        // Amulet of torture
        20366 => 19553,
        // Ancestral hat
        24664 => 21018,
        // Ancestral robe bottom
        24668 => 21024,
        // Ancestral robe top
        24666 => 21021,
        // Ancient halo#Normal
        24203 => 24201,
        // Ancient sceptre#Normal
        27626 => 27624,
        // Archers ring (i)#Nightmare Zone
        26768 | 25260 => 11771,
        // Armadyl chainskirt
        26716 => 11830,
        // Armadyl godsword
        29605 | 20368 => 11802,
        // Armadyl halo#Normal
        24194 => 24192,
        // Armadyl helmet
        26714 => 11826,
        // Assembler max cape#Normal
        24135 => 21898,
        // Ava's assembler#Normal
        24222 | 27359 | 27376 | 27374 => 22109,
        // Avernic defender#Normal
        24186 => 22322,
        // Bandos boots
        26720 => 11836,
        // Bandos godsword
        20370 => 11804,
        // Bandos halo#Normal
        24197 => 24195,
        // Bandos tassets
        26719 => 11834,
        // Barronite mace#Normal
        25643 => 25641,
        // Berserker necklace
        23240 => 11128,
        // Berserker ring (i)#Nightmare Zone
        26770 | 25264 => 11773,
        // Black d'hide body
        12381 | 12385 => 2503,
        // Black d'hide chaps
        12383 | 12387 => 2497,
        // Black defender#Normal
        24139 => 8847,
        // Black full helm
        2595 | 2587 | 10306 | 10308 | 10310 | 10312 | 10314 => 1165,
        // Black kiteshield
        2597 | 2589 | 7332 | 7338 | 7344 | 7350 | 7356 => 1195,
        // Black mask#(10)
        8919 | 8917 | 8915 | 8913 | 8911 | 8909 | 8907 | 8905 | 8903 | 8921 => 8901,
        // Black mask (i)#(10)
        11783 | 25275 | 26780 | 25266 | 26771 | 11782 | 25274 | 26779 | 11781 | 25273 | 26778
        | 11780 | 25272 | 26777 | 11779 | 25271 | 26776 | 11778 | 25270 | 26775 | 11777 | 25269
        | 26774 | 11776 | 25268 | 26773 | 11775 | 25267 | 26772 | 11784 | 25276 | 26781 => 11774,
        // Black platebody
        2591 | 23366 | 23369 | 23372 | 23375 | 23378 | 2583 => 1125,
        // Black platelegs
        2593 | 2585 => 1077,
        // Black plateskirt
        3473 | 3472 => 1089,
        // Black skirt
        12445 | 12447 => 1015,
        // Blade of Saeldor (c)
        25882 | 25876 | 25878 | 25872 | 25870 | 25880 | 25874 => 24551,
        // Blessed Dizana's quiver#Normal
        28957 => 28955,
        // Blood ancient sceptre#Normal
        28473 => 28260,
        // Blood moon chestplate#New
        29043 => 29022,
        // Blood moon helm#New
        29047 => 29028,
        // Blood moon tassets#New
        29045 => 29025,
        // Blue d'hide body
        7374 | 7376 => 2499,
        // Blue d'hide chaps
        7382 | 7384 => 2493,
        // Blue moon chestplate#New
        29037 => 29013,
        // Blue moon helm#New
        29041 => 29019,
        // Blue moon tassets#New
        29039 => 29016,
        // Blue skirt
        7386 | 7388 => 1011,
        // Blue wizard hat
        7394 | 7396 => 579,
        // Blue wizard robe
        7390 | 7392 => 577,
        // Book of Balance
        26488 => 3844,
        // Book of Darkness
        26490 => 12612,
        // Book of Law
        26492 => 12610,
        // Book of War
        26494 => 12608,
        // Bow of Faerdhinen (c)
        25896 | 25890 | 25892 | 25886 | 25884 | 25894 | 25888 | 33021 => 25867,
        // Brassica halo#Normal
        24206 => 24204,
        // Bronze defender#Normal
        24136 => 8844,
        // Bronze full helm
        12211 | 12221 => 1155,
        // Bronze kiteshield
        12213 | 12223 => 1189,
        // Bronze platebody
        12205 | 12215 => 1117,
        // Bronze platelegs
        12207 | 12217 => 1075,
        // Bronze plateskirt
        12209 | 12219 => 1087,
        // Bucket helm
        20059 => 19991,
        // Calamity breeches#Normal
        26756 => 26755,
        // Calamity chest#Normal
        26750 => 26749,
        // Centurion cuirass#Normal
        26722 => 26721,
        // Climbing boots
        23413 => 3105,
        // Collection log
        30593 => 22711,
        // Crystal body#Active
        27769 | 27745 | 27757 | 27697 | 27721 | 27709 | 27733 | 33023 => 23975,
        // Crystal body#Inactive
        27771 | 27747 | 27759 | 27699 | 27723 | 27711 | 27735 | 33025 => 23977,
        // Crystal helm#Active
        27777 | 27753 | 27765 | 27705 | 27729 | 27717 | 27741 | 33031 => 23971,
        // Crystal helm#Inactive
        27779 | 27755 | 27767 | 27707 | 27731 | 27719 | 27743 | 33033 => 23973,
        // Crystal legs#Active
        27773 | 27749 | 27761 | 27701 | 27725 | 27713 | 27737 | 33027 => 23979,
        // Crystal legs#Inactive
        27775 | 27751 | 27763 | 27703 | 27727 | 27715 | 27739 | 33029 => 23981,
        // Dagon'hai hat
        27123 => 24288,
        // Dagon'hai robe bottom
        27127 => 24294,
        // Dagon'hai robe top
        27125 => 24291,
        // Dark bow#Regular
        12766 | 12765 | 12768 | 12767 => 11235,
        // Decorative armour (gold platebody)#Normal
        24158 => 4509,
        // Decorative armour (gold platelegs)#Normal
        24159 => 4510,
        // Decorative armour (gold plateskirt)#Normal
        24162 => 11895,
        // Decorative armour (magic hat)#Normal
        24165 => 11898,
        // Decorative armour (magic legs)#Normal
        24164 => 11897,
        // Decorative armour (magic top)#Normal
        24163 => 11896,
        // Decorative armour (quiver)#Normal
        24168 => 11901,
        // Decorative armour (ranged legs)#Normal
        24167 => 11900,
        // Decorative armour (ranged top)#Normal
        24166 => 11899,
        // Decorative boots (gold)#Normal
        25173 => 25171,
        // Decorative full helm (gold)#Normal
        25176 => 25174,
        // Decorative helm (gold)#Normal
        24160 => 4511,
        // Decorative shield (gold)#Normal
        24161 => 4512,
        // Decorative sword (gold)#Normal
        24157 => 4508,
        // Dharok's greataxe#Undamaged
        4890 | 4886 | 4889 | 4888 | 4887 => 4718,
        // Dharok's helm#Undamaged
        4884 | 4880 | 4883 | 4882 | 4881 => 4716,
        // Dharok's platebody#Undamaged
        4896 | 4892 | 4895 | 4894 | 4893 => 4720,
        // Dharok's platelegs#Undamaged
        4902 | 4898 | 4901 | 4900 | 4899 => 4722,
        // Dinh's bulwark
        28682 => 21015,
        // Dizana's max cape#Normal
        28906 => 28902,
        // Dragon 2h sword
        28051 => 7158,
        // Dragon axe
        25378 | 30352 => 6739,
        // Dragon battleaxe
        28037 => 1377,
        // Dragon boots
        28055 | 22234 => 11840,
        // Dragon chainbody
        28065 | 12414 => 3140,
        // Dragon claws
        28039 | 26708 => 13652,
        // Dragon crossbow
        28053 => 21902,
        // Dragon dagger#Poison
        28021 => 1231,
        // Dragon dagger#Poison+
        28023 => 5680,
        // Dragon dagger#Poison++
        28025 => 5698,
        // Dragon dagger#Unpoisoned
        28019 => 1215,
        // Dragon defender#Normal
        24143 | 27008 | 19722 => 12954,
        // Dragon full helm
        12417 => 11335,
        // Dragon halberd
        28049 => 3204,
        // Dragon harpoon
        25373 | 30349 => 21028,
        // Dragon hunter crossbow
        25918 | 25916 => 21012,
        // Dragon kiteshield
        22244 => 21895,
        // Dragon longsword
        28033 => 1305,
        // Dragon mace
        28027 => 1434,
        // Dragon med helm
        28057 => 1149,
        // Dragon pickaxe
        23677 | 12797 => 11920,
        // Dragon pickaxe (or)
        25376 | 30351 => 23677,
        // Dragon platebody
        22242 => 21892,
        // Dragon platelegs
        28061 | 12415 => 4087,
        // Dragon plateskirt
        28063 | 12416 => 4585,
        // Dragon scimitar
        28031 | 20000 => 4587,
        // Dragon spear#Poison
        28043 => 1263,
        // Dragon spear#Poison+
        28045 => 5716,
        // Dragon spear#Poison++
        28047 => 5730,
        // Dragon spear#Unpoisoned
        28041 => 1249,
        // Dragon sq shield
        28059 | 12418 => 1187,
        // Dragon sword
        28029 => 21009,
        // Dragon warhammer
        28035 | 26710 => 13576,
        // Eclipse moon chestplate#New
        29031 => 29004,
        // Eclipse moon helm#New
        29035 => 29010,
        // Eclipse moon tassets#New
        29033 => 29007,
        // Elder chaos hood
        27119 => 20595,
        // Elder chaos robe
        27117 => 20520,
        // Elder chaos top
        27115 => 20517,
        // Elder maul
        27100 => 21003,
        // Elidinis' ward (f)
        27253 => 27251,
        // Elite calamity breeches#Normal
        26760 => 26759,
        // Elite calamity chest#Normal
        26754 => 26753,
        // Elite void robe#Normal
        24180 | 27004 | 26471 => 13073,
        // Elite void top#Normal
        24178 | 27003 | 26469 => 13072,
        // Fighter hat#Normal
        24173 => 10548,
        // Fighter torso#Normal
        24175 | 28069 | 28067 => 10551,
        // Fire cape#Normal
        24223 => 6570,
        // Fire max cape#Normal
        24134 => 13329,
        // Ghommal's avernic defender 5#Normal
        27551 => 27550,
        // Ghommal's avernic defender 6#Normal
        27553 => 27552,
        // Ghrazi rapier
        25734 => 22324,
        // Granite maul#Normal
        12848 => 4153,
        // Granite ring (i)#Nightmare Zone
        26685 | 25193 => 21752,
        // Green d'hide body
        7370 | 7372 => 1135,
        // Green d'hide chaps
        7378 | 7380 => 1099,
        // Grid master tabard
        31190 => 31181,
        // Guthan's chainskirt#Undamaged
        4926 | 4922 | 4925 | 4924 | 4923 => 4730,
        // Guthan's helm#Undamaged
        4908 | 4904 | 4907 | 4906 | 4905 => 4724,
        // Guthan's platebody#Undamaged
        4920 | 4916 | 4919 | 4918 | 4917 => 4728,
        // Guthan's warspear#Undamaged
        4914 | 4910 | 4913 | 4912 | 4911 => 4726,
        // Guthix halo#Normal
        24171 => 12639,
        // Healer hat#Normal
        24172 => 10547,
        // Heavy ballista
        26712 => 19481,
        // Helm of Neitiznot
        28070 => 10828,
        // Holy book
        26496 => 3840,
        // Iban's staff#Broken
        33333 => 1410,
        // Iban's staff#Regular
        33330 => 1409,
        // Iban's staff (u)
        33332 => 12658,
        // Ice ancient sceptre#Normal
        28474 => 28262,
        // Imbued Guthix cape#Normal
        24249 | 29615 => 21793,
        // Imbued Guthix max cape#Normal
        24234 => 21784,
        // Imbued Saradomin cape#Normal
        24248 | 29617 => 21791,
        // Imbued Saradomin max cape#Normal
        24232 => 21776,
        // Imbued Zamorak cape#Normal
        24250 | 29613 => 21795,
        // Imbued Zamorak max cape#Normal
        24233 => 21780,
        // Infernal cape#Normal
        24224 => 21295,
        // Infernal max cape#Normal
        24133 => 21285,
        // Infinity bottoms
        12459 | 12421 => 6924,
        // Infinity hat
        12457 | 12419 => 6918,
        // Infinity top
        12458 | 12420 => 6916,
        // Iron defender#Normal
        24137 => 8845,
        // Iron full helm
        12241 | 12231 => 1153,
        // Iron kiteshield
        12243 | 12233 => 1191,
        // Iron platebody
        12235 | 12225 => 1115,
        // Iron platelegs
        12237 | 12227 => 1067,
        // Iron plateskirt
        12239 | 12229 => 1081,
        // Karil's coif#Undamaged
        4932 | 4928 | 4931 | 4930 | 4929 => 4732,
        // Karil's crossbow#Undamaged
        4938 | 4934 | 4937 | 4936 | 4935 => 4734,
        // Karil's leatherskirt#Undamaged
        4950 | 4946 | 4949 | 4948 | 4947 => 4738,
        // Karil's leathertop#Undamaged
        4944 | 4940 | 4943 | 4942 | 4941 => 4736,
        // Koriff's coif#Normal
        26742 => 26741,
        // Koriff's cowl#Normal
        26740 => 26739,
        // Koriff's headband#Normal
        26738 => 26737,
        // Lava battlestaff
        21198 => 3053,
        // Leather body
        23381 => 1129,
        // Leather chaps
        23384 => 1095,
        // Malediction ward
        12806 => 11924,
        // Maoma's full helm#Normal
        26746 => 26745,
        // Maoma's great helm#Normal
        26748 => 26747,
        // Maoma's med helm#Normal
        26744 => 26743,
        // Masori assembler max cape#Normal
        27365 => 27363,
        // Mithril defender#Normal
        24140 => 8848,
        // Mithril full helm
        12283 | 12293 => 1159,
        // Mithril kiteshield
        12281 | 12291 => 1197,
        // Mithril platebody
        12277 | 12287 => 1121,
        // Mithril platelegs
        12279 | 12289 => 1071,
        // Mithril plateskirt
        12285 | 12295 => 1085,
        // Monk's robe
        20202 | 23306 => 542,
        // Monk's robe top
        20199 | 23303 => 544,
        // Mystic boots
        4107 | 23059 | 4117 | 26539 => 4097,
        // Mystic gloves
        4105 | 23056 | 4115 | 26537 => 4095,
        // Mystic hat
        4099 | 23047 | 4109 | 26531 => 4089,
        // Mystic lava staff
        21200 => 3054,
        // Mystic robe bottom
        4103 | 23053 | 4113 | 26535 => 4093,
        // Mystic robe top
        4101 | 23050 | 4111 | 26533 => 4091,
        // Mystic steam staff
        12796 => 11789,
        // Necklace of anguish
        22249 => 19547,
        // Oathplate chest
        30779 => 30753,
        // Oathplate helm
        30777 => 30750,
        // Oathplate legs
        30781 => 30756,
        // Obsidian cape
        20050 => 6568,
        // Occult necklace
        19720 => 12002,
        // Odium ward
        12807 => 11926,
        // Osmumten's fang
        27246 => 26219,
        // Penance skirt#Normal
        24176 => 10555,
        // Ranger hat#Normal
        24174 => 10550,
        // Red d'hide body
        12327 | 12331 => 2501,
        // Red d'hide chaps
        12329 | 12333 => 2495,
        // Ring of suffering (i)#Recoil
        25248 | 26762 => 20657,
        // Ring of suffering (i)#Uncharged
        25246 | 26761 => 19710,
        // Ring of the gods (i)#Nightmare Zone
        26764 | 25252 => 13202,
        // Rune crossbow
        26486 => 9185,
        // Rune defender#Normal
        24142 | 27009 | 23230 => 8850,
        // Rune full helm
        2619 | 2627 | 10286 | 10288 | 10290 | 10292 | 10294 => 1163,
        // Rune kiteshield
        8714 | 8716 | 8718 | 8720 | 8722 | 8724 | 8726 | 8728 | 8730 | 8732 | 8734 | 8736
        | 8738 | 8740 | 8742 | 8744 | 2621 | 2629 | 7336 | 7342 | 7348 | 7354 | 7360 => 1201,
        // Rune platebody
        2615 | 23209 | 23212 | 23215 | 23218 | 23221 | 2623 => 1127,
        // Rune platelegs
        2617 | 2625 => 1079,
        // Rune plateskirt
        3476 | 3477 => 1093,
        // Rune scimitar
        23330 | 23332 | 23334 => 1333,
        // Runner hat#Normal
        24533 => 10549,
        // Saika's hood#Normal
        26732 => 26731,
        // Saika's shroud#Normal
        26736 => 26735,
        // Saika's veil#Normal
        26734 => 26733,
        // Salve amulet(ei)#Nightmare Zone
        26782 | 25278 => 12018,
        // Salve amulet(i)#Nightmare Zone
        26763 | 25250 => 12017,
        // Sanguinesti staff#Charged
        25731 => 22323,
        // Sanguinesti staff#Uncharged
        25733 => 22481,
        // Saradomin godsword
        20372 => 11806,
        // Saradomin halo#Normal
        24169 => 12637,
        // Scythe of Vitur#Charged
        25736 | 25739 => 22325,
        // Scythe of Vitur#Uncharged
        25738 | 25741 => 22486,
        // Seers ring (i)#Nightmare Zone
        26767 | 25258 => 11770,
        // Seren halo#Normal
        24200 => 24198,
        // Shadow ancient sceptre#Normal
        28476 => 28266,
        // Slayer helmet
        29816 | 19639 | 19643 | 33066 | 23073 | 33338 | 21264 | 33340 | 19647 | 21888 | 24370
        | 25910 | 25898 | 25904 => 11864,
        // Slayer helmet (i)#Nightmare Zone
        29822 | 29818 | 29820 | 26675 | 19641 | 25179 | 26676 | 19645 | 25181 | 33072 | 33068
        | 33070 | 26680 | 23075 | 25189 | 33443 | 33439 | 33441 | 26678 | 21266 | 25185 | 33449
        | 33445 | 33447 | 26677 | 19649 | 25183 | 26674 | 25177 | 26679 | 21890 | 25187 | 26681
        | 24444 | 25191 | 26684 | 25912 | 25914 | 26682 | 25900 | 25902 | 26683 | 25906 | 25908 => {
            11865
        }
        // Smoke ancient sceptre#Normal
        28475 => 28264,
        // Soulreaper axe
        33335 => 28338,
        // Steam battlestaff
        12795 => 11787,
        // Steel defender#Normal
        24138 => 8846,
        // Steel full helm
        20178 | 20193 => 1157,
        // Steel kiteshield
        8746 | 8748 | 8750 | 8752 | 8754 | 8756 | 8758 | 8760 | 8762 | 8764 | 8766 | 8768
        | 8770 | 8772 | 8774 | 8776 | 20181 | 20196 => 1193,
        // Steel platebody
        20169 | 20184 => 1119,
        // Steel platelegs
        20172 | 20187 => 1069,
        // Steel plateskirt
        20175 | 20190 => 1083,
        // Studded body
        7362 | 7364 => 1133,
        // Studded chaps
        7366 | 7368 => 1097,
        // Superior calamity breeches#Normal
        26758 => 26757,
        // Superior calamity chest#Normal
        26752 => 26751,
        // Swords and emblem
        31202 => 31193,
        // Torag's hammers#Undamaged
        4962 | 4958 | 4961 | 4960 | 4959 => 4747,
        // Torag's helm#Undamaged
        4956 | 4952 | 4955 | 4954 | 4953 => 4745,
        // Torag's platebody#Undamaged
        4968 | 4964 | 4967 | 4966 | 4965 => 4749,
        // Torag's platelegs#Undamaged
        4974 | 4970 | 4973 | 4972 | 4971 => 4751,
        // Tormented bracelet
        23444 => 19544,
        // Torva full helm#Restored
        28254 => 26382,
        // Torva platebody#Restored
        28256 => 26384,
        // Torva platelegs#Restored
        28258 => 26386,
        // Toxic blowpipe#Charged
        28688 => 12926,
        // Toxic blowpipe#Empty
        28687 => 12924,
        // Treasonous ring (i)#Nightmare Zone
        26766 | 25256 => 12692,
        // Trident of the Seas#Partially charged
        33322 => 11907,
        // Trident of the Seas (e)#Charged
        33326 => 22288,
        // Trident of the Seas (e)#Uncharged
        33328 => 22290,
        // Trident of the Swamp#Charged
        33314 => 12899,
        // Trident of the Swamp#Uncharged
        33316 => 12900,
        // Trident of the Swamp (e)#Charged
        33318 => 22292,
        // Trident of the Swamp (e)#Uncharged
        33320 => 22294,
        // Tyrannical ring (i)#Nightmare Zone
        26765 | 25254 => 12691,
        // Tzhaar-ket-om
        23235 => 6528,
        // Unholy book
        26498 => 3842,
        // Venator bow#Charged
        30434 => 27610,
        // Venator bow#Uncharged
        30436 => 27612,
        // Verac's brassard#Undamaged
        4992 | 4988 | 4991 | 4990 | 4989 => 4757,
        // Verac's flail#Undamaged
        4986 | 4982 | 4985 | 4984 | 4983 => 4755,
        // Verac's helm#Undamaged
        4980 | 4976 | 4979 | 4978 | 4977 => 4753,
        // Verac's plateskirt#Undamaged
        4998 | 4994 | 4997 | 4996 | 4995 => 4759,
        // Virtus mask
        30437 => 26241,
        // Virtus robe bottom
        30441 => 26245,
        // Virtus robe top
        30439 => 26243,
        // Void knight gloves#Normal
        24182 | 27002 | 26467 => 8842,
        // Void knight mace#Normal
        24181 => 8841,
        // Void knight robe#Normal
        24179 | 27001 | 26465 => 8840,
        // Void knight top#Normal
        24177 | 27000 | 26463 => 8839,
        // Void mage helm#Normal
        24183 | 27005 | 26473 => 11663,
        // Void melee helm#Normal
        24185 | 27007 | 26477 => 11665,
        // Void ranger helm#Normal
        24184 | 27006 | 26475 => 11664,
        // Void seal#(8)
        11673 | 11672 | 11671 | 11670 | 11669 | 11668 | 11667 => 11666,
        // Voidwaker
        29607 => 27690,
        // Volatile Nightmare staff
        29609 => 24424,
        // Warrior ring (i)#Nightmare Zone
        26769 | 25262 => 11772,
        // Wooden shield
        20166 => 1171,
        // Wristbands of the Arena#Normal
        26724 => 26723,
        // Wristbands of the Arena (i)#Normal
        26728 => 26727,
        // Zamorak godsword
        20374 => 11808,
        // Zamorak halo#Normal
        24170 => 12638,
        _ => id,
    }
}
