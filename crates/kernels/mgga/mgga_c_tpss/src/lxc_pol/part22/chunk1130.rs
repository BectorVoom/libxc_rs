//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1130/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1130<F: Float>(t140: F, t4227: F, t1098: F, t1015: F, t4246: F, t3068: F, t1562: F, t2850: F, t2846: F, t1554: F, t3025: F, t1125: F, t12483: F, t12487: F, t12492: F, t12498: F, t12503: F, t12507: F, t12512: F, t12516: F, t12520: F, t12524: F, t12530: F, t3035: F, t3040: F, t3044: F, t3052: F, t3067: F, t3099: F, t4212: F, t4265: F, t9618: F, t9626: F) -> F {
    let t12535 = t140 * t4227;
    let t12537 = t1098 * t12535 / F::new(432.0);
    let t12538 = t4246 * t1015;
    let t12539 = t3068 * t12538;
    let t12542 = t1562 * t2850;
    let t12543 = t3068 * t12542;
    let t12546 = t1562 * t2846;
    let t12547 = t3068 * t12546;
    let t12550 = t1554 * t3025;
    let t12552 = F::new(5.0) / F::new(6912.0) * t1125 * t12483 + F::new(5.0) / F::new(13824.0) * t1125 * t12487 + F::new(5.0) / F::new(2304.0) * t1125 * t12492 - F::new(5.0) / F::new(2592.0) * t4265 * t3099 + t3052 * t12498 / F::new(1536.0) + t9618 * t12503 / F::new(512.0) - t9626 * t12507 / F::new(512.0) - F::new(5.0) / F::new(5184.0) * t1125 * t12512 - t1125 * t12516 / F::new(1152.0) - t1125 * t12520 / F::new(2304.0) - t1098 * t12524 / F::new(288.0) - t4212 * t3035 / F::new(81.0) + t12530 + t4212 * t3044 / F::new(108.0) + t4212 * t3040 / F::new(54.0) - t12537 - t3067 * t12539 / F::new(2304.0) - t3067 * t12543 / F::new(4608.0) - t3067 * t12547 / F::new(2304.0) + t12550 / F::new(162.0);
    t12552
}
