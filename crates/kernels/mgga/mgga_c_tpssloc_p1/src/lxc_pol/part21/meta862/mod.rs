//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta862 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3129;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3130;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3131;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3132;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3133;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3134;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3135;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3136;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3137;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3138;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3139;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta862<F: Float>(t16558: F, t3450: F, t11588: F, t6138: F, t3447: F, t3451: F, t4904: F, t52036: F, t15313: F, t15338: F, t18523: F, t3448: F, t6144: F, t15402: F, t18237: F, t15376: F, t15395: F, t15406: F, t3449: F, t4900: F, t4908: F, t63294: F, t63311: F, t63372: F, t63378: F, t1887: F, t337: F, t5416: F, t51968: F, t11575: F, t15409: F, t15412: F, t18427: F, t3452: F, t52096: F, t63315: F, t63368: F, t63390: F, t63402: F, t63406: F, t63410: F, t63420: F, t3446: F, t61064: F, t1176: F, t1714: F, t1184: F, t15293: F, t15382: F, t3439: F, t44424: F, t44439: F, t52074: F, t52076: F, t52081: F, t52084: F, t52086: F, t52089: F, t52092: F, t52109: F, t4928: F, t1174: F, t135: F, t18525: F, t11583: F, t17691: F, t12652: F, t4723: F, t3428: F, t6109: F, t6146: F, t698: F, t15320: F, t457: F, t460: F, t4733: F, t4919: F, t52122: F, t52124: F, t52170: F, t7319: F, t974: F, t6140: F, t63841: F, t63843: F, t63845: F, t63886: F, t63888: F, t63891: F, t63893: F, t63896: F, t63899: F, t63903: F, t63906: F, t63909: F, t50846: F, t50848: F, t50853: F, t63911: F, t63914: F, t63918: F, t63921: F, t63924: F, t63927: F, t63930: F, t63933: F, t63936: F, t63939: F, t43855: F, t43859: F, t43861: F, t43863: F, t44466: F, t50968: F, t50970: F, t50972: F, t50978: F, t64003: F, t64006: F, t64045: F, t51039: F, t51041: F, t51043: F, t51051: F, t51053: F, t64074: F, t64076: F, t64079: F, t64082: F, t64085: F, t64087: F, t64089: F, t64092: F, t18321: F, t3435: F, t15390: F, t1653: F, t24705: F, t3472: F, t3478: F, t44478: F, t52127: F, t52135: F, t52138: F, t52161: F, t52271: F, t15281: F, t18563: F, t3432: F, t11529: F, t6130: F, t15282: F, t4889: F, t18558: F, t3431: F, t14730: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t64756, t64765, t64770, t64773, t64775) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3129::<F>(t16558, t3450, t11588, t6138, t3447, t3451, t4904, t52036, t15313, t15338, t18523, t3448);
        let t64786 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3130::<F>(t11588, t6144, t3447, t3451, t15402, t18237, t15376, t15395, t15406, t3449, t4900, t4908, t63294, t63311, t63372, t63378, t64756, t64765, t64770, t64773, t64775);
        let t64823 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3131::<F>(t1887, t337, t5416, t3447, t4904, t51968, t11575, t15376, t15409, t15412, t18427, t3452, t4900, t4908, t52096, t63315, t63368, t63390, t63402, t63406, t63410, t63420);
        let t64845 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3132::<F>(t3446, t61064, t1176, t1714, t1184, t15293, t15382, t3439, t44424, t44439, t52074, t52076, t52081, t52084, t52086, t52089, t52092, t52109);
        let (t64851, t64858, t64870, t64874, t64878, t64881) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3133::<F>(t4928, t1174, t135, t18525, t11583, t17691, t12652, t4723, t3428, t6109, t6146, t698);
        let t64883 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3134::<F>(t1174, t15293, t15320, t3447, t3449, t457, t460, t4733, t4908, t4919, t52122, t52124, t52170, t64851, t64858, t64870, t64874, t64878, t64881, t7319, t974);
        let (t64885, t64903) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3135::<F>(t1174, t6140, t698, t63841, t63843, t63845, t63886, t63888, t63891, t63893, t63896, t63899, t63903, t63906, t63909);
        let t64916 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3136::<F>(t50846, t50848, t50853, t63911, t63914, t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939);
        let t64929 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3137::<F>(t43855, t43859, t43861, t43863, t44466, t50968, t50970, t50972, t50978, t64003, t64006, t64045);
        let t64943 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3138::<F>(t51039, t51041, t51043, t51051, t51053, t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092);
        let t64966 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3139::<F>(t18321, t3435, t1174, t15390, t1653, t24705, t3447, t3472, t3478, t44478, t457, t460, t4919, t52127, t52135, t52138, t52161, t52271, t64885, t64903, t64916, t64929, t64943, t974);
        let (t64969, t64976, t64979, t64981, t64988, t64990) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3140::<F>(t1174, t15281, t18563, t18321, t3432, t11529, t6130, t15282, t4889, t18558, t3431, t12652, t14730);
    (t64786, t64823, t64845, t64874, t64883, t64966, t64969, t64976, t64979, t64981, t64988, t64990)
}
