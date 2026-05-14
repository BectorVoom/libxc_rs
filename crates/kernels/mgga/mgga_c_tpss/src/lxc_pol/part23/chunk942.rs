//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 942/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk942<F: Float>(t3048: F, t8549: F, t8548: F, t3054: F, t9080: F, t1107: F, t3308: F, t8229: F, t1183: F, t123: F, t2349: F, t8220: F, t8223: F, t8232: F, t1186: F, t3305: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9763 = t8549 * t3048;
    let t9764 = t8548 * t9763;
    let t9765 = t9080 * t3054;
    let t9786 = t8549 * t1107;
    let t9787 = t8548 * t9786;
    let t9839 = 0.21687162600603479684e-1 * t3308 * t8229;
    let t9840 = t1183 * t123;
    let t9841 = t9840 * t2349;
    let t9844 = 0.16265371950452609763e-1 * t3308 * t8220;
    let t9846 = 0.48159733137676571078e0 * t3308 * t8223;
    let t9848 = 0.32530743900905219526e-1 * t3308 * t8232;
    let t9854 = 60.0 * t3305 * t1186;
    (t9764, t9765, t9787, t9839, t9841, t9844, t9846, t9848, t9854)
}
