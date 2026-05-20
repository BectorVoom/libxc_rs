//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta682 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2304;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2305;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2306;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2307;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2308;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2309;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2310;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2311;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2312;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta682<F: Float>(t24574: F, t27462: F, t1185: F, t86036: F, t974: F, t3030: F, t460: F, t27488: F, t27491: F, t27495: F, t27497: F, t1170: F, t2121: F, t27732: F, t11881: F, t15000: F, t1653: F, t1716: F, t24778: F, t24795: F, t24829: F, t27406: F, t27531: F, t3243: F, t4964: F, t7283: F, t7362: F, t7373: F, t7376: F, t7389: F, t8073: F, t8082: F, t85814: F, t85947: F, t86076: F, t15590: F, t7338: F, t27614: F, t3572: F, t27617: F, t3523: F, t1218: F, t15531: F, t15535: F, t15622: F, t15627: F, t15631: F, t15637: F, t24729: F, t24733: F, t4984: F, t86120: F, t86146: F, t86164: F, t86167: F, t86171: F, t15437: F, t24728: F, t24732: F, t4965: F, t7344: F, t1232: F, t1737: F, t27604: F, t3496: F, t3511: F, t3518: F, t3527: F, t3531: F, t86122: F, t86124: F, t86126: F, t86136: F, t24658: F, t27683: F, t1184: F, t24682: F, t27607: F, t1209: F, t85821: F, t1215: F, t15555: F, t15612: F, t15650: F, t15704: F, t24655: F, t24664: F, t24670: F, t24716: F, t24736: F, t27684: F, t478: F, t4974: F, t4980: F, t5014: F, t7345: F, t86140: F, t86327: F, t15743: F, t24649: F, t27710: F, t23508: F, t8026: F, t7325: F, t27628: F, t7324: F, t7331: F, t15730: F, t7339: F, t24661: F, t15617: F, t27711: F, t86174: F, t86176: F, t86184: F, t86234: F, t24668: F, t15643: F, t27639: F, t86264: F, t27645: F, t3540: F, t8043: F, t15545: F, t15667: F, t24699: F, t24749: F, t27655: F, t7310: F, t7316: F, t8028: F, t8035: F, t86191: F, t2136: F, t607: F, t8027: F, t1714: F, t27634: F, t10469: F, t24719: F, t3: F, t86154: F, t2132: F, t24746: F, t24685: F, t27629: F, t27636: F, t27638: F, t27642: F, t27692: F, t3032: F, t3503: F, t3507: F, t3566: F, t475: F, t488: F, t4954: F, t5011: F, t8040: F, t8048: F, t86199: F, t86330: F) -> (F, F, F, F, F, F, F, F) {
        let (t95192, t95194, t95197, t95201, t95213) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2304::<F>(t24574, t27462, t1185, t86036, t974, t3030, t460, t27488, t27491, t27495, t27497, t1170, t2121, t27732);
        let t95224 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2305::<F>(t11881, t15000, t1653, t1716, t24778, t24795, t24829, t27406, t27531, t3243, t4964, t7283, t7362, t7373, t7376, t7389, t8073, t8082, t85814, t85947, t86076, t95192, t95194, t95197, t95201, t95213);
        let t95260 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2306::<F>(t15590, t7338, t27614, t3572, t27617, t3523, t1218, t15531, t15535, t15622, t15627, t15631, t15637, t24729, t24733, t4984, t86120, t86146, t86164, t86167, t86171);
        let t95285 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2307::<F>(t15437, t24728, t24732, t4965, t7344, t1232, t1737, t27604, t27614, t27617, t3496, t3511, t3518, t3527, t3531, t86122, t86124, t86126, t86136);
        let t95316 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2308::<F>(t24658, t27683, t1184, t24682, t27607, t1209, t85821, t1215, t15555, t15612, t15650, t15704, t24655, t24664, t24670, t24716, t24729, t24736, t27684, t478, t4974, t4980, t5014, t7345, t7376, t86140, t86327);
        let (t95320, t95323, t95327, t95334, t95335) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2309::<F>(t15743, t7345, t24649, t27710, t23508, t8026, t7325, t27628, t7324, t7331, t15730, t7339);
        let t95343 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2310::<F>(t24661, t27491, t15617, t24655, t24664, t24670, t27711, t7331, t7345, t86174, t86176, t86184, t86234, t95320, t95323, t95327, t95334, t95335);
        let t95367 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2311::<F>(t24668, t27497, t15643, t7345, t27639, t86264, t27645, t3540, t8043, t15545, t15667, t24699, t24749, t27655, t7310, t7316, t8028, t8035, t86191, t86234);
        let (t95370, t95382, t95384, t95387, t95396) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2312::<F>(t2136, t607, t8027, t1714, t24682, t460, t27628, t27634, t10469, t24719, t3, t86154);
        let t95407 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2313::<F>(t2132, t24746, t95382, t24655, t24664, t24670, t24685, t27629, t27636, t27638, t27642, t27692, t3032, t3503, t3507, t3566, t475, t488, t4954, t5011, t7331, t8040, t8048, t86199, t86330, t95370, t95384, t95387, t95396);
    (t95224, t95260, t95285, t95316, t95343, t95367, t95396, t95407)
}
