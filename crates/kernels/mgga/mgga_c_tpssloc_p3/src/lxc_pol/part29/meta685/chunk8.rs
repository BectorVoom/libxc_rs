//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2346/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2346<F: Float>(t5: F, t95996: F, t96021: F, t96050: F, t96077: F, t96105: F, t96133: F, t96180: F, t96209: F, t112: F, t671: F, t7263: F, t12813: F, t1459: F, t1849: F, t2165: F, t2314: F, t24932: F, t24939: F, t27293: F, t3929: F, t4037: F, t510: F, t652: F, t8107: F, t91666: F, t91671: F, t91673: F, t91674: F, t91678: F, t91681: F, t91684: F, t91690: F, t91694: F, t91698: F, t91704: F, t91706: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t96213 = piecewise3::<F>(t8, F::new(0.0), t95996 + t96021 + t96050 + t96077 + t96105 + t96133 + t96180 + t96209);
    let t96214 = t96213 * t112;
    let t96222 = t7263 * t671;
    let t96228 = -F::new(2.0) * t12813 * t2165 * t652 - F::new(4.0) * t1459 * t96222 + t1849 * t24939 - F::new(4.0) * t2314 * t27293 - F::new(4.0) * t24932 * t4037 + t3929 * t8107 - t510 * t96214 + t91666 + t91671 - t91673 - t91674 + t91678 + t91681 - t91684 - t91690 - t91694 - t91698 - t91704 - t91706;
    (t96214, t96222, t96228)
}
