//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2368;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2369;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2370;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2371;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2372;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta645<F: Float>(t10216: F, t13797: F, t3067: F, t353: F, t373: F, t383: F, t1021: F, t820: F, t10482: F, t1615: F, t10390: F, t10858: F, t10883: F, t13975: F, t14069: F, t14080: F, t14211: F, t2986: F, t3039: F, t3041: F, t3057: F, t3064: F, t3121: F, t42388: F, t42397: F, t42436: F, t42460: F, t42511: F, t43235: F, t43361: F, t4575: F, t4582: F, t4593: F, t45971: F, t48265: F, t1041: F, t13969: F, t14142: F, t14179: F, t10309: F, t10408: F, t14126: F, t14167: F, t1616: F, t2776: F, t3070: F, t3071: F, t3117: F, t42478: F, t42481: F, t42490: F, t42546: F, t43358: F, t4579: F, t4650: F, t47779: F, t47915: F, t48260: F, t48497: F, t10375: F, t1612: F, t1539: F, t248: F, t42749: F, t14473: F, t2952: F, t10633: F, t4483: F, t47705: F, t47707: F, t47730: F, t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47722: F, t47724: F, t47728: F, t47732: F, t47736: F, t47738: F, t41655: F, t41656: F, t41658: F, t41660: F, t41662: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41713: F, t47744: F, t47748: F, t47761: F, t47765: F, t47769: F, t47777: F, t47781: F, t47785: F, t47787: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t48607, t48611, t48612, t48622) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2368::<F>(t10216, t13797, t3067, t353, t373, t383, t1021, t820, t10482, t1615, t10390, t10858, t10883, t13975, t14069, t14080, t14211, t2986, t3039, t3041, t3057, t3064, t3121, t42388, t42397, t42436, t42460, t42511, t43235, t43361, t4575, t4582, t4593, t45971, t48265);
        let t48656 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2369::<F>(t1041, t13969, t14142, t14179, t10309, t10408, t14126, t14167, t1616, t2776, t3070, t3071, t3117, t42478, t42481, t42490, t42546, t43358, t4579, t4582, t4650, t47779, t47915, t48260, t48497, t48607);
        let (t48670, t48674, t48679, t48681, t48688, t48689) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2370::<F>(t10375, t1612, t1041, t1539, t248, t42749, t14473, t2952, t10633, t4483, t47705, t47707);
        let t48702 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2371::<F>(t47730, t47681, t47686, t47691, t47695, t47699, t47703, t47709, t47711, t47713, t47715, t47717, t47722, t47724, t47728, t47732, t47736, t47738, t48688, t48689);
        let t48722 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2372::<F>(t41655, t41656, t41658, t41660, t41662, t41675, t41678, t41680, t41682, t41684, t41713, t47744, t47748, t47761, t47765, t47769, t47777, t47781, t47785, t47787);
    (t48611, t48612, t48622, t48656, t48670, t48674, t48679, t48681, t48702, t48722)
}
