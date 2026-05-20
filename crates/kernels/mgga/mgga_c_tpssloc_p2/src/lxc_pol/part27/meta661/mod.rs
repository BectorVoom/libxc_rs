//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2314;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2315;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2316;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2317;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2318;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2319;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta661<F: Float>(t90963: F, t22633: F, t26421: F, t3851: F, t6976: F, t26418: F, t6914: F, t7736: F, t80854: F, t81064: F, t22704: F, t22705: F, t26410: F, t1336: F, t1352: F, t16033: F, t16055: F, t1825: F, t22879: F, t26404: F, t26442: F, t26453: F, t26458: F, t3773: F, t3777: F, t5234: F, t5344: F, t7747: F, t81199: F, t90942: F, t90946: F, t90952: F, t90957: F, t90962: F, t26432: F, t6897: F, t794: F, t22642: F, t22690: F, t26395: F, t22863: F, t7737: F, t26448: F, t90497: F, t215: F, t6916: F, t225: F, t3787: F, t562: F, t16313: F, t22751: F, t26385: F, t16068: F, t1992: F, t81149: F, t16060: F, t26403: F, t3856: F, t5250: F, t5334: F, t6988: F, t81115: F, t81125: F, t81127: F, t81140: F, t81147: F, t81154: F, t3719: F, t6637: F, t6888: F, t7722: F, t16307: F, t90915: F, t81187: F, t81197: F, t1307: F, t26331: F, t26446: F, t90818: F, t3734: F, t90591: F, t22710: F, t22874: F, t22877: F, t26456: F, t3793: F, t81160: F, t81184: F, t81189: F, t26389: F, t22897: F, t3792: F, t90870: F, t26467: F, t26426: F, t81046: F, t7732: F, t81195: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90964, t90968, t90971, t90980, t90983) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2314::<F>(t90963, t22633, t26421, t3851, t6976, t26418, t6914, t7736, t80854, t81064, t22704, t22705, t26410);
        let t90985 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2315::<F>(t90983, t1336, t1352, t16033, t16055, t1825, t22879, t26404, t26442, t26453, t26458, t3773, t3777, t3851, t5234, t5344, t7747, t81199, t90942, t90946, t90952, t90957, t90962, t90964, t90968, t90971, t90980);
        let (t90988, t90993, t91000, t91002, t91004) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2316::<F>(t26432, t6897, t794, t22642, t22690, t26395, t22863, t7737, t26448, t90497, t215, t6916);
        let (t91005, t91019) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2317::<F>(t225, t3787, t562, t16313, t91004, t22751, t26385, t16068, t1992, t6976, t81149, t16060, t26403, t3856, t5250, t5334, t5344, t6988, t81115, t81125, t81127, t81140, t81147, t81154, t90942, t90988, t90993, t91000, t91002);
        let (t91025, t91029, t91036, t91043, t91045, t91048) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2318::<F>(t26395, t3719, t6637, t6888, t3787, t7722, t16307, t90915, t91004, t81187, t81197, t1307, t26331, t26446, t90818);
        let t91059 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2319::<F>(t26421, t26446, t3734, t90591, t1336, t22710, t22874, t22877, t26403, t26456, t26458, t3777, t3793, t3851, t3856, t5234, t5250, t5334, t5344, t81160, t81184, t81189, t90946, t91025, t91029, t91036, t91043, t91045, t91048);
        let (t91065, t91074, t91077, t91078, t91081) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2320::<F>(t22751, t26389, t1992, t22897, t3792, t90870, t26467, t6914, t26426, t81046, t22690, t7732, t81195);
    (t90985, t91005, t91019, t91059, t91065, t91074, t91077, t91078, t91081)
}
