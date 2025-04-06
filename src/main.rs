// Rust macro that generates 1000 structs with compute() logic
macro_rules! generate_structs {
    ($($name:ident => $value:expr),*) => {
        $(
            pub struct $name;

            impl $name {
                pub fn new() -> Self {
                    $name
                }

                pub fn compute(&self) -> u64 {
                    $value * $value
                }
            }
        )*
    };
}

// Macro expansion with 1000 structs
generate_structs!(
    Struct1 => 1, Struct2 => 2, Struct3 => 3, Struct4 => 4, Struct5 => 5, Struct6 => 6, Struct7 => 7, Struct8 => 8, Struct9 => 9, Struct10 => 10,
    Struct11 => 11, Struct12 => 12, Struct13 => 13, Struct14 => 14, Struct15 => 15, Struct16 => 16, Struct17 => 17, Struct18 => 18, Struct19 => 19, Struct20 => 20,
    Struct21 => 21, Struct22 => 22, Struct23 => 23, Struct24 => 24, Struct25 => 25, Struct26 => 26, Struct27 => 27, Struct28 => 28, Struct29 => 29, Struct30 => 30,
    Struct31 => 31, Struct32 => 32, Struct33 => 33, Struct34 => 34, Struct35 => 35, Struct36 => 36, Struct37 => 37, Struct38 => 38, Struct39 => 39, Struct40 => 40,
    Struct41 => 41, Struct42 => 42, Struct43 => 43, Struct44 => 44, Struct45 => 45, Struct46 => 46, Struct47 => 47, Struct48 => 48, Struct49 => 49, Struct50 => 50,
    Struct51 => 51, Struct52 => 52, Struct53 => 53, Struct54 => 54, Struct55 => 55, Struct56 => 56, Struct57 => 57, Struct58 => 58, Struct59 => 59, Struct60 => 60,
    Struct61 => 61, Struct62 => 62, Struct63 => 63, Struct64 => 64, Struct65 => 65, Struct66 => 66, Struct67 => 67, Struct68 => 68, Struct69 => 69, Struct70 => 70,
    Struct71 => 71, Struct72 => 72, Struct73 => 73, Struct74 => 74, Struct75 => 75, Struct76 => 76, Struct77 => 77, Struct78 => 78, Struct79 => 79, Struct80 => 80,
    Struct81 => 81, Struct82 => 82, Struct83 => 83, Struct84 => 84, Struct85 => 85, Struct86 => 86, Struct87 => 87, Struct88 => 88, Struct89 => 89, Struct90 => 90,
    Struct91 => 91, Struct92 => 92, Struct93 => 93, Struct94 => 94, Struct95 => 95, Struct96 => 96, Struct97 => 97, Struct98 => 98, Struct99 => 99, Struct100 => 100,
    Struct101 => 101, Struct102 => 102, Struct103 => 103, Struct104 => 104, Struct105 => 105, Struct106 => 106, Struct107 => 107, Struct108 => 108, Struct109 => 109, Struct110 => 110,
    Struct111 => 111, Struct112 => 112, Struct113 => 113, Struct114 => 114, Struct115 => 115, Struct116 => 116, Struct117 => 117, Struct118 => 118, Struct119 => 119, Struct120 => 120,
    Struct121 => 121, Struct122 => 122, Struct123 => 123, Struct124 => 124, Struct125 => 125, Struct126 => 126, Struct127 => 127, Struct128 => 128, Struct129 => 129, Struct130 => 130,
    Struct131 => 131, Struct132 => 132, Struct133 => 133, Struct134 => 134, Struct135 => 135, Struct136 => 136, Struct137 => 137, Struct138 => 138, Struct139 => 139, Struct140 => 140,
    Struct141 => 141, Struct142 => 142, Struct143 => 143, Struct144 => 144, Struct145 => 145, Struct146 => 146, Struct147 => 147, Struct148 => 148, Struct149 => 149, Struct150 => 150,
    Struct151 => 151, Struct152 => 152, Struct153 => 153, Struct154 => 154, Struct155 => 155, Struct156 => 156, Struct157 => 157, Struct158 => 158, Struct159 => 159, Struct160 => 160,
    Struct161 => 161, Struct162 => 162, Struct163 => 163, Struct164 => 164, Struct165 => 165, Struct166 => 166, Struct167 => 167, Struct168 => 168, Struct169 => 169, Struct170 => 170,
    Struct171 => 171, Struct172 => 172, Struct173 => 173, Struct174 => 174, Struct175 => 175, Struct176 => 176, Struct177 => 177, Struct178 => 178, Struct179 => 179, Struct180 => 180,
    Struct181 => 181, Struct182 => 182, Struct183 => 183, Struct184 => 184, Struct185 => 185, Struct186 => 186, Struct187 => 187, Struct188 => 188, Struct189 => 189, Struct190 => 190,
    Struct191 => 191, Struct192 => 192, Struct193 => 193, Struct194 => 194, Struct195 => 195, Struct196 => 196, Struct197 => 197, Struct198 => 198, Struct199 => 199, Struct200 => 200,
    Struct201 => 201, Struct202 => 202, Struct203 => 203, Struct204 => 204, Struct205 => 205, Struct206 => 206, Struct207 => 207, Struct208 => 208, Struct209 => 209, Struct210 => 210,
    Struct211 => 211, Struct212 => 212, Struct213 => 213, Struct214 => 214, Struct215 => 215, Struct216 => 216, Struct217 => 217, Struct218 => 218, Struct219 => 219, Struct220 => 220,
    Struct221 => 221, Struct222 => 222, Struct223 => 223, Struct224 => 224, Struct225 => 225, Struct226 => 226, Struct227 => 227, Struct228 => 228, Struct229 => 229, Struct230 => 230,
    Struct231 => 231, Struct232 => 232, Struct233 => 233, Struct234 => 234, Struct235 => 235, Struct236 => 236, Struct237 => 237, Struct238 => 238, Struct239 => 239, Struct240 => 240,
    Struct241 => 241, Struct242 => 242, Struct243 => 243, Struct244 => 244, Struct245 => 245, Struct246 => 246, Struct247 => 247, Struct248 => 248, Struct249 => 249, Struct250 => 250,
    Struct251 => 251, Struct252 => 252, Struct253 => 253, Struct254 => 254, Struct255 => 255, Struct256 => 256, Struct257 => 257, Struct258 => 258, Struct259 => 259, Struct260 => 260,
    Struct261 => 261, Struct262 => 262, Struct263 => 263, Struct264 => 264, Struct265 => 265, Struct266 => 266, Struct267 => 267, Struct268 => 268, Struct269 => 269, Struct270 => 270,
    Struct271 => 271, Struct272 => 272, Struct273 => 273, Struct274 => 274, Struct275 => 275, Struct276 => 276, Struct277 => 277, Struct278 => 278, Struct279 => 279, Struct280 => 280,
    Struct281 => 281, Struct282 => 282, Struct283 => 283, Struct284 => 284, Struct285 => 285, Struct286 => 286, Struct287 => 287, Struct288 => 288, Struct289 => 289, Struct290 => 290,
    Struct291 => 291, Struct292 => 292, Struct293 => 293, Struct294 => 294, Struct295 => 295, Struct296 => 296, Struct297 => 297, Struct298 => 298, Struct299 => 299, Struct300 => 300,
    Struct301 => 301, Struct302 => 302, Struct303 => 303, Struct304 => 304, Struct305 => 305, Struct306 => 306, Struct307 => 307, Struct308 => 308, Struct309 => 309, Struct310 => 310,
    Struct311 => 311, Struct312 => 312, Struct313 => 313, Struct314 => 314, Struct315 => 315, Struct316 => 316, Struct317 => 317, Struct318 => 318, Struct319 => 319, Struct320 => 320,
    Struct321 => 321, Struct322 => 322, Struct323 => 323, Struct324 => 324, Struct325 => 325, Struct326 => 326, Struct327 => 327, Struct328 => 328, Struct329 => 329, Struct330 => 330,
    Struct331 => 331, Struct332 => 332, Struct333 => 333, Struct334 => 334, Struct335 => 335, Struct336 => 336, Struct337 => 337, Struct338 => 338, Struct339 => 339, Struct340 => 340,
    Struct341 => 341, Struct342 => 342, Struct343 => 343, Struct344 => 344, Struct345 => 345, Struct346 => 346, Struct347 => 347, Struct348 => 348, Struct349 => 349, Struct350 => 350,
    Struct351 => 351, Struct352 => 352, Struct353 => 353, Struct354 => 354, Struct355 => 355, Struct356 => 356, Struct357 => 357, Struct358 => 358, Struct359 => 359, Struct360 => 360,
    Struct361 => 361, Struct362 => 362, Struct363 => 363, Struct364 => 364, Struct365 => 365, Struct366 => 366, Struct367 => 367, Struct368 => 368, Struct369 => 369, Struct370 => 370,
    Struct371 => 371, Struct372 => 372, Struct373 => 373, Struct374 => 374, Struct375 => 375, Struct376 => 376, Struct377 => 377, Struct378 => 378, Struct379 => 379, Struct380 => 380,
    Struct381 => 381, Struct382 => 382, Struct383 => 383, Struct384 => 384, Struct385 => 385, Struct386 => 386, Struct387 => 387, Struct388 => 388, Struct389 => 389, Struct390 => 390,
    Struct391 => 391, Struct392 => 392, Struct393 => 393, Struct394 => 394, Struct395 => 395, Struct396 => 396, Struct397 => 397, Struct398 => 398, Struct399 => 399, Struct400 => 400,
    Struct401 => 401, Struct402 => 402, Struct403 => 403, Struct404 => 404, Struct405 => 405, Struct406 => 406, Struct407 => 407, Struct408 => 408, Struct409 => 409, Struct410 => 410,
    Struct411 => 411, Struct412 => 412, Struct413 => 413, Struct414 => 414, Struct415 => 415, Struct416 => 416, Struct417 => 417, Struct418 => 418, Struct419 => 419, Struct420 => 420,
    Struct421 => 421, Struct422 => 422, Struct423 => 423, Struct424 => 424, Struct425 => 425, Struct426 => 426, Struct427 => 427, Struct428 => 428, Struct429 => 429, Struct430 => 430,
    Struct431 => 431, Struct432 => 432, Struct433 => 433, Struct434 => 434, Struct435 => 435, Struct436 => 436, Struct437 => 437, Struct438 => 438, Struct439 => 439, Struct440 => 440,
    Struct441 => 441, Struct442 => 442, Struct443 => 443, Struct444 => 444, Struct445 => 445, Struct446 => 446, Struct447 => 447, Struct448 => 448, Struct449 => 449, Struct450 => 450,
    Struct451 => 451, Struct452 => 452, Struct453 => 453, Struct454 => 454, Struct455 => 455, Struct456 => 456, Struct457 => 457, Struct458 => 458, Struct459 => 459, Struct460 => 460,
    Struct461 => 461, Struct462 => 462, Struct463 => 463, Struct464 => 464, Struct465 => 465, Struct466 => 466, Struct467 => 467, Struct468 => 468, Struct469 => 469, Struct470 => 470,
    Struct471 => 471, Struct472 => 472, Struct473 => 473, Struct474 => 474, Struct475 => 475, Struct476 => 476, Struct477 => 477, Struct478 => 478, Struct479 => 479, Struct480 => 480,
    Struct481 => 481, Struct482 => 482, Struct483 => 483, Struct484 => 484, Struct485 => 485, Struct486 => 486, Struct487 => 487, Struct488 => 488, Struct489 => 489, Struct490 => 490,
    Struct491 => 491, Struct492 => 492, Struct493 => 493, Struct494 => 494, Struct495 => 495, Struct496 => 496, Struct497 => 497, Struct498 => 498, Struct499 => 499, Struct500 => 500,
    Struct501 => 501, Struct502 => 502, Struct503 => 503, Struct504 => 504, Struct505 => 505, Struct506 => 506, Struct507 => 507, Struct508 => 508, Struct509 => 509, Struct510 => 510,
    Struct511 => 511, Struct512 => 512, Struct513 => 513, Struct514 => 514, Struct515 => 515, Struct516 => 516, Struct517 => 517, Struct518 => 518, Struct519 => 519, Struct520 => 520,
    Struct521 => 521, Struct522 => 522, Struct523 => 523, Struct524 => 524, Struct525 => 525, Struct526 => 526, Struct527 => 527, Struct528 => 528, Struct529 => 529, Struct530 => 530,
    Struct531 => 531, Struct532 => 532, Struct533 => 533, Struct534 => 534, Struct535 => 535, Struct536 => 536, Struct537 => 537, Struct538 => 538, Struct539 => 539, Struct540 => 540,
    Struct541 => 541, Struct542 => 542, Struct543 => 543, Struct544 => 544, Struct545 => 545, Struct546 => 546, Struct547 => 547, Struct548 => 548, Struct549 => 549, Struct550 => 550,
    Struct551 => 551, Struct552 => 552, Struct553 => 553, Struct554 => 554, Struct555 => 555, Struct556 => 556, Struct557 => 557, Struct558 => 558, Struct559 => 559, Struct560 => 560,
    Struct561 => 561, Struct562 => 562, Struct563 => 563, Struct564 => 564, Struct565 => 565, Struct566 => 566, Struct567 => 567, Struct568 => 568, Struct569 => 569, Struct570 => 570,
    Struct571 => 571, Struct572 => 572, Struct573 => 573, Struct574 => 574, Struct575 => 575, Struct576 => 576, Struct577 => 577, Struct578 => 578, Struct579 => 579, Struct580 => 580,
    Struct581 => 581, Struct582 => 582, Struct583 => 583, Struct584 => 584, Struct585 => 585, Struct586 => 586, Struct587 => 587, Struct588 => 588, Struct589 => 589, Struct590 => 590,
    Struct591 => 591, Struct592 => 592, Struct593 => 593, Struct594 => 594, Struct595 => 595, Struct596 => 596, Struct597 => 597, Struct598 => 598, Struct599 => 599, Struct600 => 600,
    Struct601 => 601, Struct602 => 602, Struct603 => 603, Struct604 => 604, Struct605 => 605, Struct606 => 606, Struct607 => 607, Struct608 => 608, Struct609 => 609, Struct610 => 610,
    Struct611 => 611, Struct612 => 612, Struct613 => 613, Struct614 => 614, Struct615 => 615, Struct616 => 616, Struct617 => 617, Struct618 => 618, Struct619 => 619, Struct620 => 620,
    Struct621 => 621, Struct622 => 622, Struct623 => 623, Struct624 => 624, Struct625 => 625, Struct626 => 626, Struct627 => 627, Struct628 => 628, Struct629 => 629, Struct630 => 630,
    Struct631 => 631, Struct632 => 632, Struct633 => 633, Struct634 => 634, Struct635 => 635, Struct636 => 636, Struct637 => 637, Struct638 => 638, Struct639 => 639, Struct640 => 640,
    Struct641 => 641, Struct642 => 642, Struct643 => 643, Struct644 => 644, Struct645 => 645, Struct646 => 646, Struct647 => 647, Struct648 => 648, Struct649 => 649, Struct650 => 650,
    Struct651 => 651, Struct652 => 652, Struct653 => 653, Struct654 => 654, Struct655 => 655, Struct656 => 656, Struct657 => 657, Struct658 => 658, Struct659 => 659, Struct660 => 660,
    Struct661 => 661, Struct662 => 662, Struct663 => 663, Struct664 => 664, Struct665 => 665, Struct666 => 666, Struct667 => 667, Struct668 => 668, Struct669 => 669, Struct670 => 670,
    Struct671 => 671, Struct672 => 672, Struct673 => 673, Struct674 => 674, Struct675 => 675, Struct676 => 676, Struct677 => 677, Struct678 => 678, Struct679 => 679, Struct680 => 680,
    Struct681 => 681, Struct682 => 682, Struct683 => 683, Struct684 => 684, Struct685 => 685, Struct686 => 686, Struct687 => 687, Struct688 => 688, Struct689 => 689, Struct690 => 690,
    Struct691 => 691, Struct692 => 692, Struct693 => 693, Struct694 => 694, Struct695 => 695, Struct696 => 696, Struct697 => 697, Struct698 => 698, Struct699 => 699, Struct700 => 700,
    Struct701 => 701, Struct702 => 702, Struct703 => 703, Struct704 => 704, Struct705 => 705, Struct706 => 706, Struct707 => 707, Struct708 => 708, Struct709 => 709, Struct710 => 710,
    Struct711 => 711, Struct712 => 712, Struct713 => 713, Struct714 => 714, Struct715 => 715, Struct716 => 716, Struct717 => 717, Struct718 => 718, Struct719 => 719, Struct720 => 720,
    Struct721 => 721, Struct722 => 722, Struct723 => 723, Struct724 => 724, Struct725 => 725, Struct726 => 726, Struct727 => 727, Struct728 => 728, Struct729 => 729, Struct730 => 730,
    Struct731 => 731, Struct732 => 732, Struct733 => 733, Struct734 => 734, Struct735 => 735, Struct736 => 736, Struct737 => 737, Struct738 => 738, Struct739 => 739, Struct740 => 740,
    Struct741 => 741, Struct742 => 742, Struct743 => 743, Struct744 => 744, Struct745 => 745, Struct746 => 746, Struct747 => 747, Struct748 => 748, Struct749 => 749, Struct750 => 750,
    Struct751 => 751, Struct752 => 752, Struct753 => 753, Struct754 => 754, Struct755 => 755, Struct756 => 756, Struct757 => 757, Struct758 => 758, Struct759 => 759, Struct760 => 760,
    Struct761 => 761, Struct762 => 762, Struct763 => 763, Struct764 => 764, Struct765 => 765, Struct766 => 766, Struct767 => 767, Struct768 => 768, Struct769 => 769, Struct770 => 770,
    Struct771 => 771, Struct772 => 772, Struct773 => 773, Struct774 => 774, Struct775 => 775, Struct776 => 776, Struct777 => 777, Struct778 => 778, Struct779 => 779, Struct780 => 780,
    Struct781 => 781, Struct782 => 782, Struct783 => 783, Struct784 => 784, Struct785 => 785, Struct786 => 786, Struct787 => 787, Struct788 => 788, Struct789 => 789, Struct790 => 790,
    Struct791 => 791, Struct792 => 792, Struct793 => 793, Struct794 => 794, Struct795 => 795, Struct796 => 796, Struct797 => 797, Struct798 => 798, Struct799 => 799, Struct800 => 800,
    Struct801 => 801, Struct802 => 802, Struct803 => 803, Struct804 => 804, Struct805 => 805, Struct806 => 806, Struct807 => 807, Struct808 => 808, Struct809 => 809, Struct810 => 810,
    Struct811 => 811, Struct812 => 812, Struct813 => 813, Struct814 => 814, Struct815 => 815, Struct816 => 816, Struct817 => 817, Struct818 => 818, Struct819 => 819, Struct820 => 820,
    Struct821 => 821, Struct822 => 822, Struct823 => 823, Struct824 => 824, Struct825 => 825, Struct826 => 826, Struct827 => 827, Struct828 => 828, Struct829 => 829, Struct830 => 830,
    Struct831 => 831, Struct832 => 832, Struct833 => 833, Struct834 => 834, Struct835 => 835, Struct836 => 836, Struct837 => 837, Struct838 => 838, Struct839 => 839, Struct840 => 840,
    Struct841 => 841, Struct842 => 842, Struct843 => 843, Struct844 => 844, Struct845 => 845, Struct846 => 846, Struct847 => 847, Struct848 => 848, Struct849 => 849, Struct850 => 850,
    Struct851 => 851, Struct852 => 852, Struct853 => 853, Struct854 => 854, Struct855 => 855, Struct856 => 856, Struct857 => 857, Struct858 => 858, Struct859 => 859, Struct860 => 860,
    Struct861 => 861, Struct862 => 862, Struct863 => 863, Struct864 => 864, Struct865 => 865, Struct866 => 866, Struct867 => 867, Struct868 => 868, Struct869 => 869, Struct870 => 870,
    Struct871 => 871, Struct872 => 872, Struct873 => 873, Struct874 => 874, Struct875 => 875, Struct876 => 876, Struct877 => 877, Struct878 => 878, Struct879 => 879, Struct880 => 880,
    Struct881 => 881, Struct882 => 882, Struct883 => 883, Struct884 => 884, Struct885 => 885, Struct886 => 886, Struct887 => 887, Struct888 => 888, Struct889 => 889, Struct890 => 890,
    Struct891 => 891, Struct892 => 892, Struct893 => 893, Struct894 => 894, Struct895 => 895, Struct896 => 896, Struct897 => 897, Struct898 => 898, Struct899 => 899, Struct900 => 900,
    Struct901 => 901, Struct902 => 902, Struct903 => 903, Struct904 => 904, Struct905 => 905, Struct906 => 906, Struct907 => 907, Struct908 => 908, Struct909 => 909, Struct910 => 910,
    Struct911 => 911, Struct912 => 912, Struct913 => 913, Struct914 => 914, Struct915 => 915, Struct916 => 916, Struct917 => 917, Struct918 => 918, Struct919 => 919, Struct920 => 920,
    Struct921 => 921, Struct922 => 922, Struct923 => 923, Struct924 => 924, Struct925 => 925, Struct926 => 926, Struct927 => 927, Struct928 => 928, Struct929 => 929, Struct930 => 930,
    Struct931 => 931, Struct932 => 932, Struct933 => 933, Struct934 => 934, Struct935 => 935, Struct936 => 936, Struct937 => 937, Struct938 => 938, Struct939 => 939, Struct940 => 940,
    Struct941 => 941, Struct942 => 942, Struct943 => 943, Struct944 => 944, Struct945 => 945, Struct946 => 946, Struct947 => 947, Struct948 => 948, Struct949 => 949, Struct950 => 950,
    Struct951 => 951, Struct952 => 952, Struct953 => 953, Struct954 => 954, Struct955 => 955, Struct956 => 956, Struct957 => 957, Struct958 => 958, Struct959 => 959, Struct960 => 960,
    Struct961 => 961, Struct962 => 962, Struct963 => 963, Struct964 => 964, Struct965 => 965, Struct966 => 966, Struct967 => 967, Struct968 => 968, Struct969 => 969, Struct970 => 970,
    Struct971 => 971, Struct972 => 972, Struct973 => 973, Struct974 => 974, Struct975 => 975, Struct976 => 976, Struct977 => 977, Struct978 => 978, Struct979 => 979, Struct980 => 980,
    Struct981 => 981, Struct982 => 982, Struct983 => 983, Struct984 => 984, Struct985 => 985, Struct986 => 986, Struct987 => 987, Struct988 => 988, Struct989 => 989, Struct990 => 990,
    Struct991 => 991, Struct992 => 992, Struct993 => 993, Struct994 => 994, Struct995 => 995, Struct996 => 996, Struct997 => 997, Struct998 => 998, Struct999 => 999, Struct1000 => 1000
);

fn main() {
    let s = Struct1000::new();
    println!("Struct1000::compute() = {}", s.compute());
}
