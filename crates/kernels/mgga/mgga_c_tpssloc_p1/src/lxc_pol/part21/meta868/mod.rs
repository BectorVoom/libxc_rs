//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta868 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3176;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3177;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3178;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3179;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3180;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta868<F: Float>(t11738: F, t15560: F, t15564: F, t15612: F, t15617: F, t18300: F, t19077: F, t3252: F, t3494: F, t3509: F, t3516: F, t3577: F, t3578: F, t44836: F, t44965: F, t45037: F, t4582: F, t4980: F, t4984: F, t5005: F, t5024: F, t52621: F, t52628: F, t52649: F, t52653: F, t52664: F, t52903: F, t53372: F, t53399: F, t6219: F, t15643: F, t19201: F, t3576: F, t44951: F, t6191: F, t11668: F, t15569: F, t15663: F, t15704: F, t15708: F, t15750: F, t18210: F, t18231: F, t19056: F, t3515: F, t3580: F, t44847: F, t52666: F, t52674: F, t52680: F, t52682: F, t52684: F, t52766: F, t52879: F, t13969: F, t19061: F, t11665: F, t11678: F, t11692: F, t1227: F, t14731: F, t14736: F, t14740: F, t15654: F, t1735: F, t19016: F, t19068: F, t3490: F, t4724: F, t4987: F, t5012: F, t52725: F, t52731: F, t52733: F, t55662: F, t55666: F, t5979: F, t62044: F, t15568: F, t5064: F, t1174: F, t18206: F, t44562: F, t18958: F, t15591: F, t15714: F, t18342: F, t18387: F, t44621: F, t4950: F, t5014: F, t52751: F, t52758: F, t52773: F, t53322: F, t5971: F, t5975: F, t63420: F, t248: F, t45046: F, t15438: F, t15453: F, t15527: F, t15555: F, t15637: F, t15737: F, t19080: F, t3496: F, t44886: F, t44890: F, t44894: F, t5002: F, t52776: F, t52781: F, t52792: F, t52795: F, t52801: F, t1009: F, t18571: F, t1011: F, t1212: F, t3032: F, t65253: F, t3505: F, t3514: F, t1218: F, t15455: F, t15541: F, t15545: F, t15656: F, t18590: F, t18594: F, t18955: F, t19047: F, t3511: F, t3518: F, t4972: F, t52817: F, t52845: F, t52859: F, t61798: F) -> (F, F, F, F, F, F, F) {
        let t65802 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3176::<F>(t11738, t15560, t15564, t15612, t15617, t18300, t19077, t3252, t3494, t3509, t3516, t3577, t3578, t44836, t44965, t45037, t4582, t4980, t4984, t5005, t5024, t52621, t52628, t52649, t52653, t52664, t52903, t53372, t53399, t6219);
        let t65835 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3177::<F>(t15643, t5024, t19201, t3576, t3577, t44951, t6191, t11668, t15569, t15663, t15704, t15708, t15750, t18210, t18231, t19056, t3494, t3515, t3580, t44847, t4582, t52666, t52674, t52680, t52682, t52684, t52766, t52879);
        let t65883 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3178::<F>(t13969, t19061, t3515, t11665, t11668, t11678, t11692, t1227, t14731, t14736, t14740, t15654, t1735, t19016, t19068, t3490, t3509, t3516, t3577, t3578, t4582, t4724, t4987, t5012, t52725, t52731, t52733, t55662, t55666, t5979, t62044);
        let t65925 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3179::<F>(t15568, t5064, t1174, t18206, t44562, t1227, t13969, t18958, t11665, t11668, t11678, t11692, t15569, t15591, t15714, t18342, t18387, t3490, t3494, t3509, t3516, t3577, t3578, t3580, t44621, t4950, t5014, t52751, t52758, t52773, t53322, t5971, t5975, t63420);
        let t65954 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3180::<F>(t1227, t248, t45046, t5971, t15643, t5005, t15438, t15453, t15527, t15555, t15637, t15737, t19080, t3496, t44886, t44890, t44894, t4582, t5002, t52776, t52781, t52792, t52795, t52801, t62044);
        let (t65955, t65990) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3181::<F>(t1009, t18571, t1011, t1212, t3032, t65253, t3505, t3514, t1218, t1227, t15455, t15541, t15545, t15656, t18590, t18594, t18955, t19047, t3490, t3496, t3511, t3518, t4582, t4972, t5005, t52817, t52845, t52859, t61798);
    (t65802, t65835, t65883, t65925, t65954, t65955, t65990)
}
