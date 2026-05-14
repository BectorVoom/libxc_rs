//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 817/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk817<F: Float>(t1873: F, t7056: F, t3941: F, t2039: F, t6534: F, t23877: F, t23880: F, t31284: F, t31287: F, t31781: F, t31795: F, t31799: F, t31801: F, t31803: F, t31811: F, t31813: F, t577: F, t671: F, t7010: F, t7235: F, t8508: F) -> (F, F, F) {
    let t31814 = t7056 * t1873;
    let t31816 = 27.0 * t3941 * t31814;
    let t31817 = t2039 * t6534;
    let t31819 = 27.0 * t3941 * t31817;
    let t31820 = 0.45e1 * t31781 * t577 + 0.135e2 * t31795 * t671 + t31799 + t31801 + t31803 + 0.135e2 * t23877 * t2039 + 27.0 * t23880 * t7235 + 0.135e2 * t7010 * t7056 + t31811 + t31813 + t31816 + t31819 + t31284 + t31287 + t8508;
    (t31814, t31817, t31820)
}
